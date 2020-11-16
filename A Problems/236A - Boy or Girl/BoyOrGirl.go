package main

import "fmt"

func main() {
	var line string
	var chars []rune
	var uniqueChars []rune
	var uniqueCount int

	fmt.Scanln(&line)
	chars = []rune(line)
	uniqueChars = unique(chars)
	uniqueCount = len(uniqueChars)

	if uniqueCount%2 == 1 {
		fmt.Println("IGNORE HIM!")
	} else {
		fmt.Println("CHAT WITH HER!")
	}
}

func unique(list []rune) []rune {
	var u []rune

	for _, element := range list {
		if !contains(u[:], element) {
			u = append(u, element)
		}
	}

	return u
}

func contains(list []rune, r rune) bool {
	for _, element := range list {
		if element == r {
			return true
		}
	}
	return false
}
