package main

import (
	"fmt"
	"strings"
	"unicode"
)

var numbers []string = []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

func searchFirst(str string, words bool) int {
	for i := 0; i < len(str); i++ {
		if unicode.IsDigit(rune(str[i])) {
			return int(str[i] - '0')
		}
		if words {
			for j, number := range numbers {
				if strings.HasPrefix(string(str[i:]), number) {
					return j + 1
				}
			}
		}
	}
	return -1
}

func searchLast(str string, words bool) int {
	for i := len(str) - 1; i >= 0; i-- {
		if unicode.IsDigit(rune(str[i])) {
			return int(str[i] - '0')
		}
		if words {
			for j, number := range numbers {
				if strings.HasSuffix(string(str[:i+1]), number) {
					return j + 1
				}
			}
		}
	}
	return -1
}

func Part1(file []string) {
	var sum int
	for _, riga := range file {
		var first, last int = searchFirst(riga, false), searchLast(riga, false)
		sum += first*10 + last
	}
	fmt.Println(sum)
}

func Part2(file []string) {
	var sum int
	for _, riga := range file {
		var first, last int = searchFirst(riga, true), searchLast(riga, true)
		sum += first*10 + last
	}
	fmt.Println(sum)
}
