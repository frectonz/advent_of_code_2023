module Part1 exposing (..)

import Html exposing (Html, div, h1, p, text)
import Input
import Parser exposing ((|.), (|=), Parser, Trailing(..), int, keyword, oneOf, spaces, succeed, symbol)


type alias Game =
    { id : Int
    , sets : List CubeSet
    }


type alias CubeSet =
    List Cube


type Cube
    = Red Int
    | Green Int
    | Blue Int


type alias Constraint =
    { red : Int, green : Int, blue : Int }


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


game : Parser Game
game =
    succeed Game
        |. keyword "Game"
        |. spaces
        |= int
        |. symbol ":"
        |= cubeSets


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


parseInput : String -> List Game
parseInput input =
    input
        |> String.lines
        |> List.map (Parser.run game)
        |> List.map (Result.withDefault (Game -1 []))


possibleGames : Constraint -> List Game -> List Game
possibleGames constraint games =
    games
        |> List.filter
            (possibleCubeSets constraint)


possibleCubeSets : Constraint -> Game -> Bool
possibleCubeSets constraint g =
    g.sets
        |> List.all
            (\set ->
                (countCubes getRed set <= constraint.red)
                    && (countCubes getBlue set <= constraint.blue)
                    && (countCubes getGreen set <= constraint.green)
            )


countCubes : (Cube -> Int) -> CubeSet -> Int
countCubes getter set =
    set
        |> List.map getter
        |> List.sum


main : Html msg
main =
    let
        games =
            parseInput Input.input

        possible =
            possibleGames { red = 12, green = 13, blue = 14 } games

        sum =
            possible
                |> List.map (\g -> g.id)
                |> List.sum
    in
    div []
        [ h1 [] [ text "Part 1" ]
        , p [] [ text ("Answer " ++ String.fromInt sum) ]
        , p [] [ text ("Parsed " ++ (games |> List.length |> String.fromInt) ++ " games") ]
        , p [] [ text ((possible |> List.length |> String.fromInt) ++ " possible games") ]
        , p [] [ text ("The possible games were " ++ (possible |> List.map (\g -> String.fromInt g.id) |> String.join ", ")) ]
        ]
