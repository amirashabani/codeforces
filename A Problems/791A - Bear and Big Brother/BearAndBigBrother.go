package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	var scanner *bufio.Scanner
	var line []string
	var a, b int
	var numerator, denominator float64
	var result int

	denominator = math.Log10(1.5)

	scanner = bufio.NewScanner(os.Stdin)

	if scanner.Scan() {
		line = strings.Fields(scanner.Text())
		a, _ = strconv.Atoi(line[0])
		b, _ = strconv.Atoi(line[1])

		numerator = math.Log10(float64(b) / float64(a))
		result = int(math.Floor(numerator/denominator)) + 1
		fmt.Println(result)
	}
}
