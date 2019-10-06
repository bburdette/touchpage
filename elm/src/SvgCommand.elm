module SvgCommand exposing (Command(..), TextSizeRequest)

import SvgThings exposing (ControlId)


type alias TextSizeRequest =
    { string : String
    , font : String
    , controlId : ControlId
    }


type Command
    = Send String
    | RequestTextWidth TextSizeRequest
    | None
    | Batch (List Command)
