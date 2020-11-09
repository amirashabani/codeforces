package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	var scanner *bufio.Scanner
	var lines []string
	var isEasy bool = true

	scanner = bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	for _, str := range strings.Fields(lines[1]) {
		if str == "1" {
			isEasy = false
		}
	}

	if isEasy {
		fmt.Println("EASY")
	} else {
		fmt.Println("HARD")
	}
}
