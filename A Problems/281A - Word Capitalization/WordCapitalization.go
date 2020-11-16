package main

import (
	"fmt"
	"strings"
)

func main() {
	var word string
	fmt.Scanln(&word)
	fmt.Println(strings.Title(word))
}
