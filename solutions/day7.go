//This solution is very bad but I am not that smart so no blame
package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type fileStructure struct {
	parent   *fileStructure
	children map[string]*fileStructure
	isDir    bool
	size     int
}

func constructTree(input string) (*fileStructure, error) {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	ret := &fileStructure{
		parent:   nil,
		children: make(map[string]*fileStructure, 0),
		isDir:    true,
		size:     0,
	}
	cwd := ret
	listing := false
	for idx, line := range lines {
		if strings.HasPrefix(line, "$ cd ") {
			listing = false
			arg := strings.TrimPrefix(line, "$ cd ")
			if arg == "/" {
				cwd = ret
			} else if arg == ".." {
				if cwd.parent != nil {
					cwd = cwd.parent
				} else {
					return ret, fmt.Errorf("found cd .. on line %d but the directory has no parent", idx)
				}
			} else {
				var found bool
				cwd, found = cwd.children[arg]
				if !found {
					return ret, fmt.Errorf("line: %d, %s: No directory", idx, arg)
				}
			}
		} else if strings.HasPrefix(line, "$ ls") {
			listing = true
		} else if listing {
			var args = strings.Split(line, " ")
			size := 0
			isDir := false
			var children map[string]*fileStructure = nil
			if len(args) != 2 {
				return ret, fmt.Errorf("line: %d, %s: is not a valid ls entry", idx, line)
			}
			if args[0] == "dir" {
				children = make(map[string]*fileStructure)
				isDir = true
			} else {
				var err error
				size, err = strconv.Atoi(args[0])
				if err != nil {
					return ret, fmt.Errorf("line: %d, %s has an invalid size", idx, line)
				}
				for parent := cwd; parent != nil; parent = parent.parent {
					parent.size += size
				}
			}
			cwd.children[args[1]] = &fileStructure{
				parent:   cwd,
				children: children,
				isDir:    isDir,
				size:     size,
			}
		}
	}
	return ret, nil
}

func traversePt1(f *fileStructure, size *int) {
	if f.isDir {
		if f.size <= 100000 {
			*size += f.size
		}
		for _, file := range f.children {
			traversePt1(file, size)
		}
	}
}

func traversePt2(f *fileStructure, wantedSize int, min *int) {
	if f.isDir {
		if f.size >= wantedSize && f.size < *min {
			*min = f.size
		}
		for _, i := range f.children {
			traversePt2(i, wantedSize, min)
		}
	}
}

func solvePt2(f *fileStructure) int {
	var size int = f.size
	traversePt2(f, f.size-40000000, &size)
	return size
}

func main() {
	input, err := ioutil.ReadFile("./inputs/day7")
	if err != nil {
		fmt.Println("Failed to read input")
	}
	solution, err := constructTree(string(input[:]))
	if err != nil {
		fmt.Printf("error: %v\n", err)
		return
	}
	var size = 0
	traversePt1(solution, &size)
	fmt.Println("The solution to part1 is: ", size)
	size = 0
	fmt.Println("The solution to part2 is: ", solvePt2(solution))
}
