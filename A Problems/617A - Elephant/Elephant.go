package main

import "fmt"

func main() {
	var number int
	var minimumSteps int

	fmt.Scanln(&number)
	minimumSteps = minSteps(number)
	fmt.Println(minimumSteps)
}

func minSteps(number int) int {
	if number%5 == 0 {
		return number / 5
	}
	return (number / 5) + 1
}
