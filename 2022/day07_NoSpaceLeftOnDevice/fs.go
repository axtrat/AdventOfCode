package main

import "fmt"

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
