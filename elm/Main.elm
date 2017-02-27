module Main exposing (..)

import SvgButton
import SvgSlider
import SvgControlPage
import SvgControl
import Task
import Task exposing (Task)


-- import Keyboard

import Char
import String
import WebSocket
import SvgThings
import Window


-- import SvgTouch

import SvgTextSize
import Html
-- import Html.App as App



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
            (\model ->
                Sub.batch
                    [ WebSocket.listen model.sendaddr SvgControlPage.JsonMsg
                    , Window.resizes SvgControlPage.Resize
                    ]
            )
        }


init : String -> ( SvgControlPage.Model, Cmd SvgControlPage.Msg )
init wsUrl =
    SvgControlPage.init
        wsUrl
        (SvgThings.Rect 0 0 500 300)
        (SvgControlPage.Spec wsUrl (SvgControl.CsSlider (SvgSlider.Spec "blah" Nothing SvgThings.Vertical)) Nothing)
