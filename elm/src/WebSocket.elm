module WebSocket exposing (WebSocketCmd(..), WebSocketMsg(..), decodeMsg, encodeCmd, receive, send)

import Json.Decode as JD
import Json.Encode as JE


{-| make you some ports like this!

port receiveSocketMsg : (JD.Value -> msg) -> Sub msg
port sendSocketCommand : JE.Value -> Cmd msg

-}
send : (JE.Value -> Cmd msg) -> WebSocketCmd -> Cmd msg
send tocmd wsc =
    tocmd (encodeCmd wsc)


receive : (Result JD.Error WebSocketMsg -> msg) -> (JD.Value -> msg)
receive wsmMsg =
    \v ->
        JD.decodeValue decodeMsg v
            |> wsmMsg


type WebSocketCmd
    = Connect { name : String, address : String, protocol : String }
    | Send { name : String, content : String }
    | Close { name : String }


type WebSocketMsg
    = Error { name : String, error : String }
    | Data { name : String, data : String }


encodeCmd : WebSocketCmd -> JE.Value
encodeCmd wsc =
    case wsc of
        Connect msg ->
            JE.object
                [ ( "cmd", JE.string "connect" )
                , ( "name", JE.string msg.name )
                , ( "address", JE.string msg.address )
                , ( "protocol", JE.string msg.protocol )
                ]

        Send msg ->
            JE.object
                [ ( "cmd", JE.string "send" )
                , ( "name", JE.string msg.name )
                , ( "content", JE.string msg.content )
                ]

        Close msg ->
            JE.object
                [ ( "cmd", JE.string "close" )
                , ( "name", JE.string msg.name )
                ]


decodeMsg : JD.Decoder WebSocketMsg
decodeMsg =
    JD.field "msg" JD.string
        |> JD.andThen
            (\msg ->
                case msg of
                    "error" ->
                        JD.map2 (\a b -> Error { name = a, error = b })
                            (JD.field "name" JD.string)
                            (JD.field "error" JD.string)

                    "data" ->
                        JD.map2 (\a b -> Data { name = a, data = b })
                            (JD.field "name" JD.string)
                            (JD.field "data" JD.string)

                    unk ->
                        JD.fail <| "unknown websocketmsg type: " ++ unk
            )
