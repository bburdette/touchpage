module SvgControl exposing (ID, Model(..), Msg(..), Spec(..), SzModel, SzMsg(..), SzSpec, border, controlId, controlName, findControl, firstJust, init, jsCs, jsSpec, jsSzSpec, jsUmType, jsUpdateMessage, mkRlist, myTail, onTextSize, processProps, resize, szFindControl, szOnTextSize, szinit, szresize, szupdate, szview, toCtrlMsg, tupMap2, update, update_list, view, viewSvgControls, zip)

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
import SvgTextSize exposing (TextSizeReply, calcTextSvg, resizeCommand)
import SvgThings exposing (Orientation(..), UiColor(..), UiTheme)
import SvgXY
import Task
import VirtualDom as VD



----------------------------------------------------------
-- Both control container and sizer ard in this file.
-- Normally I'd break them out into separate files, but
-- they are mutually recursive so they have to
-- both be in a single file.
-------------------- control container -------------------


type Spec
    = CsButton SvgButton.Spec
    | CsSlider SvgSlider.Spec
    | CsXY SvgXY.Spec
    | CsLabel SvgLabel.Spec
    | CsSizer SzSpec


type Model
    = CmButton SvgButton.Model
    | CmSlider SvgSlider.Model
    | CmXY SvgXY.Model
    | CmLabel SvgLabel.Model
    | CmSizer SzModel


type Msg
    = CaButton SvgButton.Msg
    | CaSlider SvgSlider.Msg
    | CaXY SvgXY.Msg
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

        CmXY smod ->
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

        CmXY smod ->
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

        CmXY smod ->
            Just smod.name

        CmLabel smod ->
            Just smod.name

        CmSizer szmod ->
            Nothing


tupMap2 : (a -> c) -> ( a, b ) -> ( c, b )
tupMap2 fa ab =
    ( fa (Tuple.first ab), Tuple.second ab )


resize : Model -> SvgThings.Rect -> ( Model, Command )
resize model rect =
    let
        aptg =
            \f ( m, c ) -> ( f m, c )
    in
    case model of
        CmButton mod ->
            aptg CmButton <| SvgButton.resize mod (SvgThings.shrinkRect border rect)

        CmSlider mod ->
            aptg CmSlider <| SvgSlider.resize mod (SvgThings.shrinkRect border rect)

        CmXY mod ->
            aptg CmXY <| SvgXY.resize mod (SvgThings.shrinkRect border rect)

        CmLabel mod ->
            aptg CmLabel <| SvgLabel.resize mod (SvgThings.shrinkRect border rect)

        CmSizer mod ->
            aptg CmSizer <| szresize mod rect


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

        "xy" ->
            SvgXY.jsSpec |> JD.andThen (\a -> JD.succeed (CsXY a))

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

        "xy" ->
            SvgXY.jsUpdateMessage
                |> JD.andThen
                    (\x -> JD.succeed (toCtrlMsg x.controlId (CaXY (SvgXY.SvgUpdate x))))

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


onTextSize : UiTheme -> TextSizeReply -> Model -> Model
onTextSize theme tsr model =
    case model of
        CmButton m ->
            CmButton <|
                SvgTextSize.onTextSizeReply theme tsr m

        CmSlider m ->
            CmSlider <|
                SvgTextSize.onTextSizeReply theme tsr m

        CmXY m ->
            CmXY <|
                SvgTextSize.onTextSizeReply theme tsr m

        CmLabel m ->
            CmLabel <| SvgTextSize.onTextSizeReply theme tsr m

        CmSizer m ->
            CmSizer <| szOnTextSize theme tsr m


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

        ( CaXY ms, CmXY m ) ->
            let
                ( a, b ) =
                    SvgXY.update ms m
            in
            ( CmXY a, b )

        ( CaLabel ms, CmLabel m ) ->
            let
                ( md, c ) =
                    SvgLabel.update ms m
            in
            ( CmLabel md, c )

        ( CaSizer ms, CmSizer m ) ->
            let
                ( a, b ) =
                    szupdate ms m
            in
            ( CmSizer a, b )

        _ ->
            ( model, None )



