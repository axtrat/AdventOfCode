package main

import (
	"fmt"
	"regexp"
	"slices"
	"strconv"
)

func Map[T, V any](slice []T, fn func(T) V) []V {
	result := make([]V, len(slice))
	for i, t := range slice {
		result[i] = fn(t)
	}
	return result
}

func MapStringToInt(slice []string) []int {
	return Map(slice, func(s string) int {
		n, _ := strconv.Atoi(s)
		return n
	})
}

func Part1(file []string) {
	var (
		re    = regexp.MustCompile("\\d+")
		index int
		seeds []int = MapStringToInt(re.FindAllString(file[index], -1))
	)
	for index < len(file) {
		mapp, i := nextMap(file, index)
		seeds = Map(seeds, mapp)
		index = i + 1
	}

	fmt.Println(slices.Min(seeds))
}

func nextMap(file []string, index int) (func(int) int, int) {
	var (
		maps [][]int = make([][]int, 0)
		re           = regexp.MustCompile("\\d+")
		line []int
	)
	for len(line) <= 0 {
		index++
		line = MapStringToInt(re.FindAllString(file[index], -1))
	}
	for len(line) > 0 && index < len(file)-1 {
		//fmt.Println(index+1, line)
		maps = append(maps, line)
		index++
		line = MapStringToInt(re.FindAllString(file[index], -1))
	}

	return func(i int) int {
		for _, mapp := range maps {
			diff := i - mapp[1]
			if diff >= 0 && diff < mapp[2] {
				return mapp[0] + diff
			}
		}
		return i
	}, index
}

func Part2(file []string) {
	fmt.Println("Non Implementata")
}
