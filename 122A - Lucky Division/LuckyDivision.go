package main

import (
	"fmt"
	"math"
	"strconv"
)

func main() {
	var number int

	fmt.Scanln(&number)
	if almostLucky(number) {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}

func almostLucky(number int) bool {
	var divs []int

	divs = divisors(number)

	for _, div := range divs {
		if lucky(div) {
			return true
		}
	}
	return false
}

func lucky(number int) bool {
	var str string

	str = strconv.Itoa(number)

	for _, letter := range str {
		if letter != '4' && letter != '7' {
			return false
		}
	}

	return true
}

func divisors(number int) []int {
	var divs []int
	var sqrt int

	divs = []int{1, number}
	sqrt = int(math.Sqrt(float64(number)))

	for i := 2; i <= sqrt; i++ {
		if number%i == 0 {
			divs = append(divs, i)
			if number/i != i {
				divs = append(divs, number/i)
			}
		}
	}
	return divs
}
