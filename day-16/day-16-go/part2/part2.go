package main

import (
	"fmt"
	"os"
	"strings"
)

type Pos struct {
	row int
	col int
}

type Dir int

const (
	Up Dir = iota
	Down
	Left
	Right
)

type Beam struct {
	pos Pos
	dir Dir
}

func main() {
	contentsByte, fileReadErr := os.ReadFile("input.txt")
	if fileReadErr != nil {
		panic("failed to read file")
	}

	contents := string(contentsByte)
	lines := strings.Split(strings.TrimSpace(contents), "\n")

	grid := make([][]rune, len(lines))
	for i, line := range lines {
		grid[i] = []rune(line)
	}

	rows := len(grid)
	cols := len(grid[0])

	answer := 0

	// from left
	for i := 0; i < rows; i++ {
		answer = max(countEnergized(Pos{i, -1}, Right, grid), answer)
	}

	// from right
	for i := 0; i < rows; i++ {
		answer = max(countEnergized(Pos{i, cols}, Left, grid), answer)
	}

	// from top
	for i := 0; i < cols; i++ {
		answer = max(countEnergized(Pos{-1, i}, Down, grid), answer)
	}

	// from bottom
	for i := 0; i < cols; i++ {
		answer = max(countEnergized(Pos{rows, i}, Up, grid), answer)
	}

	fmt.Println("Answer", answer)
}

func countEnergized(start Pos, dir Dir, grid [][]rune) int {
	rows := len(grid)
	cols := len(grid[0])

	energized := map[Pos]bool{}

	queue := make([]Beam, 1)
	queue[0] = Beam{start, dir}

	visisted := map[string]bool{}

	for len(queue) != 0 {
		curr := queue[0]
		queue = queue[1:]

		key := fmt.Sprintf("%d-%d-%d", curr.dir, curr.pos.row, curr.pos.col)
		if visisted[key] {
			continue
		} else {
			visisted[key] = true
		}

		next := curr.pos
		switch curr.dir {
		case Up:
			if next.row == 0 {
				continue
			} else {
				next.row = next.row - 1
			}
		case Down:
			if next.row+1 == rows {
				continue
			} else {
				next.row = next.row + 1
			}
		case Left:
			if next.col == 0 {
				continue
			} else {
				next.col = next.col - 1
			}
		case Right:
			if next.col+1 == cols {
				continue
			} else {
				next.col = next.col + 1
			}
		default:
			panic("this will never happen: unknown dir")
		}

		energized[next] = true
		nextVal := grid[next.row][next.col]

		switch nextVal {
		case '.':
			queue = append(queue, Beam{next, curr.dir})

		case '\\':
			nextDir := curr.dir

			switch nextDir {
			case Up:
				nextDir = Left
			case Down:
				nextDir = Right
			case Left:
				nextDir = Up
			case Right:
				nextDir = Down
			default:
				panic("this will never happen: unknown dir")
			}

			queue = append(queue, Beam{next, nextDir})

		case '/':
			nextDir := curr.dir

			switch nextDir {
			case Up:
				nextDir = Right
			case Down:
				nextDir = Left
			case Left:
				nextDir = Down
			case Right:
				nextDir = Up
			default:
				panic("this will never happen: unknown dir")
			}

			queue = append(queue, Beam{next, nextDir})

		case '|':
			switch curr.dir {
			case Up, Down:
				queue = append(queue, Beam{next, curr.dir})
			case Left, Right:
				queue = append(queue, Beam{next, Up})
				queue = append(queue, Beam{next, Down})
			default:
				panic("this will never happen: unknown dir")
			}

		case '-':
			switch curr.dir {
			case Left, Right:
				queue = append(queue, Beam{next, curr.dir})
			case Up, Down:
				queue = append(queue, Beam{next, Left})
				queue = append(queue, Beam{next, Right})
			default:
				panic("this will never happen: unknown dir")
			}

		default:
			panic("this will never happen: unknown char")
		}

	}

	return len(energized)
}
