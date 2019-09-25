module SvgControl exposing (ID, Model(..), Msg(..), Spec(..), SzModel, SzMsg(..), SzSpec, border, controlId, controlName, findControl, firstJust, init, jsCs, jsSpec, jsSzSpec, jsUmType, jsUpdateMessage, mkRlist, myTail, processProps, resize, szFindControl, szinit, szresize, szupdate, szview, toCtrlMsg, tupMap2, update, view, viewSvgControls, zip)

import Dict exposing (..)
import Html
import Json.Decode as JD
import List exposing (..)
import Svg exposing (Svg)
import Svg.Attributes as SA
import SvgButton
import SvgCommand exposing (Command(..))
import SvgLabel
import SvgSlider
import SvgThings
import Task
import VirtualDom as VD



----------------------------------------------------------
-- Two things (objects?) in this file; control container
-- and sizer.  They are mutually recursive so they have to
-- both be in a single file.


border : Int
border =
    1



-------------------- control container -------------------


type Spec
    = CsButton SvgButton.Spec
    | CsSlider SvgSlider.Spec
    | CsLabel SvgLabel.Spec
    | CsSizer SzSpec


type Model
    = CmButton SvgButton.Model
    | CmSlider SvgSlider.Model
    | CmLabel SvgLabel.Model
    | CmSizer SzModel


type Msg
    = CaButton SvgButton.Msg
    | CaSlider SvgSlider.Msg
    | CaLabel SvgLabel.Msg
    | CaSizer SzMsg


findControl : Int -> Int -> Model -> Maybe Model
findControl x y mod =
    case mod of
        CmButton bmod ->
            if SvgThings.containsXY bmod.rect x y then
                Just mod

            else
                Nothing

        CmSlider smod ->
            if SvgThings.containsXY smod.rect x y then
                Just mod

            else
                Nothing

        CmLabel smod ->
            Nothing

        CmSizer szmod ->
            szFindControl szmod x y


controlId : Model -> SvgThings.ControlId
controlId mod =
    case mod of
        CmButton bmod ->
            bmod.cid

        CmSlider smod ->
            smod.cid

        CmLabel smod ->
            smod.cid

        CmSizer szmod ->
            szmod.cid


controlName : Model -> Maybe String
controlName mod =
    case mod of
        CmButton bmod ->
            Just bmod.name

        CmSlider smod ->
            Just smod.name

        CmLabel smod ->
            Just smod.name

        CmSizer szmod ->
            Nothing


tupMap2 : (a -> c) -> ( a, b ) -> ( c, b )
tupMap2 fa ab =
    ( fa (Tuple.first ab), Tuple.second ab )


resize : Model -> SvgThings.Rect -> Model
resize model rect =
    case model of
        CmButton mod ->
            CmButton <| SvgButton.resize mod (SvgThings.shrinkRect border rect)

        CmSlider mod ->
            CmSlider <| SvgSlider.resize mod (SvgThings.shrinkRect border rect)

        CmLabel mod ->
            CmLabel <| SvgLabel.resize mod (SvgThings.shrinkRect border rect)

        CmSizer mod ->
            CmSizer <| szresize mod rect


jsSpec : JD.Decoder Spec
jsSpec =
    JD.field "type" JD.string |> JD.andThen jsCs


jsCs : String -> JD.Decoder Spec
jsCs t =
    case t of
        "button" ->
            SvgButton.jsSpec |> JD.andThen (\a -> JD.succeed (CsButton a))

        "slider" ->
            SvgSlider.jsSpec |> JD.andThen (\a -> JD.succeed (CsSlider a))

        "label" ->
            SvgLabel.jsSpec |> JD.andThen (\a -> JD.succeed (CsLabel a))

        "sizer" ->
            jsSzSpec |> JD.andThen (\a -> JD.succeed (CsSizer a))

        _ ->
            JD.fail ("unkown type: " ++ t)


jsUpdateMessage : JD.Decoder Msg
jsUpdateMessage =
    JD.field "controlType" JD.string |> JD.andThen jsUmType


jsUmType : String -> JD.Decoder Msg
jsUmType wat =
    case wat of
        "button" ->
            SvgButton.jsUpdateMessage
                |> JD.andThen
                    (\x -> JD.succeed (toCtrlMsg x.controlId (CaButton (SvgButton.SvgUpdate x))))

        "slider" ->
            SvgSlider.jsUpdateMessage
                |> JD.andThen
                    (\x -> JD.succeed (toCtrlMsg x.controlId (CaSlider (SvgSlider.SvgUpdate x))))

        "label" ->
            SvgLabel.jsUpdateMessage
                |> JD.andThen
                    (\x -> JD.succeed (toCtrlMsg x.controlId (CaLabel (SvgLabel.SvgUpdate x))))

        _ ->
            JD.fail ("unknown update type" ++ wat)


myTail : List a -> List a
myTail lst =
    let
        tl =
            tail lst
    in
    case tl of
        Just l ->
            l

        Nothing ->
            []


toCtrlMsg : SvgThings.ControlId -> Msg -> Msg
toCtrlMsg id msg =
    case head id of
        Nothing ->
            msg

        Just x ->
            CaSizer (SzCMsg x (toCtrlMsg (myTail id) msg))


update : Msg -> Model -> ( Model, Command )
update msg model =
    case ( msg, model ) of
        ( CaButton ms, CmButton m ) ->
            let
                ( a, b ) =
                    SvgButton.update ms m
            in
            ( CmButton a, b )

        ( CaSlider ms, CmSlider m ) ->
            let
                ( a, b ) =
                    SvgSlider.update ms m
            in
            ( CmSlider a, b )

        ( CaLabel ms, CmLabel m ) ->
            ( CmLabel <| SvgLabel.update ms m, None )

        ( CaSizer ms, CmSizer m ) ->
            let
                ( a, b ) =
                    szupdate ms m
            in
            ( CmSizer a, b )

        _ ->
            ( model, None )



