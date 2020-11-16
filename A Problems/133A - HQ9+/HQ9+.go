package main

import "fmt"

func main() {
	var program string

	fmt.Scanln(&program)

	if outputs(program) {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}

func outputs(program string) bool {
	for _, letter := range program {
		if letter == 'H' || letter == 'Q' ||
			letter == '9' {
			return true
		}
	}

	return false
}
