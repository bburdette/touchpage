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


main : Program Flags SvgControlPage.Model Msg
main =
    Browser.document
        { init =
            \flags ->
                let
                    ( mod, cmd ) =
                        init flags
                in
                ( mod
                , Cmd.batch
                    [ Cmd.map ScpMsg cmd
                    , wssend <|
                        WebSocket.Connect
                            { name = "touchpage"
                            , address = mod.sendaddr
                            , protocol = "rust-websocket"
                            }
                    ]
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
                                SvgControlPage.update sm mod
                        in
                        ( umod, Cmd.map ScpMsg cmd )

                    WsMsg x ->
                        let
                            _ =
                                Debug.log "wsmsg: " x
                        in
                        ( mod, Cmd.none )
        , view =
            \model ->
                Browser.Document "svg control"
                    [ Html.map ScpMsg <| SvgControlPage.view model ]
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


init : Flags -> ( SvgControlPage.Model, Cmd SvgControlPage.Msg )
init flags =
    let
        wsUrl =
            String.split ":" flags.location
                |> List.tail
                |> Maybe.andThen List.head
                |> Maybe.map (\loc -> "ws:" ++ loc ++ ":" ++ String.fromInt flags.wsport)
                |> Maybe.withDefault ""
    in
    SvgControlPage.init
        wsUrl
        (SvgThings.Rect 0 0 flags.width flags.height)
        (SvgControlPage.Spec wsUrl (SvgControl.CsSlider (SvgSlider.Spec "blah" Nothing SvgThings.Vertical)) Nothing)
