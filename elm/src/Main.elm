port module Main exposing (init, main)

-- import Keyboard
-- import SvgTouch

import Browser
import Browser.Events as BE
import Char
import Html
import Json.Decode as JD
import Json.Encode as JE
import String
import SvgButton
import SvgCommand exposing (Command(..))
import SvgControl
import SvgControlPage
import SvgSlider
import SvgTextSize
import SvgThings
import Task exposing (Task)
import Util exposing (RectSize)
import WebSocket


port receiveSocketMsg : (JD.Value -> msg) -> Sub msg


port sendSocketCommand : JE.Value -> Cmd msg


wssend =
    WebSocket.send sendSocketCommand


wsreceive =
    receiveSocketMsg <|
        WebSocket.receive WsMsg


type Msg
    = WsMsg (Result JD.Error WebSocket.WebSocketMsg)
    | ScpMsg SvgControlPage.Msg


type alias Flags =
    { location : String
    , wsport : Int
    , width : Int
    , height : Int
    }


type alias Model =
    { scpModel : SvgControlPage.Model
    , wsUrl : String
    }


main : Program Flags Model Msg
main =
    Browser.document
        { init =
            \flags ->
                let
                    mod =
                        init flags
                in
                ( mod
                , wssend <|
                    WebSocket.Connect
                        { name = "touchpage"
                        , address = mod.wsUrl
                        , protocol = "rust-websocket"
                        }
                )
        , subscriptions =
            \_ ->
                Sub.batch
                    [ Sub.map ScpMsg <|
                        BE.onResize
                            (\a b ->
                                SvgControlPage.Resize <|
                                    RectSize (toFloat a) (toFloat b)
                            )
                    , wsreceive
                    ]
        , update =
            \msg mod ->
                case msg of
                    ScpMsg sm ->
                        let
                            ( umod, cmd ) =
                                SvgControlPage.update sm mod.scpModel

                            _ =
                                Debug.log "cmd: " cmd
                        in
                        -- ( umod, Cmd.map ScpMsg cmd )
                        ( { mod | scpModel = umod }
                        , case cmd of
                            Send dta ->
                                wssend <|
                                    WebSocket.Send
                                        { name = "touchpage"
                                        , content = dta
                                        }

                            None ->
                                Cmd.none
                        )

                    WsMsg x ->
                        let
                            _ =
                                Debug.log "wsmsg: " x
                        in
                        ( mod, Cmd.none )
        , view =
            \model ->
                Browser.Document "svg control"
                    [ Html.map ScpMsg <| SvgControlPage.view model.scpModel ]
        }



{- Html.programWithFlags
   { init = init
   , update = SvgControlPage.update
   , view = SvgControlPage.view
   , subscriptions =
       \model ->
           Sub.batch
               [ Window.resizes SvgControlPage.Resize

               -- WebSocket.listen model.sendaddr SvgControlPage.JsonMsg
               ]
   }


-}


init : Flags -> Model
init flags =
    let
        wsUrl =
            String.split ":" flags.location
                |> List.tail
                |> Maybe.andThen List.head
                |> Maybe.map (\loc -> "ws:" ++ loc ++ ":" ++ String.fromInt flags.wsport)
                |> Maybe.withDefault ""
    in
    { scpModel =
        SvgControlPage.init
            (SvgThings.Rect 0 0 flags.width flags.height)
            (SvgControlPage.Spec wsUrl (SvgControl.CsSlider (SvgSlider.Spec "blah" Nothing SvgThings.Vertical)) Nothing)
    , wsUrl = wsUrl
    }
