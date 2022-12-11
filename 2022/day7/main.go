package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

type El interface {
	Name() string
	Size() int
	IsFile() bool
	fmt.Stringer
}

type File struct {
	name string
	size int
}

func (file File) IsFile() bool {
	return true
}

func (file File) Name() string {
	return file.name
}

func (file File) Size() int {
	return file.size
}

func (file File) String() string {
	return file.name + " (file, size=" + fmt.Sprint(file.size) + ")"
}

type Dir struct {
	name   string
	size   int
	subDir []El
}

func (dir Dir) IsFile() bool {
	return false
}

func (dir Dir) Name() string {
	return dir.name
}

func (dir *Dir) Size() int {
	if dir.size != -1 {
		return dir.size
	}

	size := 0
	for _, d := range dir.subDir {
		size += d.Size()
	}

	dir.size = size
	return size
}

func (dir Dir) getSubElem(name string) *Dir {
	for _, el := range dir.subDir {
		if el.Name() == name && !el.IsFile() {
			return el.(*Dir)
		}
	}
	return nil
}

func contains(dirs []El, name string) bool {
	for _, el := range dirs {
		if el.Name() == name {
			return true
		}
	}
	return false
}

func (dir *Dir) Add(el El) {
	if !contains(dir.subDir, el.Name()) {
		dir.subDir = append(dir.subDir, el)
	}
}

func stringRec(dir *Dir, s string) string {
	str := s + "- " + dir.name + " (dir)\n"
	for _, sub := range dir.subDir {
		str += s + sub.String() + "\n"
	}
	return str
}

func (d Dir) String() string {
	res := stringRec(&d, "  ")
	return res[:len(res)-1]
}

func parseInput(file []string) *Dir {
	var stack []*Dir = make([]*Dir, 0)
	stack = append(stack, &Dir{name: "/", size: -1})

	for _, line := range file[1:] {
		occ := strings.Split(line, " ")
		switch occ[0] {
		case "$":
			switch {
			case occ[1] == "ls":
			case occ[2] == "..":
				stack = stack[:len(stack)-1]
			default:
				stack = append(stack, stack[len(stack)-1].getSubElem(occ[2]))
			}
		case "dir":
			stack[len(stack)-1].Add(&Dir{name: occ[1], size: -1})
		default:
			fsize, _ := strconv.Atoi(occ[0])
			stack[len(stack)-1].Add(&File{occ[1], fsize})
		}
	}

	return stack[0]
}

func filterSizes(el El, condition func(int) bool) []int {
	var res []int = make([]int, 0)

	if el.IsFile() {
		return res
	}

	if condition(el.Size()) {
		res = append(res, el.Size())
	}

	for _, d := range el.(*Dir).subDir {
		res = append(res, filterSizes(d, condition)...)
	}
	return res
}

func min(slice []int) int {
	min := slice[0]

	for i := 1; i < len(slice); i++ {
		if slice[i] < min {
			min = slice[i]
		}
	}

	return min
}

func part1(file []string) {
	root := parseInput(file)

	res := filterSizes(root, func(i int) bool { return i <= 100000 })
	size := 0
	for _, v := range res {
		size += v
	}
	fmt.Println(size)
}

func part2(file []string) {
	root := parseInput(file)
	total := root.Size()

	res := filterSizes(root, func(i int) bool { return 70000000-(total-i) >= 30000000 })

	fmt.Println(min(res))
}

func main() {
	var lines []string = input()
	var start time.Time

	start = time.Now()
	part1(lines)
	fmt.Printf("--- %v ---\n", time.Since(start))

	start = time.Now()
	part2(lines)
	fmt.Printf("--- %v ---\n", time.Since(start))
}

func input() []string {
	file, _ := os.Open(os.Args[1] + ".txt")
	defer file.Close()

	sc := bufio.NewScanner(file)
	sc.Split(bufio.ScanLines)

	lines := make([]string, 0)
	for sc.Scan() {
		lines = append(lines, sc.Text())
	}

	return lines
}
