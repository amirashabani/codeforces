package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var scanner *bufio.Scanner
	var lines []string
	var previous string
	var groups int = 0

	scanner = bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	for i, magnet := range lines[1:] {
		if i == 0 {
			previous = magnet
			groups++
		} else {
			if magnet != previous {
				groups++
			}

			previous = magnet
		}
	}

	fmt.Println(groups)
}
