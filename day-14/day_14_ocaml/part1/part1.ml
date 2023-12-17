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

  let not_tilted_north { positions; _ } =
    let found_untilted =
      Hashtbl.fold
        (fun { row; col } pos found_untilted ->
          if found_untilted then found_untilted
          else
            let north =
              Hashtbl.find_opt positions { row = row - 1; col }
              |> Option.value ~default:Position.ImmovableRock
            in
            if pos == Position.MoveableRock && north == Position.Empty then true
            else false)
        positions false
    in
    found_untilted

  let shift_north { rows; columns; positions } =
    for y = 0 to rows - 1 do
      for x = 0 to columns - 1 do
        let key = { row = y; col = x } in
        let north_key = { row = y - 1; col = x } in
        let pos = Hashtbl.find positions key in
        let north =
          Hashtbl.find_opt positions north_key
          |> Option.value ~default:Position.ImmovableRock
        in
        if pos == Position.MoveableRock && north == Position.Empty then (
          Hashtbl.replace positions north_key Position.MoveableRock;
          Hashtbl.replace positions { row = y; col = x } Position.Empty)
      done
    done

  let tilt_north platform =
    while not_tilted_north platform do
      shift_north platform
    done;
    platform

  let calculate_load { positions; rows; _ } =
    Hashtbl.fold
      (fun { row; _ } pos load ->
        if pos == Position.MoveableRock then
          let inverted_row = rows - row in
          load + inverted_row
        else load)
      positions 0
end

let read_file file = In_channel.with_open_bin file In_channel.input_all

let () =
  read_file "input.txt" |> String.trim |> Platform.parse |> Platform.tilt_north
  |> Platform.calculate_load |> string_of_int |> print_endline
