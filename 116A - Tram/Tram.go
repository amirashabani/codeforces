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
	var stops int
	var exits, enters int
	var line []string
	var capacity, maxCapacity int

	capacity = 0
	scanner = bufio.NewScanner(os.Stdin)

	fmt.Scanln(&stops)
	for i := 0; i < stops; i++ {
		if scanner.Scan() {
			line = strings.Fields(scanner.Text())
			exits, _ = strconv.Atoi(line[0])
			enters, _ = strconv.Atoi(line[1])

			capacity += (enters - exits)
			if capacity > maxCapacity {
				maxCapacity = capacity
			}
		}
	}

	fmt.Println(maxCapacity)
}
