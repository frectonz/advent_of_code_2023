module Part2 exposing (..)

import Html exposing (Html, div, h1, p, text)
import Input
import Parser exposing ((|.), (|=), Parser, Trailing(..), int, keyword, oneOf, spaces, succeed, symbol)


type alias Game =
    { id : Int
    , sets : List CubeSet
    }


type alias FewestCubes =
    { id : Int
    , red : Int
    , blue : Int
    , green : Int
    }


type Cube
    = Red Int
    | Green Int
    | Blue Int


getRed : Cube -> Int
getRed c =
    case c of
        Red x ->
            x

        Blue _ ->
            0

        Green _ ->
            0


getBlue : Cube -> Int
getBlue c =
    case c of
        Red _ ->
            0

        Blue x ->
            x

        Green _ ->
            0


getGreen : Cube -> Int
getGreen c =
    case c of
        Red _ ->
            0

        Blue _ ->
            0

        Green x ->
            x


type alias CubeSet =
    List Cube


type alias Constraint =
    { red : Int, green : Int, blue : Int }


cube : Parser Cube
cube =
    succeed (\num cubeType -> cubeType num)
        |. spaces
        |= int
        |. spaces
        |= oneOf
            [ Parser.map (\_ -> Red) (keyword "red")
            , Parser.map (\_ -> Blue) (keyword "blue")
            , Parser.map (\_ -> Green) (keyword "green")
            ]


cubeSet : Parser CubeSet
cubeSet =
    Parser.sequence
        { start = ""
        , separator = ","
        , end = ""
        , spaces = spaces
        , item = cube
        , trailing = Forbidden
        }


cubeSets : Parser (List CubeSet)
cubeSets =
    Parser.sequence
        { start = ""
        , separator = ";"
        , end = ""
        , spaces = spaces
        , item = cubeSet
        , trailing = Forbidden
        }


game : Parser Game
game =
    succeed Game
        |. keyword "Game"
        |. spaces
        |= int
        |. symbol ":"
        |= cubeSets


parseInput : String -> List Game
parseInput input =
    input
        |> String.lines
        |> List.map (Parser.run game)
        |> List.map (Result.withDefault (Game -1 []))


fewestCubes : List Game -> List FewestCubes
fewestCubes games =
    games |> List.map neededCubes


neededCubes : Game -> FewestCubes
neededCubes g =
    g.sets
        |> List.foldl
            (\set acc ->
                let
                    reds =
                        List.foldl (\c red_acc -> max (getRed c) red_acc) acc.red set

                    blues =
                        List.foldl (\c blue_acc -> max (getBlue c) blue_acc) acc.blue set

                    greens =
                        List.foldl (\c green_acc -> max (getGreen c) green_acc) acc.green set
                in
                FewestCubes acc.id reds blues greens
            )
            (FewestCubes g.id 0 0 0)


main : Html msg
main =
    let
        games =
            parseInput Input.input

        cubes =
            fewestCubes games

        powers =
            cubes
                |> List.map (\fewest -> fewest.red * fewest.blue * fewest.green)

        sum =
            List.sum powers
    in
    div []
        [ h1 [] [ text "Part 2" ]
        , p [] [ text ("Answer " ++ String.fromInt sum) ]
        , p [] [ text ("Parsed " ++ (games |> List.length |> String.fromInt) ++ " games") ]
        , p [] [ text ("Powers " ++ (powers |> List.map String.fromInt |> String.join ", ")) ]
        ]
