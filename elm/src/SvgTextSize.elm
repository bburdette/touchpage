module SvgTextSize exposing (getTextWidth)

-- import Native.SvgTextSize


getTextWidth : String -> String -> Int
getTextWidth text font =
    String.length text * 12



-- Native.SvgTextSize.getTextWidth text font
