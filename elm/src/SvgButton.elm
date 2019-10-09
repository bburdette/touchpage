module SvgButton exposing (Model, Msg(..), Spec, UpdateMessage, UpdateType(..), buildEvtHandlerList, buttonEvt, encodeUpdateMessage, encodeUpdateType, init, jsSpec, jsUpdateMessage, jsUpdateType, onTouchCancel, onTouchEnd, onTouchLeave, onTouchMove, onTouchStart, pressedColor, pressup, resize, update, view)

import Dict
import Html exposing (Html)
import Html.Events exposing (onClick, onMouseDown, onMouseOut, onMouseUp)
import Json.Decode as JD
import Json.Encode as JE
import Svg exposing (Attribute, Svg, g, rect, svg, text)
import Svg.Attributes exposing (..)
import SvgCommand exposing (Command(..))
import SvgTextSize exposing (calcTextSvg, resizeCommand)
import SvgThings exposing (UiColor(..), UiTheme)
import SvgTouch as ST
import Task
import VirtualDom as VD


type alias Spec =
    { name : String
    , label : Maybe String
    }


jsSpec : JD.Decoder Spec
jsSpec =
    JD.map2 Spec
        (JD.field "name" JD.string)
        (JD.maybe (JD.field "label" JD.string))


type alias Model =
    { name : String
    , label : String
    , stringWidth : Maybe Float
    , cid : SvgThings.ControlId
    , rect : SvgThings.Rect
    , srect : SvgThings.SRect
    , pressed : Bool
    , textSvg : List (Svg ())
    , touchonly : Bool
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
                False
                []
                False
    in
    ( model, resizeCommand model )


pressedColor : Bool -> String
pressedColor pressed =
    case pressed of
        True ->
            "#f000f0"

        False ->
            "#60B5CC"


type Msg
    = SvgPress
    | SvgUnpress
    | NoOp
    | Reply String
    | SvgTouch ST.Msg
    | SvgUpdate UpdateMessage


type UpdateType
    = Press
    | Unpress


type alias UpdateMessage =
    { controlId : SvgThings.ControlId
    , updateType : Maybe UpdateType
    , label : Maybe String
    }


encodeUpdateMessage : UpdateMessage -> JD.Value
encodeUpdateMessage um =
    let
        outlist1 =
            [ ( "controlType", JE.string "button" )
            , ( "controlId", SvgThings.encodeControlId um.controlId )
            ]

        outlist2 =
            case um.updateType of
                Just ut ->
                    List.append outlist1 [ ( "state", encodeUpdateType ut ) ]

                Nothing ->
                    outlist1

        outlist3 =
            case um.label of
                Just txt ->
                    List.append outlist2 [ ( "label", JE.string txt ) ]

                Nothing ->
                    outlist2
    in
    JE.object outlist3


encodeUpdateType : UpdateType -> JD.Value
encodeUpdateType ut =
    case ut of
        Press ->
            JE.string "Press"

        Unpress ->
            JE.string "Unpress"


jsUpdateMessage : JD.Decoder UpdateMessage
jsUpdateMessage =
    JD.map3 UpdateMessage
        (JD.field "controlId" SvgThings.decodeControlId)
        (JD.maybe (JD.field "state" JD.string |> JD.andThen jsUpdateType))
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


update : Msg -> Model -> ( Model, Command )
update msg model =
    case msg of
        SvgPress ->
            pressup model Press

        SvgUnpress ->
            if model.pressed then
                pressup model Unpress

            else
                ( model, None )

        NoOp ->
            ( model, None )

        Reply s ->
            ( { model | name = s }, None )

        SvgUpdate um ->
            -- sanity check for ids?  or don't.
            let
                nm1 =
                    { model
                        | pressed =
                            case um.updateType of
                                Just Press ->
                                    True

                                Just Unpress ->
                                    False

                                _ ->
                                    model.pressed
                    }

                nm2 =
                    case um.label of
                        Just txt ->
                            { nm1
                                | label = txt
                                , stringWidth = Nothing
                                , textSvg = []
                            }

                        Nothing ->
                            nm1
            in
            ( nm2, resizeCommand nm2 )

        SvgTouch stm ->
            case ST.extractFirstTouchSE stm of
                Nothing ->
                    if model.pressed == True then
                        pressup model Unpress

                    else
                        ( model, None )

                Just _ ->
                    if model.pressed == False then
                        pressup model Press

                    else
                        ( model, None )


pressup : Model -> UpdateType -> ( Model, Command )
pressup model ut =
    let
        um =
            JE.encode 0
                (encodeUpdateMessage
                    (UpdateMessage model.cid (Just ut) Nothing)
                )
    in
    ( { model | pressed = ut == Press }
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


buttonEvt : String -> (JD.Value -> Msg) -> VD.Attribute Msg
buttonEvt evtname mkmsg =
    VD.on evtname <|
        VD.Custom
            (JD.map
                (\v ->
                    { stopPropagation = True, preventDefault = True, message = mkmsg v }
                )
                JD.value
            )


onTouchStart =
    buttonEvt "touchstart" (\e -> SvgTouch (ST.SvgTouchStart e))


onTouchEnd =
    buttonEvt "touchend" (\e -> SvgTouch (ST.SvgTouchEnd e))


onTouchCancel =
    buttonEvt "touchcancel" (\e -> SvgTouch (ST.SvgTouchCancel e))


onTouchLeave =
    buttonEvt "touchleave" (\e -> SvgTouch (ST.SvgTouchLeave e))


onTouchMove =
    buttonEvt "touchmove" (\e -> SvgTouch (ST.SvgTouchMove e))


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
            [ onMouseDown SvgPress
            , onMouseUp SvgUnpress
            , onMouseOut SvgUnpress
            ]
    in
    if touchonly then
        te

    else
        List.append me te


view : UiTheme -> Model -> Svg Msg
view theme model =
    g (buildEvtHandlerList model.touchonly)
        [ rect
            [ x model.srect.x
            , y model.srect.y
            , width model.srect.w
            , height model.srect.h
            , rx "15"
            , ry "15"
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
