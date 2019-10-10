module SvgSlider exposing (Model, Msg(..), Spec, UpdateMessage, UpdateType(..), buildEvtHandlerList, encodeUpdateMessage, encodeUpdateType, getLocation, getX, getY, init, jsSpec, jsUpdateMessage, jsUpdateType, onMouseDown, onMouseLeave, onMouseMove, onMouseUp, onTouchCancel, onTouchEnd, onTouchLeave, onTouchMove, onTouchStart, resize, sliderEvt, update, updsend, view)

-- import NoDragEvents exposing (onClick, onMouseUp, onMouseMove, onMouseDown, onMouseOut)

import Json.Decode as JD
import Json.Encode as JE
import List
import Svg exposing (Attribute, Svg, g, rect, svg, text)
import Svg.Attributes exposing (..)
import Svg.Events exposing (onClick, onMouseDown, onMouseOut, onMouseUp)
import SvgCommand exposing (Command(..))
import SvgTextSize exposing (calcTextSvg, calcTextSvgM, resizeCommand)
import SvgThings exposing (Orientation(..), UiColor(..), UiTheme)
import SvgTouch as ST
import Toop
import VirtualDom as VD


type alias Spec =
    { name : String
    , label : Maybe String
    , orientation : SvgThings.Orientation
    }


jsSpec : JD.Decoder Spec
jsSpec =
    JD.map3 Spec
        (JD.field "name" JD.string)
        (JD.maybe (JD.field "label" JD.string))
        (JD.field "orientation" JD.string |> JD.andThen SvgThings.jsOrientation)


type alias Model =
    { name : String
    , label : String
    , stringWidth : Maybe Float
    , cid : SvgThings.ControlId
    , rect : SvgThings.Rect
    , srect : SvgThings.SRect
    , orientation : SvgThings.Orientation
    , pressed : Bool
    , location : Float
    , textSvg : List (Svg ())
    , touchonly : Bool
    }


type Msg
    = SvgPress JE.Value
    | SvgUnpress JE.Value
    | NoOp
    | Reply String
    | SvgMoved JE.Value
    | SvgTouch ST.Msg
    | SvgUpdate UpdateMessage


type UpdateType
    = Press
    | Unpress


type alias UpdateMessage =
    { controlId : SvgThings.ControlId
    , updateType : Maybe UpdateType
    , location : Maybe Float
    , label : Maybe String
    }


init :
    SvgThings.Rect
    -> SvgThings.ControlId
    -> Spec
    -> ( Model, Command )
init rect cid spec =
    let
        model =
            Model spec.name
                (Maybe.withDefault "" spec.label)
                Nothing
                cid
                rect
                (SvgThings.SRect (String.fromInt rect.x)
                    (String.fromInt rect.y)
                    (String.fromInt rect.w)
                    (String.fromInt rect.h)
                )
                spec.orientation
                False
                0.5
                []
                False
    in
    ( model, resizeCommand model )


getX : JD.Decoder Int
getX =
    JD.field "pageX" JD.int


getY : JD.Decoder Int
getY =
    JD.field "pageY" JD.int


encodeUpdateMessage : UpdateMessage -> JD.Value
encodeUpdateMessage um =
    let
        outlist1 =
            [ ( "controlType", JE.string "slider" )
            , ( "controlId", SvgThings.encodeControlId um.controlId )
            ]

        outlist2 =
            case um.updateType of
                Just ut ->
                    List.append outlist1 [ ( "state", encodeUpdateType ut ) ]

                Nothing ->
                    outlist1

        outlist3 =
            case um.location of
                Just loc ->
                    List.append outlist2 [ ( "location", JE.float loc ) ]

                Nothing ->
                    outlist2

        outlist4 =
            case um.label of
                Just txt ->
                    List.append outlist3 [ ( "label", JE.string txt ) ]

                Nothing ->
                    outlist3
    in
    JE.object outlist4


