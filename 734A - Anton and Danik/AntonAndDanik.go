package main

import "fmt"

func main() {
	var n int
	var games string
	var anton, danik int = 0, 0

	fmt.Scanln(&n)
	fmt.Scanln(&games)

	for _, game := range games {
		if game == 'A' {
			anton++
		} else {
			danik++
		}
	}

	if anton > danik {
		fmt.Println("Anton")
	} else if danik > anton {
		fmt.Println("Danik")
	} else {
		fmt.Println("Friendship")
	}
}
