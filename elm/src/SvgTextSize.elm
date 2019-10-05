module SvgTextSize exposing (Metrics, TextSizeReply, TextSizeRequest, calcText, calcTextSvg, computeFontScaling, decodeMetrics, decodeTextSizeReply, encodeTextSizeRequest, estimateTextWidth)

import Json.Decode as JD
import Json.Encode as JE
import Svg exposing (Attribute, Svg, g, rect, svg, text, text_)
import Svg.Attributes exposing (..)
import SvgThings exposing (ControlId, Rect, decodeControlId, encodeControlId)
import Template exposing (render, template, withString, withValue)
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


type alias TextSizeRequest =
    { string : String
    , font : String
    , controlId : ControlId
    }


encodeTextSizeRequest : TextSizeRequest -> JE.Value
encodeTextSizeRequest tsr =
    JE.object
        [ ( "string", JE.string tsr.string )
        , ( "font", JE.string tsr.font )
        , ( "controlId", encodeControlId tsr.controlId )
        ]


type alias TextSizeReply =
    { width : Float
    , controlId : ControlId
    }


decodeTextSizeReply : JD.Decoder TextSizeReply
decodeTextSizeReply =
    JD.succeed TextSizeReply
        |> andMap (JD.field "width" JD.float)
        |> andMap (JD.field "controlId" decodeControlId)


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
--


calcTextSvg : String -> String -> Rect -> List (Svg ())
calcTextSvg fontFam textString rect =
    let
        w =
            estimateTextWidth textString ("20px " ++ fontFam)

        fs =
            computeFontScaling (toFloat w) 20.0 (toFloat rect.w) (toFloat rect.h)
    in
    calcText fontFam textString w fs rect


calcText : String -> String -> Int -> Float -> Rect -> List (Svg ())
calcText fontFam lbtext labelMeasuredWidth fontScaling rect =
    let
        width =
            labelMeasuredWidth

        scale =
            fontScaling

        xc =
            toFloat rect.x + toFloat rect.w / 2

        yc =
            toFloat rect.y + toFloat rect.h / 2

        xt =
            xc - (toFloat width * scale * 0.5)

        yt =
            yc + 20.0 * scale * 0.5

        tmpl =
            template "matrix("
                |> withValue .scale
                |> withString ", 0, 0, "
                |> withValue .scale
                |> withString ", "
                |> withValue .xt
                |> withString ", "
                |> withValue .yt
                |> withString ")"

        -- template "matrix(" <% .scale %> ", 0, 0, " <% .scale %> ", " <% .xt %> ", " <% .yt %> ")"
        xf =
            render { scale = String.fromFloat scale, xt = String.fromFloat xt, yt = String.fromFloat yt } tmpl
    in
    [ text_
        [ fill "black"

        -- , textAnchor "middle"
        -- , x model.middlex
        -- , y fonty
        -- , lengthAdjust "glyphs"
        -- , textLength model.srect.w
        -- , fontSize "20" -- model.srect.h
        , fontSize "20px"
        , fontFamily fontFam
        , transform xf
        , style "cursor: default; -webkit-user-select: none;  -moz-user-select: none;  -ms-user-select: none; user-select: none;"
        ]
        [ text lbtext ]
    ]


computeFontScaling : Float -> Float -> Float -> Float -> Float
computeFontScaling fw fh rw rh =
    let
        wr =
            rw / fw

        hr =
            rh / fh
    in
    if wr < hr then
        wr

    else
        hr
