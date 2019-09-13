module Main exposing (Msg(..), init, main)

-- import Keyboard
-- import SvgTouch

import Browser
import Browser.Events as BE
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
import Util exposing (RectSize)



-- import WebSocket
---------------------------------------
{- wsUrl : String
   wsUrl =
       "ws://localhost:1234"
-}


type Msg
    = Receive String
    | Send


type alias Flags =
    { location : String
    , wsport : Int
    , width : Int
    , height : Int
    }


main : Program Flags SvgControlPage.Model SvgControlPage.Msg
main =
    Browser.document
        { init = init
        , subscriptions = \_ -> BE.onResize (\a b -> SvgControlPage.Resize <| RectSize (toFloat a) (toFloat b))
        , update = SvgControlPage.update
        , view =
            \model ->
                Browser.Document "svg control"
                    [ SvgControlPage.view model ]
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

        _ =
            Debug.log "wsurl" <| wsUrl
    in
    SvgControlPage.init
        wsUrl
        (SvgThings.Rect 0 0 500 300)
        (SvgControlPage.Spec wsUrl (SvgControl.CsSlider (SvgSlider.Spec "blah" Nothing SvgThings.Vertical)) Nothing)
