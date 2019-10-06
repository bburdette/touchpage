port module Main exposing (init, main)

import Browser
import Browser.Events as BE
import Html
import Json.Decode as JD
import Json.Encode as JE
import SvgCommand exposing (Command(..), TextSizeRequest)
import SvgControl
import SvgControlPage
import SvgSlider
import SvgTextSize exposing (TextSizeReply, decodeTextSizeReply, encodeTextSizeRequest, estimateTextWidth)
import SvgThings
import Util exposing (RectSize)
import WebSocket


port receiveSocketMsg : (JD.Value -> msg) -> Sub msg


port sendSocketCommand : JE.Value -> Cmd msg


port requestTextSize : JE.Value -> Cmd msg


port receiveTextMetrics : (JD.Value -> msg) -> Sub msg


wssend =
    WebSocket.send sendSocketCommand


wsreceive =
    receiveSocketMsg <| WebSocket.receive WsMsg


type Msg
    = WsMsg (Result JD.Error WebSocket.WebSocketMsg)
    | TextSize (Result JD.Error TextSizeReply)
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


commandToCmd : SvgCommand.Command -> Cmd Msg
commandToCmd scmd =
    case scmd of
        Send dta ->
            wssend <|
                WebSocket.Send
                    { name = "touchpage"
                    , content = dta
                    }

        RequestTextWidth rtw ->
            requestTextSize <|
                encodeTextSizeRequest <|
                    rtw

        -- TextSizeRequest "blah" "20px sans-serif" []
        None ->
            -- test socket close.
            -- wssend <| WebSocket.Close { name = "touchpage" }
            Cmd.none

        Batch cmds ->
            Cmd.batch (List.map commandToCmd cmds)


main : Program Flags Model Msg
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
                    [ wssend <|
                        WebSocket.Connect
                            { name = "touchpage"
                            , address = mod.wsUrl
                            , protocol = "rust-websocket"
                            }
                    , requestTextSize <|
                        encodeTextSizeRequest <|
                            TextSizeRequest "blah" "20px sans-serif" []
                    , commandToCmd cmd
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
                    , receiveTextMetrics (TextSize << JD.decodeValue decodeTextSizeReply)
                    ]
        , update =
            \msg mod ->
                case msg of
                    ScpMsg sm ->
                        let
                            ( umod, cmd ) =
                                SvgControlPage.update sm mod.scpModel
                        in
                        -- ( umod, Cmd.map ScpMsg cmd )
                        ( { mod | scpModel = umod }
                        , commandToCmd cmd
                        )

                    WsMsg x ->
                        case x of
                            Ok (WebSocket.Data wsd) ->
                                let
                                    ( scpModel, scpCommand ) =
                                        SvgControlPage.update (SvgControlPage.JsonMsg wsd.data) mod.scpModel
                                in
                                ( { mod | scpModel = scpModel }, commandToCmd scpCommand )

                            Ok (WebSocket.Error wse) ->
                                ( mod, Cmd.none )

                            Err _ ->
                                ( mod, Cmd.none )

                    TextSize ts ->
                        case ts of
                            Ok tsr ->
                                ( { mod | scpModel = SvgControlPage.onTextSize tsr mod.scpModel }
                                , Cmd.none
                                )

                            Err _ ->
                                ( mod, Cmd.none )
        , view =
            \model ->
                Browser.Document "svg control"
                    [ Html.map ScpMsg <| SvgControlPage.view model.scpModel ]
        }


init : Flags -> ( Model, Command )
init flags =
    let
        wsUrl =
            String.split ":" flags.location
                |> List.tail
                |> Maybe.andThen List.head
                |> Maybe.map (\loc -> "ws:" ++ loc ++ ":" ++ String.fromInt flags.wsport)
                |> Maybe.withDefault ""

        ( sm, cmd ) =
            SvgControlPage.init
                (SvgThings.Rect 0 0 flags.width flags.height)
                (SvgControlPage.Spec wsUrl (SvgControl.CsSlider (SvgSlider.Spec "blah" Nothing SvgThings.Vertical)) Nothing)
    in
    ( { scpModel = sm
      , wsUrl = wsUrl
      }
    , cmd
    )
