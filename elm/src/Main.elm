module Main exposing (Model, Msg(..), init, main, navLink, subscriptions, update, view)

import Browser
import Browser.Navigation as Nav
import Css exposing (..)
import Html exposing (Html)
import Html.Styled as Styled exposing (..)
import Html.Styled.Attributes exposing (css, href, src)
import Html.Styled.Events exposing (onClick)
import Url



-- MAIN


main : Program () Model Msg
main =
    Browser.application
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        , onUrlChange = UrlChanged
        , onUrlRequest = LinkClicked
        }



-- MODEL


type alias Model =
    { key : Nav.Key
    , url : Url.Url
    }


init : () -> Url.Url -> Nav.Key -> ( Model, Cmd Msg )
init flags url key =
    ( Model key url, Cmd.none )



-- UPDATE


type Msg
    = NoOp
    | LinkClicked Browser.UrlRequest
    | UrlChanged Url.Url


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        NoOp ->
            ( model, Cmd.none )

        LinkClicked urlRequest ->
            case urlRequest of
                Browser.Internal url ->
                    ( model, Nav.pushUrl model.key (Url.toString url) )

                Browser.External href ->
                    ( model, Nav.load href )

        UrlChanged url ->
            ( { model | url = url }
            , Cmd.none
            )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none



-- VIEW


view : Model -> Browser.Document Msg
view model =
    { title = "Phil Barlow"
    , body =
        List.map toUnstyled
            [ navBar
            ]
    }


navBar : Styled.Html msg
navBar =
    nav [ css [ flexContainer, whiteBackground ] ]
        [ div [ css [ navStyle, flexItem, flexStartStyle ] ] [ text "Phil Barlow" ]
        , ul [ css [ navStyle ], css [ flexContainer, flexEndStyle ] ]
            [ navLink "/books" "Books"
            , navLink "/blog" "Blog"
            , navLink "/contact" "Contact"
            ]
        ]


navLink : String -> String -> Styled.Html msg
navLink path labelText =
    li [] [ a [ href path ] [ text labelText ] ]



-- STYLES


flexContainer : Style
flexContainer =
    Css.batch
        [ displayFlex
        ]


flexItem : Style
flexItem =
    Css.batch
        [ flex (int 1)
        ]


flexStartStyle : Style
flexStartStyle =
    Css.batch
        [ justifyContent flexStart
        ]


flexEndStyle : Style
flexEndStyle =
    Css.batch
        [ justifyContent flexEnd
        ]


whiteBackground : Style
whiteBackground =
    Css.batch
        [ backgroundColor (hex "FFFFFF") ]


navStyle : Style
navStyle =
    Css.batch
        [ listStyle none
        ]


navAStyle : Style
navAStyle =
    Css.batch
        [ textDecoration none
        , display block
        , padding2 (Css.em 0) (Css.em 1)
        ]
