module Part1 exposing (..)

import Html exposing (Html, div, h1, p, text)
import Input


hash : String -> Int
hash str =
    str
        |> String.foldl (\c acc -> modBy 256 (acc + Char.toCode c) * 17) 0
        |> modBy 256


hashSum : String -> Int
hashSum str =
    str |> String.split "," |> List.map hash |> List.sum


main : Html msg
main =
    let
        answer =
            hashSum Input.input
    in
    div []
        [ h1 []
            [ text "Part 1" ]
        , p
            []
            [ answer |> String.fromInt |> text ]
        ]
