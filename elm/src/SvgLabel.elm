module SvgLabel exposing (Model, Msg(..), Spec, UpdateMessage, init, jsSpec, jsUpdateMessage, resize, update, view)

import Html exposing (Html)
import Html.Events exposing (onClick, onMouseDown, onMouseOut, onMouseUp)
import Json.Decode as JD
import Json.Encode as JE
import String
import Svg exposing (Attribute, Svg, g, rect, svg, text)
import Svg.Attributes exposing (..)
import SvgCommand exposing (Command(..))
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
    , stringWidth : Maybe Float
    , cid : SvgThings.ControlId
    , rect : SvgThings.Rect
    , srect : SvgThings.SRect
    , textSvg : List (Svg ())
    }


type Msg
    = SvgUpdate UpdateMessage
    | NoOp


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
    -> ( Model, Command )
init rect cid spec =
    -- let
    --     ts =
    --         calcTextSvg SvgThings.ff spec.label rect
    -- in
    let
        model =
            Model spec.name
                spec.label
                Nothing
                cid
                rect
                (SvgThings.SRect (String.fromInt rect.x)
                    (String.fromInt rect.y)
                    (String.fromInt rect.w)
                    (String.fromInt rect.h)
                )
                []
    in
    ( model, resizeCommand model )


update : Msg -> Model -> ( Model, Command )
update msg model =
    case msg of
        SvgUpdate um ->
            {- let
                   tswk =
                       calcTextSvg SvgThings.ff um.label model.rect

                   ts =
                       List.map (\meh -> VD.map (\_ -> NoOp) meh) tswk
               in
            -}
            let
                newmodel =
                    { model | label = um.label, textSvg = [] }
            in
            ( newmodel, resizeCommand newmodel )

        NoOp ->
            ( model, None )


resize : Model -> SvgThings.Rect -> ( Model, Command )
resize model rect =
    let
        ts =
            calcTextSvgM model

        -- |> List.map (\meh -> VD.map (\_ -> NoOp) meh)
        {- model.stringWidth
           |> Maybe.map
               (\sw ->
                   calcTextSvg SvgThings.ff model.label sw rect
                       |> List.map (\meh -> VD.map (\_ -> NoOp) meh)
               )
           |> Maybe.withDefault []
        -}
        newmodel =
            { model
                | rect = rect
                , srect =
                    SvgThings.SRect (String.fromInt rect.x)
                        (String.fromInt rect.y)
                        (String.fromInt rect.w)
                        (String.fromInt rect.h)
                , textSvg = ts
            }
    in
    ( newmodel, resizeCommand newmodel )


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

        _ =
            Debug.log "label view : " model.textSvg

        svgl =
            lbrect :: model.textSvg
    in
    VD.map (\_ -> NoOp) (g [] svgl)
