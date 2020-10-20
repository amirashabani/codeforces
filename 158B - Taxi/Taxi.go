package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strings"
)

func main() {
	var scanner *bufio.Scanner
	var lines []string
	var numbers []string
	var count = map[string]int{
		"1": 0,
		"2": 0,
		"3": 0,
		"4": 0,
	}

	scanner = bufio.NewScanner(os.Stdin)
	const maxCapacity = 512 * 1024
	buf := make([]byte, maxCapacity)
	scanner.Buffer(buf, maxCapacity)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	numbers = strings.Fields(lines[1])

	for _, number := range numbers {
		count[number]++
	}

	fmt.Println(minimumTaxis(count))
}

func minimumTaxis(count map[string]int) int {
	var minimum int = 0

	minimum += count["4"]
	minimum += count["3"]

	if count["1"] > count["3"] {
		count["1"] -= count["3"]
	} else {
		count["1"] = 0
	}

	minimum += count["2"] / 2
	count["2"] = count["2"] % 2
	minimum += count["2"]

	if count["2"] != 0 {
		if count["1"] > 2 {
			count["1"] -= 2
		} else {
			count["1"] = 0
		}
	}

	minimum += int(math.Ceil(float64(count["1"]) / 4))

	return minimum
}