encodeUpdateType : UpdateType -> JD.Value
encodeUpdateType ut =
    case ut of
        Press ->
            JE.string "Press"

        Unpress ->
            JE.string "Unpress"


jsUpdateMessage : JD.Decoder UpdateMessage
jsUpdateMessage =
    JD.map4 UpdateMessage
        (JD.field "controlId" SvgThings.decodeControlId)
        (JD.maybe (JD.field "state" JD.string |> JD.andThen jsUpdateType))
        (JD.maybe (JD.field "location" JD.float))
        (JD.maybe (JD.field "label" JD.string))


jsUpdateType : String -> JD.Decoder UpdateType
jsUpdateType ut =
    case ut of
        "Press" ->
            JD.succeed Press

        "Unpress" ->
            JD.succeed Unpress

        _ ->
            JD.succeed Unpress



-- get mouse/whatever location from the json message,
-- compute slider location from that.


getLocation : Model -> JD.Value -> Result String Float
getLocation model v =
    case model.orientation of
        SvgThings.Horizontal ->
            case JD.decodeValue getX v of
                Ok i ->
                    Ok
                        (toFloat (i - model.rect.x)
                            / toFloat model.rect.w
                        )

                Err e ->
                    Err (JD.errorToString e)

        SvgThings.Vertical ->
            case JD.decodeValue getY v of
                Ok i ->
                    Ok
                        (toFloat (i - model.rect.y)
                            / toFloat model.rect.h
                        )

                Err e ->
                    Err (JD.errorToString e)


update : Msg -> Model -> ( Model, Command )
update msg model =
    case msg of
        SvgPress v ->
            case getLocation model v of
                Ok l ->
                    updsend model (Just Press) l

                _ ->
                    ( model, None )

        SvgUnpress v ->
            case model.pressed of
                True ->
                    updsend model (Just Unpress) model.location

                False ->
                    ( model, None )

        NoOp ->
            ( model, None )

        Reply s ->
            ( { model | name = s }, None )

        SvgMoved v ->
            case model.pressed of
                True ->
                    case getLocation model v of
                        Ok l ->
                            updsend model Nothing l

                        _ ->
                            ( model, None )

                False ->
                    ( model, None )

        SvgUpdate um ->
            -- sanity check for ids?  or don't.
            let
                mod =
                    { model
                        | pressed =
                            case um.updateType of
                                Just Press ->
                                    True

                                Just Unpress ->
                                    False

                                _ ->
                                    model.pressed
                        , location =
                            case um.location of
                                Just loc ->
                                    loc

                                Nothing ->
                                    model.location
                    }

                mod2 =
                    case um.label of
                        Just txt ->
                            { mod
                                | label = txt
                                , textSvg = []
                                , stringWidth = Nothing
                            }

                        Nothing ->
                            mod
            in
            ( mod2, resizeCommand mod2 )

        SvgTouch stm ->
            case ST.extractFirstRectTouchSE stm model.rect of
                Nothing ->
                    if model.pressed then
                        updsend model (Just Unpress) model.location

                    else
                        ( model, None )

                Just touch ->
                    case model.orientation of
                        SvgThings.Horizontal ->
                            let
                                loc =
                                    (touch.x - toFloat model.rect.x)
                                        / toFloat model.rect.w
                            in
                            if model.pressed then
                                updsend model (Just Press) loc

                            else
                                updsend model Nothing loc

                        SvgThings.Vertical ->
                            let
                                loc =
                                    (touch.y - toFloat model.rect.y)
                                        / toFloat model.rect.h
                            in
                            if model.pressed then
                                updsend model (Just Press) loc

                            else
                                updsend model Nothing loc


updsend : Model -> Maybe UpdateType -> Float -> ( Model, Command )
updsend model mbut loc =
    let
        bLoc =
            if loc > 1.0 then
                1.0

            else if loc < 0.0 then
                0.0

            else
                loc

        prest =
            mbut /= Just Unpress
    in
    -- if nothing changed, no message.
    if model.location == bLoc && model.pressed == prest then
        ( model, None )

    else
        let
            um =
                JE.encode 0
                    (encodeUpdateMessage
                        (UpdateMessage model.cid mbut (Just bLoc) Nothing)
                    )
        in
        ( { model | location = bLoc, pressed = prest }
        , Send um
        )


