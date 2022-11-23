module Main exposing (..)

import Browser
import Html exposing (h1, text)

main = Browser.sandbox { init = (), update = update, view = view }

update _ _ = ()

view _ = h1 [] [text "Hello World!"]
