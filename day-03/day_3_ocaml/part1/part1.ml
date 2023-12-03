type cell =
  | Dot of { col : int; row : int }
  | Number of { value : int; start_col : int; end_col : int; row : int }
  | Symbol of { value : char; col : int; row : int }

(** let show_cell = function
  | Dot { col; row } -> Printf.sprintf "Dot; col = %d; row = %d;" col row
  | Number { value; start_col; end_col; row } ->
      Printf.sprintf
        "Number; value = %d; start_col = %d; end_col = %d; row = %d;" value
        start_col end_col row
  | Symbol { value; col; row } ->
      Printf.sprintf "Symbol; value = %c; row = %d; col = %d;" value col row

let show_grid = List.map show_cell **)

module String = struct
  include String

  let lines = String.split_on_char '\n'
end

module List = struct
  include List

  let any f l = List.find_opt f l |> Option.is_some
  let sum = List.fold_left ( + ) 0
end

module Char = struct
  include Char

  let to_digit = function
    | '0' -> Some 0
    | '1' -> Some 1
    | '2' -> Some 2
    | '3' -> Some 3
    | '4' -> Some 4
    | '5' -> Some 5
    | '6' -> Some 6
    | '7' -> Some 7
    | '8' -> Some 8
    | '9' -> Some 9
    | _ -> None
end

let parse_line row line =
  line |> String.to_seqi
  |> Seq.fold_left
       (fun acc (col, char) ->
         match acc with
         | [] ->
             (match Char.to_digit char with
             | Some digit ->
                 Number { value = digit; start_col = col; end_col = col; row }
             | None ->
                 if char = '.' then Dot { col; row }
                 else Symbol { value = char; col; row })
             :: []
         | hd :: tl -> (
             match (hd, Char.to_digit char) with
             | Number n, Some digit ->
                 Number
                   {
                     n with
                     value =
                       string_of_int n.value ^ string_of_int digit
                       |> int_of_string;
                     end_col = col;
                   }
                 :: tl
             | (Dot _ | Symbol _), Some digit ->
                 Number { value = digit; start_col = col; end_col = col; row }
                 :: acc
             | (Number _ | Symbol _ | Dot _), None ->
                 if char = '.' then Dot { col; row } :: acc
                 else Symbol { value = char; col; row } :: acc))
       []
  |> List.rev

let parse_grid input =
  input |> String.lines |> List.mapi parse_line |> List.flatten

let nums_adjacent_to_symbols grid =
  grid
  |> List.filter_map (function
       | Number { value; start_col; end_col; row } ->
           let left =
             grid
             |> List.any (function
                  | Symbol s -> start_col - 1 = s.col && row = s.row
                  | _ -> false)
           in
           let right =
             grid
             |> List.any (function
                  | Symbol s -> end_col + 1 = s.col && row = s.row
                  | _ -> false)
           in
           let top =
             grid
             |> List.any (function
                  | Symbol s ->
                      row - 1 = s.row && s.col >= start_col && s.col <= end_col
                  | _ -> false)
           in
           let bottom =
             grid
             |> List.any (function
                  | Symbol s ->
                      row + 1 = s.row && s.col >= start_col && s.col <= end_col
                  | _ -> false)
           in
           let top_right =
             grid
             |> List.any (function
                  | Symbol s -> row - 1 = s.row && end_col + 1 = s.col
                  | _ -> false)
           in
           let top_left =
             grid
             |> List.any (function
                  | Symbol s -> row - 1 = s.row && start_col - 1 = s.col
                  | _ -> false)
           in
           let bottom_right =
             grid
             |> List.any (function
                  | Symbol s -> row + 1 = s.row && end_col + 1 = s.col
                  | _ -> false)
           in
           let bottom_left =
             grid
             |> List.any (function
                  | Symbol s -> row + 1 = s.row && start_col - 1 = s.col
                  | _ -> false)
           in
           if
             left || right || top || bottom || top_left || top_right
             || bottom_left || bottom_right
           then Some value
           else None
       | _ -> None)

let read_file file = In_channel.with_open_bin file In_channel.input_all

let () =
  read_file "input.txt" |> parse_grid |> nums_adjacent_to_symbols |> List.sum
  |> string_of_int |> print_endline
