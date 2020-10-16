package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	var scanner *bufio.Scanner
	var line []string
	var n, k int

	scanner = bufio.NewScanner(os.Stdin)

	if scanner.Scan() {
		line = strings.Fields(scanner.Text())
		n, _ = strconv.Atoi(line[0])
		k, _ = strconv.Atoi(line[1])

		for i := 0; i < k; i++ {
			n = tanya(n)
		}

		fmt.Println(n)
	}
}

func tanya(number int) int {
	if number%10 == 0 {
		return number / 10
	}
	return number - 1
}
