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

func newRanges(file []string, index int, ranges []Range) ([]Range, int) {
	var (
		re   = regexp.MustCompile("\\d+")
		line []int
		res  []Range = make([]Range, 0)
	)

	for ; index < len(file); index++ {
		line = MapStringToInt(re.FindAllString(file[index], -1))
		if len(line) == 0 {
			break
		}

		mR := newRange(line[1], line[2])
		old := make([]Range, 0)
		for _, r := range ranges {
			in := r.Intersect(mR)
			out1, out2 := r.Complementary(mR)
			if !in.IsEmpty() {
				res = append(res, in.Shift(line[0]-line[1]))
			}
			if !out1.IsEmpty() {
				old = append(old, out1)
			}
			if !out2.IsEmpty() {
				old = append(old, out2)
			}
		}
		ranges = old

	}

	return append(ranges, res...), index
}

func Part1(file []string) {
	var (
		re         = regexp.MustCompile("\\d+")
		index  int = 3
		ranges []Range
	)

	seeds := MapStringToInt(re.FindAllString(file[0], -1))
	ranges = Map(seeds, func(i int) Range { return newRange(i, 1) })

	for index < len(file) {
		ranges, index = newRanges(file, index, ranges)
		index += 2
	}

	fmt.Println(slices.Min(Map(ranges, func(r Range) int { return r.start })))
}

func Part2(file []string) {
	var (
		re             = regexp.MustCompile("\\d+")
		index  int     = 3
		ranges []Range = make([]Range, 0)
	)

	seeds := MapStringToInt(re.FindAllString(file[0], -1))
	for i := 0; i < len(seeds); i += 2 {
		ranges = append(ranges, newRange(seeds[i], seeds[i+1]))
	}

	for index < len(file) {
		ranges, index = newRanges(file, index, ranges)
		index += 2
	}
	fmt.Println(slices.Min(Map(ranges, func(r Range) int { return r.start })))
}
