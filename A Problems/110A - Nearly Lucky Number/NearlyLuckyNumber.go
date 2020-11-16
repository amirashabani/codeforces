package main

import (
	"fmt"
	"strconv"
)

func main() {
	var number string

	fmt.Scanln(&number)

	if isNearlyLucky(number) {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}

func isLucky(number string) bool {
	for _, letter := range number {
		if letter != '4' && letter != '7' {
			return false
		}
	}

	return true
}

func isNearlyLucky(number string) bool {
	var lucky int = 0

	for _, letter := range number {
		if letter == '4' || letter == '7' {
			lucky++
		}
	}

	if isLucky(strconv.Itoa(lucky)) {
		return true
	}
	return false
}
