[@@@warnerror "-unused-value-declaration"]

module String = struct
  include String

  let lines = String.split_on_char '\n'
end

module List = struct
  include List

  let sum = List.fold_left ( + ) 0

  let counts arr =
    List.fold_left
      (fun acc v ->
        if List.exists (fun (key, _) -> key = v) acc then
          acc
          |> List.map (fun (key, c) ->
                 if key = v then (key, c + 1) else (key, c))
        else (v, 1) :: acc)
      [] arr
end

module Card = struct
  type t = A | K | Q | J | T | N9 | N8 | N7 | N6 | N5 | N4 | N3 | N2

  exception UnknownCard

  let parse = function
    | 'A' -> A
    | 'K' -> K
    | 'Q' -> Q
    | 'J' -> J
    | 'T' -> T
    | '9' -> N9
    | '8' -> N8
    | '7' -> N7
    | '6' -> N6
    | '5' -> N5
    | '4' -> N4
    | '3' -> N3
    | '2' -> N2
    | _ -> raise UnknownCard

  let show = function
    | A -> "A"
    | K -> "K"
    | Q -> "Q"
    | J -> "J"
    | T -> "T"
    | N9 -> "9"
    | N8 -> "8"
    | N7 -> "7"
    | N6 -> "6"
    | N5 -> "5"
    | N4 -> "4"
    | N3 -> "3"
    | N2 -> "2"

  let to_int = function
    | A -> 13
    | K -> 12
    | Q -> 11
    | J -> 10
    | T -> 9
    | N9 -> 8
    | N8 -> 7
    | N7 -> 6
    | N6 -> 5
    | N5 -> 4
    | N4 -> 3
    | N3 -> 2
    | N2 -> 1

  let cmp x y = to_int x - to_int y
end

module HandType = struct
  type t =
    | FiveOfAKind
    | FourOfAKind
    | FullHouse
    | ThreeOfAKind
    | TwoPair
    | OnePair
    | HighCard

  let to_int = function
    | FiveOfAKind -> 7
    | FourOfAKind -> 6
    | FullHouse -> 5
    | ThreeOfAKind -> 4
    | TwoPair -> 3
    | OnePair -> 2
    | HighCard -> 1

  let cmp x y = to_int x - to_int y

  exception Unreachable

  let hand_type (c1, c2, c3, c4, c5) =
    let hand =
      List.counts [ c1; c2; c3; c4; c5 ]
      |> List.map (fun (_, c) -> c)
      |> List.sort Int.compare
    in
    match hand with
    | [ 5 ] -> FiveOfAKind
    | [ 1; 4 ] -> FourOfAKind
    | [ 2; 3 ] -> FullHouse
    | [ 1; 1; 3 ] -> ThreeOfAKind
    | [ 1; 2; 2 ] -> TwoPair
    | [ 1; 1; 1; 2 ] -> OnePair
    | [ 1; 1; 1; 1; 1 ] -> HighCard
    | _ -> raise Unreachable

  let show = function
    | FiveOfAKind -> "FiveOfAKind"
    | FourOfAKind -> "FourOfAKind"
    | FullHouse -> "FullHouse"
    | ThreeOfAKind -> "ThreeOfAKind"
    | TwoPair -> "TwoPair"
    | OnePair -> "OnePair"
    | HighCard -> "HighCard"
end

module Hand = struct
  type t = { hand : Card.t * Card.t * Card.t * Card.t * Card.t; bid : int }

  exception UnexpectedInput

  let parse str =
    match String.split_on_char ' ' str with
    | [ hand; bid ] -> (
        match hand |> String.to_seq |> Seq.map Card.parse |> List.of_seq with
        | [ c1; c2; c3; c4; c5 ] ->
            { hand = (c1, c2, c3, c4, c5); bid = int_of_string bid }
        | _ -> raise UnexpectedInput)
    | _ -> raise UnexpectedInput

  let show { hand; bid } =
    let c1, c2, c3, c4, c5 = hand in
    "hand = (" ^ Card.show c1 ^ "," ^ Card.show c2 ^ "," ^ Card.show c3 ^ ","
    ^ Card.show c4 ^ "," ^ Card.show c5 ^ ")\tbid = (" ^ string_of_int bid ^ ")"

  let bid h = h.bid
  let hand h = h.hand

  let cmp_hand_type x y =
    HandType.cmp (HandType.hand_type x.hand) (HandType.hand_type y.hand)

  let first_card { hand; _ } =
    let c1, _, _, _, _ = hand in
    c1

  let second_card { hand; _ } =
    let _, c2, _, _, _ = hand in
    c2

  let third_card { hand; _ } =
    let _, _, c3, _, _ = hand in
    c3

  let fourth_card { hand; _ } =
    let _, _, _, c4, _ = hand in
    c4

  let fifth_card { hand; _ } =
    let _, _, _, _, c5 = hand in
    c5

  let cmp_cards x y =
    let cmp_first = Card.cmp (first_card x) (first_card y) in
    if cmp_first != 0 then cmp_first
    else
      let cmp_second = Card.cmp (second_card x) (second_card y) in
      if cmp_second != 0 then cmp_second
      else
        let cmp_third = Card.cmp (third_card x) (third_card y) in
        if cmp_third != 0 then cmp_third
        else
          let cmp_fourth = Card.cmp (fourth_card x) (fourth_card y) in
          if cmp_fourth != 0 then cmp_fourth
          else Card.cmp (fifth_card x) (fifth_card y)

  let cmp x y =
    let hand_type = cmp_hand_type x y in
    if hand_type = 0 then cmp_cards x y else hand_type
end

let read_file file = In_channel.with_open_bin file In_channel.input_all

let () =
  read_file "input.txt" |> String.trim |> String.lines |> List.map Hand.parse
  |> List.sort Hand.cmp
  |> List.mapi (fun i h ->
         (*print_endline (Hand.show h);
           print_endline (Hand.hand h |> HandType.hand_type |> HandType.show);
           print_newline (); *)
         Hand.bid h * (i + 1))
  |> List.sum |> string_of_int |> print_endline
