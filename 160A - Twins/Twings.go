package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	var scanner *bufio.Scanner
	var n, allSum, minimumCoins int
	var sum int
	var line []string
	var numbers []int

	minimumCoins, sum = 0, 0
	scanner = bufio.NewScanner(os.Stdin)
	fmt.Scanln(&n)

	if scanner.Scan() {
		line = strings.Fields(scanner.Text())
		for _, word := range line {
			number, _ := strconv.Atoi(word)
			numbers = append(numbers, number)
		}

		allSum = getSum(numbers)

		sort.Slice(numbers, func(i, j int) bool {
			return numbers[i] > numbers[j]
		})

		for sum <= (allSum - sum) {
			sum += numbers[minimumCoins]
			minimumCoins++
		}

		fmt.Println(minimumCoins)
	}
}

func getSum(numbers []int) int {
	var result int
	for _, number := range numbers {
		result += number
	}

	return result
}
