package main

import (
	"fmt"
	"regexp"
	"strconv"
)

type Point struct {
	x, y int
}

func (this Point) String() string {
	return fmt.Sprintf("(%d, %d)", this.x, this.y)
}

type Part struct {
	number int
	start  Point
	len    int
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func (this Part) isAdiacent(p Point) bool {
	if abs(this.start.y-p.y) > 1 {
		return false
	}

	for i := -1; i < this.len+1; i++ {
		if this.start.x+i == p.x {
			return true
		}
	}

	return false
}

func (this Part) String() string {
	return fmt.Sprintf("%d: %v, %d", this.number, this.start, this.len)
}

func Part1(file []string) {
	var (
		sum     int
		parts   []Part
		simbles []Point
	)
	parts, simbles = parseInput(file, func(r rune) bool { return r != '.' })

	for _, part := range parts {
		for _, simble := range simbles {
			if part.isAdiacent(simble) {
				sum += part.number
				break
			}
		}
	}
	fmt.Println(sum)
}

func Part2(file []string) {
	var (
		sum     int
		parts   []Part
		simbles []Point
	)
	parts, simbles = parseInput(file, func(r rune) bool { return r == '*' })

	for _, simble := range simbles {
		var count, gear int = 0, 1
		for _, part := range parts {
			if part.isAdiacent(simble) {
				count++
				gear *= part.number
			}
		}
		if count == 2 {
			sum += gear
		}
	}
	fmt.Println(sum)
}

func parseInput(file []string, filtro func(rune) bool) ([]Part, []Point) {
	var (
		parts   []Part  = make([]Part, 0)
		simbles []Point = make([]Point, 0)
	)

	re := regexp.MustCompile("\\d+|[^.]")
	for i, line := range file {
		for _, match := range re.FindAllStringIndex(line, -1) {
			n, err := strconv.Atoi(line[match[0]:match[1]])
			if err != nil {
				simbles = append(simbles, Point{match[0], i})
				continue
			}

			var nPart = Part{n, Point{match[0], i}, match[1] - match[0]}
			parts = append(parts, nPart)
		}
	}
	return parts, simbles
}