-- should probably produce an error.  to the user??


update_list : List Msg -> Model -> ( Model, List Command )
update_list msgs model =
    List.foldl
        (\msg ( mod, cmds ) ->
            let
                ( modnew, cmd ) =
                    update msg mod
            in
            ( modnew, cmd :: cmds )
        )
        ( model, [] )
        msgs


init :
    SvgThings.Rect
    -> SvgThings.ControlId
    -> Spec
    -> ( Model, Command )
init rect cid spec =
    let
        aptg =
            \f ( m, c ) -> ( f m, c )
    in
    case spec of
        CsButton s ->
            aptg CmButton <| SvgButton.init (SvgThings.shrinkRect border rect) cid s

        CsSlider s ->
            aptg CmSlider <| SvgSlider.init (SvgThings.shrinkRect border rect) cid s

        CsXY s ->
            aptg CmXY <| SvgXY.init (SvgThings.shrinkRect border rect) cid s

        CsLabel s ->
            aptg CmLabel <| SvgLabel.init (SvgThings.shrinkRect border rect) cid s

        CsSizer s ->
            aptg CmSizer <| szinit rect cid s


view : UiTheme -> Model -> Svg Msg
view theme model =
    case model of
        CmButton m ->
            VD.map CaButton (SvgButton.view theme m)

        CmSlider m ->
            VD.map CaSlider (SvgSlider.view theme m)

        CmXY m ->
            VD.map CaXY (SvgXY.view theme m)

        CmLabel m ->
            VD.map CaLabel (SvgLabel.view theme m)

        CmSizer m ->
            VD.map CaSizer (szview m theme)



-------------------- sizer -------------------


{-| json spec
-}
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


szOnTextSize : UiTheme -> TextSizeReply -> SzModel -> SzModel
szOnTextSize theme tsr model =
    case tsr.controlId of
        idx :: rst ->
            case Dict.get idx model.controls of
                Just control ->
                    let
                        nc =
                            onTextSize theme { tsr | controlId = rst } control
                    in
                    { model | controls = Dict.insert idx nc model.controls }

                Nothing ->
                    model

        [] ->
            model


szresize : SzModel -> SvgThings.Rect -> ( SzModel, Command )
szresize model rect =
    let
        clist =
            Dict.toList model.controls

        rlist =
            mkRlist model.orientation rect (List.length clist) model.proportions

        rlist2 =
            List.map (\( ( i, c ), r ) -> ( i, resize c r )) (zip clist rlist)

        controls =
            List.map (\( i, ( m, c ) ) -> ( i, m )) rlist2

        cmds =
            List.map (\( i, ( m, c ) ) -> c) rlist2

        cdict =
            Dict.fromList controls

        nm =
            { model | rect = rect, controls = cdict }
    in
    ( nm, Batch cmds )


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
    -> ( SzModel, Command )
szinit rect cid szspec =
    let
        rlist =
            mkRlist szspec.orientation rect (List.length szspec.controls) szspec.proportions

        blist =
            List.map
                (\( spec, rect_, idx ) -> init rect_ (cid ++ [ idx ]) spec)
                (map3 (\a b c -> ( a, b, c )) szspec.controls rlist idxs)

        mods =
            List.map Tuple.first blist

        cmds =
            List.map Tuple.second blist

        idxs =
            List.range 0 (length szspec.controls)

        controlz =
            zip idxs mods

        model =
            SzModel cid rect (Dict.fromList controlz) szspec.orientation szspec.proportions
    in
    ( model, Batch cmds )



-- VIEW


szview : SzModel -> UiTheme -> Svg SzMsg
szview model theme =
    let
        controllst =
            Dict.toList model.controls
    in
    Svg.g [] (List.map (viewSvgControls theme) controllst)


viewSvgControls : UiTheme -> ( ID, Model ) -> Svg.Svg SzMsg
viewSvgControls theme ( id, model ) =
    VD.map (SzCMsg id) (view theme model)


border : Int
border =
    1
