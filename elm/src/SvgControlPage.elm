module SvgControlPage exposing (ID, JsMessage(..), Model, Msg(..), Spec, init, jsMessage, jsSpec, update, view, viewSvgControl)

import Dict exposing (..)
import Html
import Html.Attributes exposing (style)
import Json.Decode as JD
import List exposing (..)
import Svg
import Svg.Attributes as SA
import SvgCommand exposing (Command(..))
import SvgControl
import SvgThings
import Util exposing (RectSize)
import VirtualDom as VD


type alias Spec =
    { title : String
    , rootControl : SvgControl.Spec
    , state : Maybe (List SvgControl.Msg)
    }


jsSpec : JD.Decoder Spec
jsSpec =
    JD.map3 Spec
        (JD.field "title" JD.string)
        (JD.field "rootControl" SvgControl.jsSpec)
        (JD.maybe (JD.field "state" (JD.list SvgControl.jsUpdateMessage)))


type alias Model =
    { title : String
    , mahrect : SvgThings.Rect
    , srect : SvgThings.SRect
    , spec : Spec
    , control : SvgControl.Model
    , windowSize : RectSize
    }


type alias ID =
    Int


type Msg
    = JsonMsg String
    | CMsg SvgControl.Msg
    | Resize RectSize
    | NoOp


type JsMessage
    = JmSpec Spec
    | JmUpdate Msg


jsMessage : JD.Decoder JsMessage
jsMessage =
    JD.oneOf
        [ jsSpec |> JD.andThen (\x -> JD.succeed (JmSpec x))
        , SvgControl.jsUpdateMessage
            |> JD.andThen
                (\x -> JD.succeed (JmUpdate (CMsg x)))
        ]


update : Msg -> Model -> ( Model, Command )
update msg model =
    case msg of
        JsonMsg s ->
            case JD.decodeString jsMessage s of
                Ok (JmSpec spec) ->
                    ( init model.mahrect spec, None )

                Ok (JmUpdate jmact) ->
                    update jmact model

                Err e ->
                    ( { model | title = JD.errorToString e }, None )

        CMsg act ->
            let
                wha =
                    SvgControl.update act model.control

                newmod =
                    { model | control = Tuple.first wha }
            in
            ( newmod, Tuple.second wha )

        Resize newSize ->
            let
                nr =
                    SvgThings.Rect 0 0 (round (newSize.width - 1)) (round (newSize.height - 4))

                ctrl =
                    SvgControl.resize model.control nr
            in
            ( { model
                | mahrect = nr
                , srect = SvgThings.toSRect nr
                , windowSize = newSize
                , control = ctrl
              }
            , None
            )

        NoOp ->
            ( model, None )


init :
    SvgThings.Rect
    -> Spec
    -> Model
init rect spec =
    let
        conmod =
            SvgControl.init rect [] spec.rootControl

        -- throwing away commands!
        ( updmod, cmds ) =
            SvgControl.update_list (Maybe.withDefault [] spec.state) conmod
    in
    Model spec.title
        rect
        (SvgThings.toSRect rect)
        spec
        updmod
        (RectSize 0 0)


view : Model -> Html.Html Msg
view model =
    Html.div [ style "margin" "0" ]
        [ Svg.svg
            [ SA.width model.srect.w
            , SA.height model.srect.h
            , SA.viewBox
                (model.srect.x
                    ++ " "
                    ++ model.srect.y
                    ++ " "
                    ++ model.srect.w
                    ++ " "
                    ++ model.srect.h
                )
            ]
            [ VD.map CMsg (viewSvgControl model.control) ]
        ]


viewSvgControl : SvgControl.Model -> Svg.Svg SvgControl.Msg
viewSvgControl conmodel =
    SvgControl.view conmodel
