package main

import "fmt"

type Range struct {
	start, finish int
}

func newRange(start, len int) Range {
	return Range{start, start + len}
}

func (r Range) IsEmpty() bool {
	return r.finish <= r.start
}

func (r Range) Len() int {
	return r.finish - r.start
}

func (r Range) Intersect(o Range) Range {
	return Range{max(r.start, o.start), min(r.finish, o.finish)}
}

func (r Range) Complementary(o Range) (Range, Range) {
	if r.start >= o.finish || r.finish <= o.start {
		return r, Range{}
	}
	return Range{r.start, o.start}, Range{o.finish, r.finish}
}

func (r Range) Shift(n int) Range {
	return Range{r.start + n, r.finish + n}
}

func (r Range) String() string {
	if r.IsEmpty() {
		return "[]"
	}
	if r.Len() == 1 {
		return fmt.Sprintf("[%d]", r.start)
	}
	return fmt.Sprintf("[%d:%d]", r.start, r.finish)
}
