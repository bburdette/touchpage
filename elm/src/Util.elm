module Util exposing (RectSize, andMap)

import Json.Decode as JD exposing (Decoder)


type alias RectSize =
    { width : Float, height : Float }


andMap : Decoder a -> Decoder (a -> b) -> Decoder b
andMap dca dcab =
    JD.map2 (\l r -> l r) dcab dca

