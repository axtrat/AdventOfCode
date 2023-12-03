package main

import (
	"fmt"
	"unicode"
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

	for i, line := range file {
		for j := 0; j < len(line); j++ {
			if unicode.IsDigit(rune(line[j])) {
				var nPart = Part{int(line[j] - '0'), Point{j, i}, 1}

				for j++; j < len(line) && unicode.IsDigit(rune(line[j])); j++ {
					nPart.len = 1 + j - nPart.start.x
					nPart.number = nPart.number*10 + int(line[j]-'0')
				}
				parts = append(parts, nPart)
			}
			if j < len(line) && filtro(rune(line[j])) {
				simbles = append(simbles, Point{j, i})
			}

		}
	}
	return parts, simbles
}
