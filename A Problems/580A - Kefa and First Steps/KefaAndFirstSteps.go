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
	var lines []string
	var previous, current int
	const bufferLimit = 1200000
	var buffer []byte
	var localMax, max = 1, 1

	scanner = bufio.NewScanner(os.Stdin)
	buffer = make([]byte, bufferLimit)
	scanner.Buffer(buffer, bufferLimit)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	for i, str := range strings.Fields(lines[1]) {
		if i == 0 {
			previous, _ = strconv.Atoi(str)
		} else {
			current, _ = strconv.Atoi(str)
			if current >= previous {
				localMax++
				if localMax > max {
					max = localMax
				}
			} else {
				localMax = 1
			}
			previous = current
		}
	}

	fmt.Println(max)
}
