package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	var scanner *bufio.Scanner
	var line, queue string
	var t int

	scanner = bufio.NewScanner(os.Stdin)

	if scanner.Scan() {
		line = scanner.Text()
		t, _ = strconv.Atoi(strings.Fields(line)[1])
	}

	if scanner.Scan() {
		queue = scanner.Text()
	}

	for i := 0; i < t; i++ {
		queue = strings.Replace(queue, "BG", "GB", -1)
	}

	fmt.Println(queue)
}
