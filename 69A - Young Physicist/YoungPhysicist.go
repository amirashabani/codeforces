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
	var n uint8
	var line []string
	var x, y, z []int16
	var xSum, ySum, zSum int16

	scanner = bufio.NewScanner(os.Stdin)

	fmt.Scanln(&n)

	var i uint8
	for i = 0; i < n; i++ {
		if scanner.Scan() {
			line = strings.Fields(scanner.Text())
			for i, str := range line {
				number, err := strconv.Atoi(str)
				if err != nil {
					panic(err)
				}
				if i == 0 {
					x = append(x, int16(number))
				} else if i == 1 {
					y = append(y, int16(number))
				} else {
					z = append(z, int16(number))
				}
			}
		}
	}

	xSum = sum(x)
	ySum = sum(y)
	zSum = sum(z)

	if xSum == 0 && ySum == 0 && zSum == 0 {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}

func sum(array []int16) int16 {
	var result int16 = 0

	for _, element := range array {
		result += element
	}

	return result
}
