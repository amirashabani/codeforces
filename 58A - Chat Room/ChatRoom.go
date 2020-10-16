package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var scanner *bufio.Scanner
	var line string

	scanner = bufio.NewScanner(os.Stdin)

	if scanner.Scan() {
		line = scanner.Text()
		if managed(line) {
			fmt.Println("YES")
		} else {
			fmt.Println("NO")
		}
	}
}

func managed(message string) bool {
	var hello = []rune("hello")

	for _, letter := range message {
		if len(hello) == 0 {
			return true
		}
		if letter == hello[0] {
			hello = hello[1:]
		}
	}

	if len(hello) == 0 {
		return true
	}

	return false
}
