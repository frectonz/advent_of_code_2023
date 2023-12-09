[@@@warnerror "-unused-value-declaration"]

module String = struct
  include String

  let lines = String.split_on_char '\n'
end

module Hashtbl = struct
  include Hashtbl

  let find_or_add tbl key default =
    match find_opt tbl key with
    | Some value -> value
    | None ->
        add tbl key default;
        default
end

module Move = struct
  type move = R | L
  type t = move list

  exception UnknownMove

  let move = function 'R' -> R | 'L' -> L | _ -> raise UnknownMove
  let parse input = input |> String.to_seq |> Seq.map move |> List.of_seq
  let show_move = function R -> "R" | L -> "L"
  let show moves = moves |> List.map show_move |> String.concat ","
end

module Tree = struct
  type t = Leaf | Node of t * string * t

  exception UnexpectedInput

  let parse input =
    let lines = String.lines input in
    let table = Hashtbl.create (List.length lines) in
    let lines =
      lines
      |> List.map (fun line ->
             let line =
               line |> String.split_on_char '=' |> List.map String.trim
             in
             let value, l_and_r =
               match line with
               | [ value; l_and_r ] -> (value, l_and_r)
               | _ -> raise UnexpectedInput
             in
             let l_and_r =
               String.to_seq l_and_r
               |> Seq.filter (fun c -> c <> '(' && c <> ')')
               |> String.of_seq
             in
             let l_and_r =
               l_and_r |> String.split_on_char ',' |> List.map String.trim
             in
             let left, right =
               match l_and_r with
               | [ l; r ] -> (l, r)
               | _ -> raise UnexpectedInput
             in
             Hashtbl.add table value (left, right);
             (value, (left, right)))
    in
    (lines, table)

  let rec construct_tree mappings visited node =
    let visit_count = Hashtbl.find_or_add visited node 1 in
    let current_node = Hashtbl.find_opt mappings node in
    match current_node with
    | Some (left, right) when visit_count = 1 ->
        Hashtbl.replace visited node (visit_count + 1);
        let left_tree = construct_tree mappings visited left in
        let right_tree = construct_tree mappings visited right in
        Node (left_tree, node, right_tree)
    | _ -> Node (Leaf, node, Leaf)

  let construct_tree_from_list (mappings, table) =
    match mappings with
    | [] -> Leaf
    | (root, (_, _)) :: _ ->
        construct_tree table (Hashtbl.create (List.length mappings)) root

  let make input = parse input |> construct_tree_from_list

  let count_steps all_moves all_tree =
    let rec count_steps' moves tree acc =
      match (moves, tree) with
      | [], Node (_, _, _) -> count_steps' all_moves tree acc
      | _ :: _, Node (Leaf, v, Leaf) when v = "AAA" ->
          count_steps' moves all_tree acc
      | _, Node (_, v, _) when v = "ZZZ" -> acc
      | h :: tl, Node (l, _, r) -> (
          match h with
          | Move.R -> count_steps' tl r (acc + 1)
          | Move.L -> count_steps' tl l (acc + 1))
      | _ -> acc
    in
    count_steps' all_moves all_tree 0

  let rec get_sub_tree v = function
    | Leaf -> None
    | Node (Leaf, _, Leaf) -> None
    | Node (l, x, r) when x = v -> Some (Node (l, x, r))
    | Node (l, _, r) -> (
        match (get_sub_tree v l, get_sub_tree v r) with
        | None, None -> None
        | Some n, None -> Some n
        | None, Some n -> Some n
        | Some l, Some r -> Some (Node (l, v, r)))

  let show tree =
    let get_name = function Leaf -> "." | Node (_, value, _) -> value in
    let get_children = function
      | Leaf -> []
      | Node (a, _, b) -> List.filter (( <> ) Leaf) [ a; b ]
    in
    ShowTree.to_string ~get_name ~get_children tree

  let normalize all_tree =
    let rec normalize' tree =
      match tree with
      | Leaf -> Leaf
      | Node (Leaf, x, Leaf) when x <> "ZZZ" && x <> "AAA" -> (
          match get_sub_tree x all_tree with
          | None -> Node (Leaf, x, Leaf)
          | Some node -> node)
      | Node (l, x, r) -> Node (normalize' l, x, normalize' r)
    in
    normalize' all_tree
end

module Map = struct
  type t = { moves : Move.t; tree : Tree.t }

  exception UnexpectedInput

  let parse input =
    let input =
      input |> Str.split (Str.regexp "\n\n") |> List.map String.trim
    in
    match input with
    | [ moves; tree ] ->
        { moves = Move.parse moves; tree = tree |> Tree.make |> Tree.normalize }
    | _ -> raise UnexpectedInput

  let show map = Move.show map.moves ^ "\n\n" ^ Tree.show map.tree
  let count map = Tree.count_steps map.moves map.tree
  let tree map = map.tree

  let debug map =
    map |> show |> print_endline;
    map
end

let read_file file = In_channel.with_open_bin file In_channel.input_all

let () =
  read_file "test1.txt" |> Map.parse |> Map.debug |> Map.count |> string_of_int
  |> ( ^ ) "Steps: " |> print_endline
