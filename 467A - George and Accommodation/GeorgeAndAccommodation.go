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
	var lines, room []string
	var free int
	var p, q int

	scanner = bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	for _, line := range lines[1:] {
		room = strings.Fields(line)
		p, _ = strconv.Atoi(room[0])
		q, _ = strconv.Atoi(room[1])

		if (q - p) >= 2 {
			free++
		}
	}

	fmt.Println(free)
}
