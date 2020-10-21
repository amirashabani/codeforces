package main

import (
	"fmt"
	"strconv"
)

func main() {
	var year int
	var beautiful int

	fmt.Scanln(&year)

	beautiful = year + 1
	for !isBeautiful(strconv.Itoa(beautiful)) {
		beautiful++
	}

	fmt.Println(beautiful)
}

func isBeautiful(number string) bool {
	var distinctDigits = distinct(number)

	if len(distinctDigits) == len(number) {
		return true
	}

	return false
}

func distinct(str string) string {
	var uniqueChars []rune

	for _, letter := range str {
		if !contains(uniqueChars, letter) {
			uniqueChars = append(uniqueChars, letter)
		}
	}

	return string(uniqueChars)
}

func contains(slice []rune, char rune) bool {
	for _, element := range slice {
		if element == char {
			return true
		}
	}

	return false
}
