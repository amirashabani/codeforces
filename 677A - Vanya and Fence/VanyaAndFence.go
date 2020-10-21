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
	var n, h int
	var heights []int
	var bent int = 0

	scanner = bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	var nh = strings.Fields(lines[0])
	n, _ = strconv.Atoi(nh[0])
	h, _ = strconv.Atoi(nh[1])

	var height int
	for _, element := range strings.Fields(lines[1]) {
		height, _ = strconv.Atoi(element)
		heights = append(heights, height)
	}

	for _, height := range heights {
		if height > h {
			bent++
		}
	}

	fmt.Println((bent + n))
}
