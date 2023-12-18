[@@@warnerror "-unused-field"]
[@@@warnerror "-unused-value-declaration"]

module String = struct
  include String

  let lines = String.split_on_char '\n'
end

module Position = struct
  type t = MoveableRock | ImmovableRock | Empty

  exception UnknownChar

  let parse = function
    | 'O' -> MoveableRock
    | '#' -> ImmovableRock
    | '.' -> Empty
    | _ -> raise UnknownChar

  let print = function
    | MoveableRock -> print_char 'O'
    | ImmovableRock -> print_char '#'
    | Empty -> print_char '.'
end

type pos = { row : int; col : int }

let north { row; col } = { row = row - 1; col }
let south { row; col } = { row = row + 1; col }
let west { row; col } = { row; col = col - 1 }
let east { row; col } = { row; col = col + 1 }

module Platform = struct
  type t = {
    rows : int;
    columns : int;
    positions : (pos, Position.t) Hashtbl.t;
  }

  let parse input =
    let lines = input |> String.lines in
    {
      columns = List.nth lines 0 |> String.length;
      rows = lines |> List.length;
      positions =
        lines |> List.to_seq
        |> Seq.mapi (fun row line ->
               line |> String.to_seq
               |> Seq.mapi (fun col char -> ({ row; col }, Position.parse char)))
        |> Seq.concat |> Hashtbl.of_seq;
    }

  let print { rows; columns; positions } =
    for y = 0 to rows - 1 do
      for x = 0 to columns - 1 do
        let pos = Hashtbl.find positions { row = y; col = x } in
        Position.print pos
      done;
      print_newline ()
    done

  let not_tilted_on dir { positions; columns; rows } =
    let final = rows * columns in
    let rec inner num =
      if num >= final then false
      else
        let curr = { row = num / columns; col = num mod columns } in
        let pos = Hashtbl.find positions curr in
        let next =
          Hashtbl.find_opt positions (dir curr)
          |> Option.value ~default:Position.ImmovableRock
        in
        if pos = Position.MoveableRock && next = Position.Empty then true
        else inner (num + 1)
    in
    inner 0

  let shift_on dir { rows; columns; positions } =
    for y = 0 to rows - 1 do
      for x = 0 to columns - 1 do
        let key = { row = y; col = x } in
        let next_key = dir key in
        let pos = Hashtbl.find positions key in
        let next =
          Hashtbl.find_opt positions next_key
          |> Option.value ~default:Position.ImmovableRock
        in
        if pos = Position.MoveableRock && next = Position.Empty then (
          Hashtbl.replace positions next_key Position.MoveableRock;
          Hashtbl.replace positions { row = y; col = x } Position.Empty)
      done
    done

  let tilt dir platform =
    while not_tilted_on dir platform do
      shift_on dir platform
    done

  let cycle platform =
    tilt north platform;
    tilt west platform;
    tilt south platform;
    tilt east platform

  let copy platform =
    { platform with positions = Hashtbl.copy platform.positions }

  let rec repeat_cycle n cache platform =
    if n <= 0 then platform
    else
      match Hashtbl.find_opt cache platform with
      | Some in_cache -> repeat_cycle (n - 1) cache in_cache
      | None ->
          let _ = n |> string_of_int |> print_endline in
          let original = copy platform in
          cycle platform;
          Hashtbl.add cache original (copy platform);
          repeat_cycle (n - 1) cache platform

  let calculate_load { positions; rows; _ } =
    Hashtbl.fold
      (fun { row; _ } pos load ->
        if pos = Position.MoveableRock then
          let inverted_row = rows - row in
          load + inverted_row
        else load)
      positions 0
end

let read_file file = In_channel.with_open_bin file In_channel.input_all

let () =
  read_file "input.txt" |> String.trim |> Platform.parse
  |> Platform.repeat_cycle 1000 (Hashtbl.create 300)
  |> Platform.calculate_load |> string_of_int |> print_endline
