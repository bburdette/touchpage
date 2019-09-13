module SvgButton exposing (Model, Msg(..), Spec, UpdateMessage, UpdateType(..), buildEvtHandlerList, buttonEvt, encodeUpdateMessage, encodeUpdateType, init, jsSpec, jsUpdateMessage, jsUpdateType, onTouchCancel, onTouchEnd, onTouchLeave, onTouchMove, onTouchStart, pressedColor, pressup, resize, update, view)

import Dict
import Html exposing (Html)
import Html.Events exposing (onClick, onMouseDown, onMouseOut, onMouseUp)
import Json.Decode as JD
import Json.Encode as JE
import Svg exposing (Attribute, Svg, g, rect, svg, text)
import Svg.Attributes exposing (..)
import SvgThings
import SvgTouch as ST
import Task
import VirtualDom as VD



-- how to specify a button in json.


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
    , cid : SvgThings.ControlId
    , rect : SvgThings.Rect
    , srect : SvgThings.SRect
    , pressed : Bool
    , sendaddr : String
    , textSvg : List (Svg ())
    , touchonly : Bool
    }


init :
    String
    -> SvgThings.Rect
    -> SvgThings.ControlId
    -> Spec
    -> ( Model, Cmd Msg )
init sendaddr rect cid spec =
    let
        ts =
            case spec.label of
                Just lbtext ->
                    SvgThings.calcTextSvg SvgThings.ff lbtext rect

                Nothing ->
                    []
    in
    ( Model spec.name
        (Maybe.withDefault "" spec.label)
        cid
        rect
        (SvgThings.SRect (String.fromInt rect.x)
            (String.fromInt rect.y)
            (String.fromInt rect.w)
            (String.fromInt rect.h)
        )
        False
        sendaddr
        ts
        False
    , Cmd.none
    )


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



{- JE.object [ ("controlType", JE.string "button")
   , ("controlId", SvgThings.encodeControlId um.controlId)
   , ("updateType", encodeUpdateType um.updateType)
   ]
-}


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


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        SvgPress ->
            pressup model Press

        SvgUnpress ->
            if model.pressed then
                pressup model Unpress

            else
                ( model, Cmd.none )

        NoOp ->
            ( model, Cmd.none )

        Reply s ->
            ( { model | name = s }, Cmd.none )

        SvgUpdate um ->
            -- sanity check for ids?  or don't.
            ( { model
                | pressed =
                    case um.updateType of
                        Just Press ->
                            True

                        Just Unpress ->
                            False

                        _ ->
                            model.pressed
                , label =
                    case um.label of
                        Just txt ->
                            txt

                        Nothing ->
                            model.label
                , textSvg =
                    case um.label of
                        Just txt ->
                            SvgThings.calcTextSvg SvgThings.ff txt model.rect

                        Nothing ->
                            model.textSvg
              }
            , Cmd.none
            )

        SvgTouch stm ->
            case ST.extractFirstTouchSE stm of
                Nothing ->
                    if model.pressed == True then
                        pressup model Unpress

                    else
                        ( model, Cmd.none )

                Just _ ->
                    if model.pressed == False then
                        pressup model Press

                    else
                        ( model, Cmd.none )


pressup : Model -> UpdateType -> ( Model, Cmd Msg )
pressup model ut =
    let
        um =
            JE.encode 0
                (encodeUpdateMessage
                    (UpdateMessage model.cid (Just ut) Nothing)
                )
    in
    ( { model | pressed = ut == Press }
    , Cmd.none {- TODO implement.   , WebSocket.send model.sendaddr um -}
    )


resize : Model -> SvgThings.Rect -> ( Model, Cmd Msg )
resize model rect =
    let
        ts =
            SvgThings.calcTextSvg SvgThings.ff model.label rect
    in
    ( { model
        | rect = rect
        , srect =
            SvgThings.SRect (String.fromInt rect.x)
                (String.fromInt rect.y)
                (String.fromInt rect.w)
                (String.fromInt rect.h)
        , textSvg = ts
      }
    , Cmd.none
    )


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



-- VD.onWithOptions evtname (VD.Options True True) (JD.map (\v -> mkmsg v) JD.value)


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


view : Model -> Svg Msg
view model =
    g (buildEvtHandlerList model.touchonly)
        [ rect
            [ x model.srect.x
            , y model.srect.y
            , width model.srect.w
            , height model.srect.h
            , rx "15"
            , ry "15"
            , style ("fill: " ++ pressedColor model.pressed ++ ";")
            ]
            []
        , VD.map (\_ -> NoOp) (g [] model.textSvg)
        ]
