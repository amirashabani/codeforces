package main

import (
	"fmt"
	"strings"
)

func main() {
	var line string
	fmt.Scanln(&line)

	var ones string
	var zeroes string

	zeroes = strings.Repeat("0", 7)
	ones = strings.Repeat("1", 7)

	if strings.Contains(line, zeroes) || strings.Contains(line, ones) {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}
