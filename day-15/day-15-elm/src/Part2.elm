module Part2 exposing (..)

import Dict exposing (Dict)
import Html exposing (Html, div, h1, p, text)
import Input
import Parser exposing ((|.), (|=), Parser, chompUntil, getChompedString, oneOf, succeed, symbol)


type alias Step =
    { label : String
    , operation : Operation
    }


type Operation
    = Add Int
    | Remove


label : Parser String
label =
    getChompedString <|
        succeed ()
            |. oneOf [ chompUntil "=", chompUntil "-" ]


step : Parser Step
step =
    succeed Step
        |= label
        |= oneOf
            [ symbol "-" |> Parser.map (\_ -> Remove)
            , succeed Add
                |. symbol "="
                |= Parser.int
            ]


parseSteps : String -> List Step
parseSteps input =
    input
        |> String.split ","
        |> List.map (Parser.run step)
        |> List.filterMap Result.toMaybe


type alias HashMap =
    Dict Int (List ( String, Int ))


hashmap : List Step -> HashMap
hashmap steps =
    steps
        |> List.foldl
            (\s map ->
                let
                    box =
                        hash s.label
                in
                case s.operation of
                    Remove ->
                        map
                            |> Dict.update box
                                (\b ->
                                    b |> Maybe.map (List.filter (\( lens, _ ) -> s.label /= lens))
                                )

                    Add focal_l ->
                        map
                            |> Dict.update box
                                (\b ->
                                    b
                                        |> Maybe.map
                                            (List.map
                                                (\( lens, f ) ->
                                                    if s.label == lens then
                                                        ( s.label, focal_l )

                                                    else
                                                        ( lens, f )
                                                )
                                            )
                                        |> Maybe.map
                                            (\b_ ->
                                                if List.member ( s.label, focal_l ) b_ then
                                                    b_

                                                else
                                                    List.append b_ (List.singleton ( s.label, focal_l ))
                                            )
                                        |> Maybe.withDefault (List.singleton ( s.label, focal_l ))
                                        |> Just
                                )
            )
            Dict.empty


focusingPower : HashMap -> Int
focusingPower map =
    map
        |> Dict.foldl
            (\box lenses acc ->
                acc
                    + (lenses
                        |> List.indexedMap (\i ( _, focal_l ) -> (box + 1) * (i + 1) * focal_l)
                        |> List.sum
                      )
            )
            0


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
            parseSteps Input.input |> hashmap |> focusingPower
    in
    div []
        [ h1 []
            [ text "Part 2" ]
        , p
            []
            [ answer |> String.fromInt |> text ]
        ]
