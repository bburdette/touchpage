module SvgTextSize exposing (Metrics, decodeMetrics, estimateTextWidth)

import Json.Decode as JD
import Util exposing (andMap)


estimateTextWidth : String -> String -> Int
estimateTextWidth text font =
    String.length text * 12



{- TextMetrics.width Read only
       Is a double giving the calculated width of a segment of inline text in CSS pixels. It takes into account the current font of the context.
   TextMetrics.actualBoundingBoxLeft Read only
       Is a double giving the distance parallel to the baseline from the alignment point given by the CanvasRenderingContext2D.textAlign property to the left side of the bounding rectangle of the given text, in CSS pixels.
   TextMetrics.actualBoundingBoxRight Read only
       Is a double giving the distance parallel to the baseline from the alignment point given by the CanvasRenderingContext2D.textAlign property to the right side of the bounding rectangle of the given text, in CSS pixels.
   TextMetrics.fontBoundingBoxAscent Read only
       Is a double giving the distance from the horizontal line indicated by the CanvasRenderingContext2D.textBaseline attribute to the top of the highest bounding rectangle of all the fonts used to render the text, in CSS pixels.
   TextMetrics.fontBoundingBoxDescent Read only
       Is a double giving the distance from the horizontal line indicated by the CanvasRenderingContext2D.textBaseline attribute to the bottom of the bounding rectangle of all the fonts used to render the text, in CSS pixels.
   TextMetrics.actualBoundingBoxAscent Read only
       Is a double giving the distance from the horizontal line indicated by the CanvasRenderingContext2D.textBaseline attribute to the top of the bounding rectangle used to render the text, in CSS pixels.
   TextMetrics.actualBoundingBoxDescent Read only
       Is a double giving the distance from the horizontal line indicated by the CanvasRenderingContext2D.textBaseline attribute to the bottom of the bounding rectangle used to render the text, in CSS pixels.
   TextMetrics.emHeightAscent Read only
       Is a double giving the distance from the horizontal line indicated by the CanvasRenderingContext2D.textBaseline property to the top of the em square in the line box, in CSS pixels.
   TextMetrics.emHeightDescent Read only
       Is a double giving the distance from the horizontal line indicated by the CanvasRenderingContext2D.textBaseline property to the bottom of the em square in the line box, in CSS pixels.
   TextMetrics.hangingBaseline Read only
       Is a double giving the distance from the horizontal line indicated by the CanvasRenderingContext2D.textBaseline property to the hanging baseline of the line box, in CSS pixels.
   TextMetrics.alphabeticBaseline Read only
       Is a double giving the distance from the horizontal line indicated by the CanvasRenderingContext2D.textBaseline property to the alphabetic baseline of the line box, in CSS pixels.
   TextMetrics.ideographicBaseline Read only
       Is a double giving the distance from the horizontal line indicated by the CanvasRenderingContext2D.textBaseline property to the ideographic baseline of the line box, in CSS pixels."sans-serif"
-}


type alias Metrics =
    { width : Float
    , actualBoundingBoxLeft : Float
    , actualBoundingBoxRight : Float
    , fontBoundingBoxAscent : Float
    , fontBoundingBoxDescent : Float
    , actualBoundingBoxAscent : Float
    , actualBoundingBoxDescent : Float
    , emHeightAscent : Float
    , emHeightDescent : Float
    , hangingBaseline : Float
    , alphabeticBaseline : Float
    , ideographicBaseline : Float
    }


decodeMetrics : JD.Decoder Metrics
decodeMetrics =
    JD.succeed Metrics
        |> andMap (JD.field "width" JD.float)
        |> andMap (JD.field "actualBoundingBoxLeft" JD.float)
        |> andMap (JD.field "actualBoundingBoxRight" JD.float)
        |> andMap (JD.field "fontBoundingBoxAscent" JD.float)
        |> andMap (JD.field "fontBoundingBoxDescent" JD.float)
        |> andMap (JD.field "actualBoundingBoxAscent" JD.float)
        |> andMap (JD.field "actualBoundingBoxDescent" JD.float)
        |> andMap (JD.field "emHeightAscent" JD.float)
        |> andMap (JD.field "emHeightDescent" JD.float)
        |> andMap (JD.field "hangingBaseline" JD.float)
        |> andMap (JD.field "alphabeticBaseline" JD.float)
        |> andMap (JD.field "ideographicBaseline" JD.float)
