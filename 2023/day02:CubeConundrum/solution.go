package main

import (
	"fmt"
	"strings"
)

func Part1(file []string) {
	var sum int
	var limit = map[string]int{
		"red":   12,
		"green": 13,
		"blue":  14,
	}
	for _, riga := range file {
		var (
			gameId int
			sets   []string
			valid  bool = true
		)

		infos := strings.Split(riga, ": ")
		fmt.Sscanf(infos[0], "Game %d:", &gameId)
		sets = strings.Split(infos[1], "; ")

		for _, set := range sets {
			for _, cubes := range strings.Split(set, ", ") {
				var number int
				var color string
				fmt.Sscanf(cubes, "%d%s", &number, &color)
				if number > limit[color] {
					valid = false
					break
				}
			}
		}

		if valid {
			sum += gameId
		}
	}
	fmt.Println(sum)
}

func Part2(file []string) {
	var sum int
	for _, riga := range file {
		var (
			gameId int
			sets   []string
			limit  = map[string]int{
				"red":   0,
				"green": 0,
				"blue":  0,
			}
		)

		infos := strings.Split(riga, ": ")
		fmt.Sscanf(infos[0], "Game %d:", &gameId)
		sets = strings.Split(infos[1], "; ")

		for _, set := range sets {
			for _, cubes := range strings.Split(set, ", ") {
				var number int
				var color string
				fmt.Sscanf(cubes, "%d%s", &number, &color)
				if number > limit[color] {
					limit[color] = number
				}
			}
		}
		sum += limit["red"] * limit["green"] * limit["blue"]
	}
	fmt.Println(sum)
}
