module SvgControlPage exposing (ID, JsMessage(..), Model, Msg(..), Spec, init, jsMessage, jsSpec, update, view, viewSvgControl)

-- import Effects exposing (Effects, Never)
-- import Platform exposing (Cmd, none)

import Dict exposing (..)
import Html
import Json.Decode as JD
import List exposing (..)
import Svg
import Svg.Attributes as SA
import Svg.Events as SE
import SvgControl
import SvgThings
import Task
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
    , sendaddr : String
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


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        JsonMsg s ->
            case JD.decodeString jsMessage s of
                Ok (JmSpec spec) ->
                    init model.sendaddr model.mahrect spec

                Ok (JmUpdate jmact) ->
                    update jmact model

                Err e ->
                    ( { model | title = JD.errorToString e }, Cmd.none )

        CMsg act ->
            let
                wha =
                    SvgControl.update act model.control

                newmod =
                    { model | control = Tuple.first wha }
            in
            ( newmod, Cmd.map CMsg (Tuple.second wha) )

        Resize newSize ->
            let
                nr =
                    SvgThings.Rect 0 0 (round (newSize.width - 1)) (round (newSize.height - 4))

                ( ctrl, cmds ) =
                    SvgControl.resize model.control nr
            in
            ( { model
                | mahrect = nr
                , srect = SvgThings.toSRect nr
                , windowSize = newSize
                , control = ctrl
              }
            , Cmd.map CMsg cmds
            )

        NoOp ->
            ( model, Cmd.none )



{-
   Touche touchlist ->
     let tdict = touchDict model.control touchlist
         curtouches = Dict.map (\_ v -> fst v) tdict
         prevs = Dict.diff model.prevtouches curtouches in
     ({model | prevtouches = curtouches}, Cmd.batch
       (
         (List.map (\t -> Cmd.task (Task.succeed (CMsg t)))
             (List.filterMap (\(cid, (tam, tl)) ->
               Maybe.map (SvgControl.toCtrlMsg cid) (tam tl))
               (Dict.toList tdict)))
         ++
         (List.map (\t -> Cmd.task (Task.succeed (CMsg t)))
             (List.filterMap (\(cid, tam) ->
               Maybe.map (SvgControl.toCtrlMsg cid) (tam []))
               (Dict.toList prevs)))))
-}
-- build a dict of controls -> touches.
{-
   touchDict: SvgControl.Model -> (List SvgTouch.Touch) ->
       Dict SvgThings.ControlId (SvgControl.ControlTam, (List SvgTouch.Touch))
   touchDict root touches =
     let meh = List.filterMap (\t -> Maybe.andThen (SvgControl.findControl t.x t.y root) (\c -> Just (c,t))) touches in
     List.foldl updict Dict.empty meh

   updict: (SvgControl.Model, SvgTouch.Touch)
         -> Dict SvgThings.ControlId (SvgControl.ControlTam, (List SvgTouch.Touch))
         -> Dict SvgThings.ControlId (SvgControl.ControlTam, (List SvgTouch.Touch))
   updict mt dict =
     Dict.update (SvgControl.controlId (fst mt))
                 (\mbpair -> case mbpair of
                   Nothing -> Just (SvgControl.controlTouchMsgMaker (fst mt), [(snd mt)])
                   Just (a,b) -> Just (a, (snd mt) :: b))
                 dict
-}
{-

   Touche touchlist ->
     case head touchlist of
       Nothing -> ({model | title = "no touches" }, Cmd.none)
       Just touch ->
         case SvgControl.findControl touch.x touch.y model.control of
           Just control ->
             ({model | title = SvgControl.controlName control }, Cmd.none)
           Nothing -> ({model | title = "no touches" }, Cmd.none)


-}


init :
    String
    -> SvgThings.Rect
    -> Spec
    -> ( Model, Cmd Msg )
init sendaddr rect spec =
    let
        ( conmod, conevt ) =
            SvgControl.init sendaddr rect [] spec.rootControl

        ( updmod, evts ) =
            SvgControl.update_list (Maybe.withDefault [] spec.state) conmod

        -- combevts = Cmd.batch [conevt, evts]
    in
    ( Model spec.title
        rect
        (SvgThings.toSRect rect)
        spec
        updmod
        -- Dict.empty
        sendaddr
        (RectSize 0 0)
      -- , Task.perform (\x -> Resize x) Window.size
    , Cmd.none
    )



--    , Task.perform (\_ -> NoOp) (\x -> Resize x) Window.size)  -- add conevt evts to this??
-- VIEW


view : Model -> Html.Html Msg
view model =
    Html.div []
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