-- should probably produce an error.  to the user??
{- update_list : List Msg -> Model -> ( Model, Command )
   update_list msgs model =
       List.foldl
           (\msg ( mod, c ) ->
               let
                   ( modnew, cmd ) =
                       update msg mod
               in
               ( modnew, Cmd.batch [ c, cmd ] )
           )
           ( model, Cmd.none )
           msgs


-}


init :
    SvgThings.Rect
    -> SvgThings.ControlId
    -> Spec
    -> Model
init rect cid spec =
    case spec of
        CsButton s ->
            CmButton <| SvgButton.init (SvgThings.shrinkRect border rect) cid s

        CsSlider s ->
            CmSlider <| SvgSlider.init (SvgThings.shrinkRect border rect) cid s

        CsLabel s ->
            CmLabel <| SvgLabel.init (SvgThings.shrinkRect border rect) cid s

        CsSizer s ->
            CmSizer <| szinit rect cid s


view : Model -> Svg Msg
view model =
    case model of
        CmButton m ->
            VD.map CaButton (SvgButton.view m)

        CmSlider m ->
            VD.map CaSlider (SvgSlider.view m)

        CmLabel m ->
            VD.map CaLabel (SvgLabel.view m)

        CmSizer m ->
            VD.map CaSizer (szview m)



-------------------- sizer -------------------
-- json spec


type alias SzSpec =
    { orientation : SvgThings.Orientation
    , proportions : Maybe (List Float)
    , controls : List Spec
    }



-- proportions should all add up to 1.0


processProps : List Float -> List Float
processProps lst =
    let
        s =
            sum lst
    in
    List.map (\x -> x / s) lst


jsSzSpec : JD.Decoder SzSpec
jsSzSpec =
    JD.map3 SzSpec
        (JD.field "orientation" JD.string |> JD.andThen SvgThings.jsOrientation)
        (JD.maybe (JD.field "proportions" (JD.list JD.float))
            |> JD.andThen
                (\x -> JD.succeed (Maybe.map processProps x))
        )
        (JD.field "controls" (JD.list (JD.lazy (\_ -> jsSpec))))


type alias SzModel =
    { cid : SvgThings.ControlId
    , rect : SvgThings.Rect
    , controls : Dict ID Model
    , orientation : SvgThings.Orientation
    , proportions : Maybe (List Float)
    }


type alias ID =
    Int


szFindControl : SzModel -> Int -> Int -> Maybe Model
szFindControl mod x y =
    if SvgThings.containsXY mod.rect x y then
        firstJust (findControl x y) (values mod.controls)

    else
        Nothing


firstJust : (a -> Maybe b) -> List a -> Maybe b
firstJust f xs =
    case head xs of
        Nothing ->
            Nothing

        Just x ->
            case f x of
                Just v ->
                    Just v

                Nothing ->
                    Maybe.andThen (firstJust f) (tail xs)



-- UPDATE


type SzMsg
    = SzCMsg ID Msg


zip =
    List.map2 Tuple.pair


szupdate : SzMsg -> SzModel -> ( SzModel, Command )
szupdate msg model =
    case msg of
        SzCMsg id act ->
            let
                bb =
                    get id model.controls
            in
            case bb of
                Just bm ->
                    let
                        wha =
                            update act bm

                        updcontrols =
                            insert id (Tuple.first wha) model.controls

                        newmod =
                            { model | controls = updcontrols }
                    in
                    ( newmod, Tuple.second wha )

                Nothing ->
                    ( model, None )


szresize : SzModel -> SvgThings.Rect -> SzModel
szresize model rect =
    let
        clist =
            Dict.toList model.controls

        rlist =
            mkRlist model.orientation rect (List.length clist) model.proportions

        controls =
            List.map (\( ( i, c ), r ) -> ( i, resize c r )) (zip clist rlist)

        cdict =
            Dict.fromList controls
    in
    { model | rect = rect, controls = cdict }


mkRlist : SvgThings.Orientation -> SvgThings.Rect -> Int -> Maybe (List Float) -> List SvgThings.Rect
mkRlist orientation rect count mbproportions =
    case orientation of
        SvgThings.Horizontal ->
            case mbproportions of
                Nothing ->
                    SvgThings.hrects rect count

                Just p ->
                    SvgThings.hrectsp rect count p

        SvgThings.Vertical ->
            case mbproportions of
                Nothing ->
                    SvgThings.vrects rect count

                Just p ->
                    SvgThings.vrectsp rect count p


szinit :
    SvgThings.Rect
    -> SvgThings.ControlId
    -> SzSpec
    -> SzModel
szinit rect cid szspec =
    let
        rlist =
            mkRlist szspec.orientation rect (List.length szspec.controls) szspec.proportions

        blist =
            List.map
                (\( spec, rect_, idx ) -> init rect_ (cid ++ [ idx ]) spec)
                (map3 (\a b c -> ( a, b, c )) szspec.controls rlist idxs)

        idxs =
            List.range 0 (length szspec.controls)

        controlz =
            zip idxs blist
    in
    SzModel cid rect (Dict.fromList controlz) szspec.orientation szspec.proportions



-- VIEW


szview : SzModel -> Svg SzMsg
szview model =
    let
        controllst =
            Dict.toList model.controls
    in
    Svg.g [] (List.map viewSvgControls controllst)


viewSvgControls : ( ID, Model ) -> Svg.Svg SzMsg
viewSvgControls ( id, model ) =
    VD.map (SzCMsg id) (view model)
