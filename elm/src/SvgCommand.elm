module SvgCommand exposing (Command(..))

import SvgThings exposing (ControlId)


type Command
    = Send String
    | RequestTextWidth String ControlId
    | None
