package main

import (
	"fmt"
	"slices"
	"strings"
)

func Part1(file []string) {
	var sum int
	for _, line := range file {
		var (
			points     int = 1
			infos          = strings.Split(line, ": ")
			numbers        = strings.Split(infos[1], " | ")
			winNumbers     = strings.Fields(numbers[0])
		)
		for _, number := range strings.Fields(numbers[1]) {
			if slices.Contains(winNumbers, number) {
				points *= 2
			}
		}
		sum += points / 2
	}
	fmt.Println(sum)
}

func Part2(file []string) {
	var listOfCopies []int = make([]int, len(file))
	var res int = 0
	for i, line := range file {
		listOfCopies[i]++
		var (
			count      int = 0
			infos          = strings.Split(line, ": ")
			numbers        = strings.Split(infos[1], " | ")
			winNumbers     = strings.Fields(numbers[0])
		)

		for _, number := range strings.Fields(numbers[1]) {
			if slices.Contains(winNumbers, number) {
				count++
			}
		}

		res += listOfCopies[i]
		for j := 1; j <= count; j++ {
			listOfCopies[i+j] += listOfCopies[i]
		}
	}

	fmt.Println(res)
}
