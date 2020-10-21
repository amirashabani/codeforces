package main

import (
	"fmt"
	"strings"
)

func main() {
	var word, title, answer string

	fmt.Scanln(&word)
	title = strings.Title(word)

	if strings.ToUpper(title) == title {
		if word == title {
			answer = strings.ToLower(title)
		} else {
			answer = strings.ToLower(title)
			answer = strings.Title(answer)
		}
	} else {
		answer = word
	}

	fmt.Println(answer)
}
