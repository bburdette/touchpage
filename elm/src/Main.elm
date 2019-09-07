module Main exposing (Msg(..), init, main, wsUrl)

-- import Keyboard
-- import SvgTouch

import Char
import Html
import String
import SvgButton
import SvgControl
import SvgControlPage
import SvgSlider
import SvgTextSize
import SvgThings
import Task exposing (Task)



-- import WebSocket
---------------------------------------


wsUrl : String
wsUrl =
    "ws://localhost:1234"


type Msg
    = Receive String
    | Send


main =
    Html.programWithFlags
        { init = init
        , update = SvgControlPage.update
        , view = SvgControlPage.view
        , subscriptions =
            \model ->
                Sub.batch
                    [ WebSocket.listen model.sendaddr SvgControlPage.JsonMsg
                    , Window.resizes SvgControlPage.Resize
                    ]
        }


init : String -> ( SvgControlPage.Model, Cmd SvgControlPage.Msg )
init wsUrl =
    SvgControlPage.init
        wsUrl
        (SvgThings.Rect 0 0 500 300)
        (SvgControlPage.Spec wsUrl (SvgControl.CsSlider (SvgSlider.Spec "blah" Nothing SvgThings.Vertical)) Nothing)
