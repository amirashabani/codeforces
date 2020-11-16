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
	var lines []string
	var n int

	scanner = bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	n, _ = strconv.Atoi(lines[0])
	var gifts = make([]string, n)

	for i, gift := range strings.Fields(lines[1]) {
		index, _ := strconv.Atoi(gift)
		gifts[index-1] = strconv.Itoa(i + 1)
	}

	fmt.Println(strings.Join(gifts, " "))
}
