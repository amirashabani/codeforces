package main

import (
	"fmt"
	"strings"
)

func main() {
	var word string
	var lower int = 0

	fmt.Scanln(&word)

	for _, letter := range word {
		if 'a' <= letter && letter <= 'z' {
			lower++
		}
	}

	if lower >= (len(word) - lower) {
		fmt.Println(strings.ToLower(word))
	} else {
		fmt.Println(strings.ToUpper(word))
	}
}
