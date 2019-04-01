module Main exposing (Model, Msg(..), init, main, subscriptions, update, view, navLink)

import Browser
import Browser.Navigation as Nav
import Html exposing (..)
import Html.Attributes exposing (..)
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
        [ navBar
        ]
    }

navBar : Html msg
navBar = 
    nav [ class "flexContainer", class "whiteBackground"] 
        [ div [class "nav", class "flexItem", class "flexStart"] [text "Phil Barlow"]
        , ul [class "nav", class "flexContainer", class "flexEnd"] 
            [ navLink "/books" "Books"
            , navLink "/blog" "Blog"
            , navLink "/contact" "Contact"
            ]
        ]

navLink : String -> String -> Html msg
navLink path labelText =
    li [] [ a [ href path ] [ text labelText ] ]
