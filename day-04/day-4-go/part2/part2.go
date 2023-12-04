package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type card struct {
	id           int
	matchingNums int
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

	matchingNums := 0
	for _, num := range nums {
		for _, winningNum := range winningNums {
			if num == winningNum {
				matchingNums += 1
			}
		}
	}

	return card{
		id:           id,
		matchingNums: matchingNums,
	}
}

func main() {
	contentsByte, fileReadErr := os.ReadFile("input.txt")
	if fileReadErr != nil {
		panic("failed to read file")
	}

	contents := string(contentsByte)
	lines := strings.Split(strings.TrimSpace(contents), "\n")

	cards := make([]card, len(lines))
	idToMatchingNums := make(map[int]int, len(lines))
	for i, line := range lines {
		cards[i] = parseCard(line)
		idToMatchingNums[cards[i].id] = cards[i].matchingNums
	}

	for i := 0; i < len(cards); i++ {
		c := cards[i]

		for j := c.id + 1; j <= c.id+c.matchingNums; j++ {
			other_c := card{id: j, matchingNums: idToMatchingNums[j]}
			cards = append(cards, other_c)
		}
	}

	fmt.Println(len(cards))
}
