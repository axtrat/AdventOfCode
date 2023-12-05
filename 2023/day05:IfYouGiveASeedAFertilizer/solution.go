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

type Range struct {
	start, len int
}

func (r Range) IsEmpty() bool {
	return r.len <= 0
}

func (r Range) Finish() int {
	return r.start + r.len
}

func (r Range) Intersect(o Range) (in, out Range) {
	if o.start <= r.start {
		diff := r.start - o.start
		if diff+r.len <= o.len {
			return r, Range{0, 0}
		}
		if diff >= 0 && diff < o.len {
			in.start = r.start
			in.len = o.len - diff
			out.start = in.Finish()
			out.len = r.Finish() - o.Finish()
			return in, out
		}
	} else {
		diff := o.start - r.start
		if diff < r.len {
			in.start = o.start
			in.len = r.len - diff
			out.start = r.start
			out.len = diff
			return in, out
		}
	}

	return Range{0, 0}, r
}

func (r Range) String() string {
	if r.len == 0 {
		return "[]"
	}
	if r.len == 1 {
		return fmt.Sprintf("[%d]", r.start)
	}
	return fmt.Sprintf("[%d:%d]", r.start, r.start+r.len)
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

		mR := Range{line[1], line[2]}
		old := make([]Range, 0)
		for _, r := range ranges {
			in, out := r.Intersect(mR)
			if !in.IsEmpty() {
				in.start += (line[0] - line[1])
				res = append(res, in)
			}
			if !out.IsEmpty() {
				old = append(old, out)
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
	ranges = Map(seeds, func(i int) Range { return Range{i, 1} })

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
		ranges = append(ranges, Range{seeds[i], seeds[i+1]})
	}

	for index < len(file) {
		ranges, index = newRanges(file, index, ranges)
		index += 2
	}

	fmt.Println(slices.Min(Map(ranges, func(r Range) int { return r.start })))
}
