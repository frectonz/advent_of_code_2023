from enum import Enum
import sys
from itertools import pairwise


class Direction(Enum):
    Up = 3
    Down = 1
    Left = 2
    Right = 0

    @staticmethod
    def parse(input: str):
        if input == "U":
            return Direction.Up
        elif input == "D":
            return Direction.Down
        elif input == "L":
            return Direction.Left
        elif input == "R":
            return Direction.Right
        else:
            print("Unkown direction", input)
            sys.exit(1)


class DigCommand:
    dir: Direction
    dist: int

    def __init__(self, input: str) -> None:
        input_list = input.split(" ")

        if len(input_list) != 3:
            print("Unexpected num of input", input)
            sys.exit(1)

        color = input_list[2][2:-1]
        self.dir = Direction(int(color[5], 16))
        self.dist = int(color[0:5], 16)

    def __str__(self) -> str:
        return f"Dir: {self.dir.name}, Dist: {self.dist}"


Vertices = list[tuple[int, int]]


class DigPlan:
    plan: list[DigCommand]

    def __init__(self, input: str) -> None:
        if not input.strip():
            print("Input is empty.")
            sys.exit(1)
        self.plan = [DigCommand(line) for line in input.splitlines()]

    def vertices(self) -> Vertices:
        vertices: Vertices = [(0, 0)]

        row = 0
        col = 0
        for cmd in self.plan:
            if cmd.dir == Direction.Up:
                row -= cmd.dist
            elif cmd.dir == Direction.Down:
                row += cmd.dist
            elif cmd.dir == Direction.Right:
                col += cmd.dist
            elif cmd.dir == Direction.Left:
                col -= cmd.dist
            vertices.append((row, col))
        return vertices

    def area(self) -> int:
        vertices = self.vertices()
        shoelace = (
            abs(
                sum(
                    (col1 * row2 - row1 * col2)
                    for ((row1, col1), (row2, col2)) in pairwise(vertices)
                )
            )
            // 2
        )
        boundary = sum([cmd.dist for cmd in self.plan])
        interior = shoelace - boundary // 2 + 1
        return interior + boundary

    def __str__(self) -> str:
        return "\n".join(str(command) for command in self.plan)


def main():
    with open("input.txt", "r") as file:
        input = file.read()
    dig_plan = DigPlan(input)
    print("Answer", dig_plan.area())


main()
