package main

import "fmt"

type Range struct {
	start, finish int
}

func (r Range) IsEmpty() bool {
	return r.finish <= r.start
}

func (r Range) Len() int {
	return r.finish - r.start
}

func (r Range) Intersect(o Range) (in, out1, out2 Range) {

	if o.start <= r.start {
		if r.finish <= o.finish {
			return r, Range{0, 0}, Range{0, 0}
		}
		if r.start < o.finish {
			return Range{r.start, o.finish}, Range{o.finish, r.finish}, Range{0, 0}
		}
	} else {
		if o.start < r.finish {
			if o.finish < r.finish {
				return o, Range{r.start, o.start}, Range{o.finish, r.finish}
			}

			return Range{o.start, r.finish}, Range{r.start, o.start}, Range{0, 0}
		}
	}

	return Range{0, 0}, r, Range{0, 0}
}

func (r Range) String() string {
	if r.Len() == 0 {
		return "[]"
	}
	if r.Len() == 1 {
		return fmt.Sprintf("[%d]", r.start)
	}
	return fmt.Sprintf("[%d:%d]", r.start, r.finish)
}
