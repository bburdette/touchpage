module SvgTextSize exposing (Metrics, decodeMetrics, estimateTextWidth)

import Json.Decode as JD
import Util exposing (andMap)


estimateTextWidth : String -> String -> Int
estimateTextWidth text font =
    String.length text * 12


type alias Metrics =
    { width : Float

    -- None of this crap is actually implemented in the browser, unless you're in safari.
    -- Left here as a warning!
    -- , actualBoundingBoxLeft : Float
    -- , actualBoundingBoxRight : Float
    -- , fontBoundingBoxAscent : Float
    -- , fontBoundingBoxDescent : Float
    -- , actualBoundingBoxAscent : Float
    -- , actualBoundingBoxDescent : Float
    -- , emHeightAscent : Float
    -- , emHeightDescent : Float
    -- , hangingBaseline : Float
    -- , alphabeticBaseline : Float
    -- , ideographicBaseline : Float
    }


decodeMetrics : JD.Decoder Metrics
decodeMetrics =
    JD.succeed Metrics
        |> andMap (JD.field "width" JD.float)



-- |> andMap (JD.field "actualBoundingBoxLeft" JD.float)
-- |> andMap (JD.field "actualBoundingBoxRight" JD.float)
-- |> andMap (JD.field "fontBoundingBoxAscent" JD.float)
-- |> andMap (JD.field "fontBoundingBoxDescent" JD.float)
-- |> andMap (JD.field "actualBoundingBoxAscent" JD.float)
-- |> andMap (JD.field "actualBoundingBoxDescent" JD.float)
-- |> andMap (JD.field "emHeightAscent" JD.float)
-- |> andMap (JD.field "emHeightDescent" JD.float)
-- |> andMap (JD.field "hangingBaseline" JD.float)
-- |> andMap (JD.field "alphabeticBaseline" JD.float)
-- |> andMap (JD.field "ideographicBaseline" JD.float)
