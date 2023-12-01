package main

import (
	"fmt"
	"unicode"
)

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

func Part2(file []string) {
	fmt.Println("Non Implementata")
}
