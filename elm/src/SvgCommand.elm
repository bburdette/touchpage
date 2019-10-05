module SvgCommand exposing (Command(..))

import SvgTextSize exposing (TextSizeRequest)
import SvgThings exposing (ControlId)


type Command
    = Send String
    | RequestTextWidth TextSizeRequest
    | None
