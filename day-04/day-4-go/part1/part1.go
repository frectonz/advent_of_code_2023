package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type card struct {
	id          int
	nums        []int
	winningNums []int
}

func (c *card) points() int {
	points := 0

	for i := 0; i < len(c.nums); i++ {
		num := c.nums[i]
		for j := 0; j < len(c.winningNums); j++ {
			winningNum := c.winningNums[j]
			if num == winningNum {
				if points == 0 {
					points = 1
				} else {
					points *= 2
				}
			}
		}
	}

	return points
}

func parseCard(input string) card {
	inputArray := strings.Split(input, "|")

	cardAndWinningNums := strings.TrimSpace(inputArray[0])
	cardAndWinningNumsArray := strings.Split(cardAndWinningNums, ":")

	cardString := strings.TrimSpace(cardAndWinningNumsArray[0])
	cardArray := strings.Fields(cardString)

	id, cardIdErr := strconv.Atoi(cardArray[1])
	if cardIdErr != nil {
		err_msg := fmt.Sprintf("failed to parse card id: '%s'", cardArray[1])
		panic(err_msg)
	}

	winningNumsString := strings.TrimSpace(cardAndWinningNumsArray[1])
	winningNumsArray := strings.Fields(winningNumsString)

	winningNums := make([]int, len(winningNumsArray))
	for i := 0; i < len(winningNumsArray); i++ {
		num := winningNumsArray[i]

		winningNum, winningNumErr := strconv.Atoi(num)
		if winningNumErr != nil {
			err_msg := fmt.Sprintf("failed to parse winning num: '%s'", num)
			panic(err_msg)
		}

		winningNums[i] = winningNum
	}

	numsString := strings.TrimSpace(inputArray[1])
	numsArray := strings.Fields(numsString)

	nums := make([]int, len(numsArray))
	for i := 0; i < len(numsArray); i++ {
		numString := numsArray[i]

		num, numErr := strconv.Atoi(numString)
		if numErr != nil {
			err_msg := fmt.Sprintf("failed to parse scratchcard num: '%s'", numString)
			panic(err_msg)
		}

		nums[i] = num
	}

	return card{
		id:          id,
		nums:        nums,
		winningNums: winningNums,
	}
}

func main() {
	contentsByte, fileReadErr := os.ReadFile("input.txt")
	if fileReadErr != nil {
		panic("failed to read file")
	}

	contents := string(contentsByte)
	lines := strings.Split(strings.TrimSpace(contents), "\n")

	sum := 0
	for i := 0; i < len(lines); i++ {
		card := parseCard(lines[i])
		sum += card.points()
	}

	fmt.Println("Answer", sum)
}
