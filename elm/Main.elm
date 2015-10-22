
import Effects exposing (Never)
import BlahButton 
import MultiButt 
import StartApp
import Task
import Signal exposing (Signal)
import Task exposing (Task)
import Keyboard
import Char
import String
import WebSocket exposing (WebSocket)

---------------------------------------

socket : Task x WebSocket
socket = WebSocket.create "ws://localhost:1234"

listen : Signal.Mailbox String
listen = Signal.mailbox ""

port listening : Task x (List ())
port listening = socket `Task.andThen` 
  (\s -> 
    Task.sequence [WebSocket.listen listen.address s, 
                   WebSocket.connected connected.address s])

connected : Signal.Mailbox Bool
connected = Signal.mailbox False

send : String -> Task x ()
send message = socket `Task.andThen` WebSocket.send message

port sending : Signal (Task x ())
port sending = Signal.map send inputKeyboard

inputKeyboard : Signal String
-- inputKeyboard = Signal.map (\c -> Char.fromCode c |> String.fromChar) Keyboard.presses
inputKeyboard = Signal.map (\c -> toString c) Keyboard.presses

---------------------------------------

app =
  StartApp.start
    { init = MultiButt.init send (MultiButt.Spec "mehtitle" [BlahButton.Spec "cabbage",
                                                             BlahButton.Spec "grits"])
    , update = MultiButt.update
    , view = MultiButt.view
    , inputs = [(Signal.map MultiButt.JsonMsg listen.signal)]
    }

{-
app =
  StartApp.start
    { init = BlahButton.init send (BlahButton.Spec "cabbage")
    , update = BlahButton.update
    , view = BlahButton.view
    , inputs = [(Signal.map BlahButton.Reply listen.signal)]
    -- , inputs = []
    -- , inputs = [inputKeyboard, listen.signal, (Signal.map toString connected.signal)]
    -- , inputs = [inputKeyboard]
    }
-}

main =
  app.html

port tasks : Signal (Task.Task Never ())
port tasks =
  app.tasks