resize : Model -> SvgThings.Rect -> ( Model, Command )
resize model rect =
    let
        newmodel =
            { model
                | rect = rect
                , srect =
                    SvgThings.SRect (String.fromInt rect.x)
                        (String.fromInt rect.y)
                        (String.fromInt rect.w)
                        (String.fromInt rect.h)
                , textSvg = []
                , stringWidth = Nothing
            }
    in
    ( newmodel, resizeCommand newmodel )



-- VIEW
-- try VD.onWithOptions for preventing scrolling on touchscreens and
-- etc. See virtualdom docs.


sliderEvt : String -> (JD.Value -> Msg) -> VD.Attribute Msg
sliderEvt evtname mkmsg =
    -- VD.onWithOptions evtname (VD.Options True True) (JD.map (\v -> mkmsg v) JD.value)
    VD.on evtname <|
        VD.Custom
            (JD.map
                (\v ->
                    { stopPropagation = True, preventDefault = True, message = mkmsg v }
                )
                JD.value
            )


onMouseDown =
    sliderEvt "mousedown" SvgPress


onMouseMove =
    sliderEvt "mousemove" SvgMoved


onMouseLeave =
    sliderEvt "mouseleave" SvgUnpress


onMouseUp =
    sliderEvt "mouseup" SvgUnpress


onTouchStart =
    sliderEvt "touchstart" (\e -> SvgTouch (ST.SvgTouchStart e))


onTouchEnd =
    sliderEvt "touchend" (\e -> SvgTouch (ST.SvgTouchEnd e))


onTouchCancel =
    sliderEvt "touchcancel" (\e -> SvgTouch (ST.SvgTouchCancel e))


onTouchLeave =
    sliderEvt "touchleave" (\e -> SvgTouch (ST.SvgTouchLeave e))


onTouchMove =
    sliderEvt "touchmove" (\e -> SvgTouch (ST.SvgTouchMove e))


buildEvtHandlerList : Bool -> List (VD.Attribute Msg)
buildEvtHandlerList touchonly =
    let
        te =
            [ onTouchStart
            , onTouchEnd
            , onTouchCancel
            , onTouchLeave
            , onTouchMove
            ]

        me =
            [ onMouseDown
            , onMouseUp
            , onMouseLeave
            , onMouseMove
            ]
    in
    if touchonly then
        te

    else
        List.append me te


view : UiTheme -> Model -> Svg Msg
view theme model =
    let
        (Toop.T4 sx sy sw sh) =
            case model.orientation of
                SvgThings.Vertical ->
                    Toop.T4 model.srect.x
                        (String.fromInt (round (model.location * toFloat model.rect.h) + model.rect.y))
                        model.srect.w
                        "3"

                SvgThings.Horizontal ->
                    Toop.T4 (String.fromInt (round (model.location * toFloat model.rect.w) + model.rect.x))
                        model.srect.y
                        "3"
                        model.srect.h

        evtlist =
            buildEvtHandlerList model.touchonly
    in
    g evtlist
        [ rect
            [ x model.srect.x
            , y model.srect.y
            , width model.srect.w
            , height model.srect.h
            , rx "2"
            , ry "2"
            , style ("fill: #" ++ theme.colorString Controls ++ ";")
            ]
            []
        , rect
            [ x sx
            , y sy
            , width sw
            , height sh
            , rx "2"
            , ry "2"
            , style
                ("fill: #"
                    ++ theme.colorString
                        (if model.pressed then
                            Pressed

                         else
                            Unpressed
                        )
                    ++ ";"
                )
            ]
            []
        , VD.map (\_ -> NoOp) (g [] model.textSvg)
        ]
