package main

import (
	"fmt"
	"strings"
	"unicode"
)

var numbers []string = []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

func Part1(file []string) {
	var sum int
	for _, riga := range file {
		var first, last int
		for i := 0; i < len(riga); i++ {
			if unicode.IsDigit(rune(riga[i])) {
				first = int(riga[i] - '0')
				break
			}
		}
		for i := len(riga) - 1; i >= 0; i-- {
			if unicode.IsDigit(rune(riga[i])) {
				last = int(riga[i] - '0')
				break
			}
		}
		sum += first*10 + last
	}
	fmt.Println(sum)
}

func searchFirst(str string) int {
	for i := 0; i < len(str); i++ {
		if unicode.IsDigit(rune(str[i])) {
			return int(str[i] - '0')
		}

		for j, number := range numbers {
			if strings.HasPrefix(string(str[i:]), number) {
				return j + 1
			}
		}
	}
	return -1
}

func searchLast(str string) int {
	for i := len(str) - 1; i >= 0; i-- {
		if unicode.IsDigit(rune(str[i])) {
			return int(str[i] - '0')
		}

		for j, number := range numbers {
			if strings.HasSuffix(string(str[:i+1]), number) {
				return j + 1
			}
		}
	}
	return -1
}

func Part2(file []string) {
	var sum int
	for _, riga := range file {
		var first, last int = searchFirst(riga), searchLast(riga)
		sum += first*10 + last
	}
	fmt.Println(sum)
}
