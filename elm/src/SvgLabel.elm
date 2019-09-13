module SvgLabel exposing (Model, Msg(..), Spec, UpdateMessage, init, jsSpec, jsUpdateMessage, resize, update, view)

import Html exposing (Html)
import Html.Events exposing (onClick, onMouseDown, onMouseOut, onMouseUp)
import Json.Decode as JD
import Json.Encode as JE
import String
import Svg exposing (Attribute, Svg, g, rect, svg, text)
import Svg.Attributes exposing (..)
import SvgTextSize exposing (..)
import SvgThings
import Task
import Template exposing (render, template)
import Time exposing (..)
import VirtualDom as VD


type alias Spec =
    { name : String
    , label : String
    }


jsSpec : JD.Decoder Spec
jsSpec =
    JD.map2 Spec
        (JD.field "name" JD.string)
        (JD.field "label" JD.string)


type alias Model =
    { name : String
    , label : String
    , cid : SvgThings.ControlId
    , rect : SvgThings.Rect
    , srect : SvgThings.SRect
    , textSvg : List (Svg Msg)
    }


type Msg
    = SvgUpdate UpdateMessage
    | NoOp



--    | SvgTouch (List Touch.Touch)


type alias UpdateMessage =
    { controlId : SvgThings.ControlId
    , label : String
    }


jsUpdateMessage : JD.Decoder UpdateMessage
jsUpdateMessage =
    JD.map2 UpdateMessage
        (JD.field "controlId" SvgThings.decodeControlId)
        (JD.field "label" JD.string)


init :
    SvgThings.Rect
    -> SvgThings.ControlId
    -> Spec
    -> ( Model, Cmd msg )
init rect cid spec =
    let
        ts =
            SvgThings.calcTextSvg SvgThings.ff spec.label rect
    in
    ( Model spec.name
        spec.label
        cid
        rect
        (SvgThings.SRect (String.fromInt rect.x)
            (String.fromInt rect.y)
            (String.fromInt rect.w)
            (String.fromInt rect.h)
        )
        (List.map (\meh -> VD.map (\_ -> NoOp) meh) ts)
    , Cmd.none
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        SvgUpdate um ->
            let
                tswk =
                    SvgThings.calcTextSvg SvgThings.ff um.label model.rect

                ts =
                    List.map (\meh -> VD.map (\_ -> NoOp) meh) tswk
            in
            ( { model | label = um.label, textSvg = ts }
            , Cmd.none
            )

        NoOp ->
            ( model, Cmd.none )



--    SvgTouch touches -> (model, Cmd.none)


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
        , textSvg = List.map (\meh -> VD.map (\_ -> NoOp) meh) ts
      }
    , Cmd.none
    )


view : Model -> Svg Msg
view model =
    let
        lbrect =
            rect
                [ x model.srect.x
                , y model.srect.y
                , width model.srect.w
                , height model.srect.h
                , rx "15"
                , ry "15"
                , style "fill: #A1A1A1;"
                ]
                []

        svgl =
            lbrect :: model.textSvg
    in
    VD.map (\_ -> NoOp) (g [] svgl)
