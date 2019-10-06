module SvgThings exposing (ControlId, Orientation(..), Rect, SRect, containsXY, decodeControlId, encodeControlId, hrects, hrectsp, jsOrientation, mekhr, mekhrp, mekvr, mekvrp, processProps, shrinkRect, somme, toSRect, vrects, vrectsp)

-- import SvgTextSize

import Json.Decode as JD
import Json.Encode as JE
import List exposing (..)
import Svg exposing (Attribute, Svg, g, rect, svg, text, text_)
import Svg.Attributes exposing (..)
import Template exposing (render, template, withString, withValue)
import Tuple



-- font family


type Orientation
    = Vertical
    | Horizontal


jsOrientation : String -> JD.Decoder Orientation
jsOrientation o =
    case o of
        "vertical" ->
            JD.succeed Vertical

        "horizontal" ->
            JD.succeed Horizontal

        _ ->
            JD.succeed Horizontal


type alias ControlId =
    List Int


encodeControlId : ControlId -> JE.Value
encodeControlId cid =
    JE.list JE.int cid


decodeControlId : JD.Decoder (List Int)
decodeControlId =
    JD.list JD.int


type alias Rect =
    { x : Int
    , y : Int
    , w : Int
    , h : Int
    }


containsXY : Rect -> Int -> Int -> Bool
containsXY rect x y =
    rect.x
        <= x
        && rect.w
        >= (x - rect.x)
        && rect.y
        <= y
        && rect.h
        >= (y - rect.y)


type alias SRect =
    { x : String
    , y : String
    , w : String
    , h : String
    }


toSRect : Rect -> SRect
toSRect rect =
    SRect
        (String.fromInt rect.x)
        (String.fromInt rect.y)
        (String.fromInt rect.w)
        (String.fromInt rect.h)


shrinkRect : Int -> Rect -> Rect
shrinkRect border rect =
    Rect (rect.x + border)
        (rect.y + border)
        (rect.w - border - border)
        (rect.h - border - border)


{-| make a number of horizontally evenly spaced rects.
-}
hrects : Rect -> Int -> List Rect
hrects rct count =
    let
        w : Int
        w =
            round (toFloat rct.w / toFloat count)

        idxs =
            List.range 0 (count - 1)
    in
    map (mekhr rct w) idxs


mekhr : Rect -> Int -> Int -> Rect
mekhr br w i =
    Rect (br.x + (w * i)) br.y w br.h


{-| make a number of horizontally proportionally sized rects.
-}
hrectsp : Rect -> Int -> List Float -> List Rect
hrectsp rct count props =
    let
        pprops =
            processProps count props

        fw =
            toFloat rct.w

        widths =
            map (\p -> round (p * fw)) pprops

        xes =
            somme rct.x widths
    in
    map (mekhrp rct) (map2 Tuple.pair xes widths)


mekhrp : Rect -> ( Int, Int ) -> Rect
mekhrp prect ( x, w ) =
    Rect x prect.y w prect.h


{-| make a number of vertically evenly spaced rects.
-}
vrectsp : Rect -> Int -> List Float -> List Rect
vrectsp rct count props =
    let
        pprops =
            processProps count props

        fh =
            toFloat rct.h

        heights =
            map (\p -> round (p * fh)) pprops

        yes =
            somme rct.y heights
    in
    map (mekvrp rct) (map2 Tuple.pair yes heights)


mekvrp : Rect -> ( Int, Int ) -> Rect
mekvrp prect ( y, h ) =
    Rect prect.x y prect.w h


{-| given a list [a,b,c,d,e], produce the sum list:
[0, a, a+b, a+b+c, etc]
-}
somme : Int -> List Int -> List Int
somme f lst =
    case head lst of
        Nothing ->
            lst

        Just hf ->
            let
                s =
                    f + hf

                tl =
                    tail lst
            in
            case tl of
                Nothing ->
                    [ s ]

                Just t ->
                    f :: somme s t


{-| make a number of vertically evenly spaced rects.
-}
vrects : Rect -> Int -> List Rect
vrects rct count =
    let
        h : Int
        h =
            round (toFloat rct.h / toFloat count)

        idxs =
            List.range 0 (count - 1)
    in
    map (mekvr rct h) idxs


mekvr : Rect -> Int -> Int -> Rect
mekvr br h i =
    Rect br.x (br.y + (h * i)) br.w h


processProps : Int -> List Float -> List Float
processProps controlcount lst =
    let
        l =
            length lst

        r =
            if controlcount > l then
                controlcount - l

            else
                0
    in
    let
        nwlst =
            append (take controlcount lst) (repeat r 0.0)

        s =
            sum nwlst
    in
    List.map (\x -> x / s) nwlst
