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
	var line []string
	var k, n, w int
	var needed int

	scanner = bufio.NewScanner(os.Stdin)

	if scanner.Scan() {
		line = strings.Fields(scanner.Text())
		k, _ = strconv.Atoi(line[0])
		n, _ = strconv.Atoi(line[1])
		w, _ = strconv.Atoi(line[2])

		needed = (((w * (w + 1)) / 2) * k) - n
		if needed > 0 {
			fmt.Println(needed)
		} else {
			fmt.Println("0")
		}
	}
}
