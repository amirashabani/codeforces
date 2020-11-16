package main

import "fmt"

func main() {
	var length uint8
	var line string

	fmt.Scanln(&length)
	fmt.Scanln(&line)

	var min uint8 = 0
	var i uint8
	for i = 0; i < (length - 1); i++ {
		if line[i] == line[i+1] {
			min++
		}
	}

	fmt.Println(min)
}
