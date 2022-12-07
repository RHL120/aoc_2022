//This solution is very bad but I am not that smart so no blame
package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type fileStructure struct {
	name     string
	parent   *fileStructure
	children []*fileStructure
	is_dir   bool
	size     int
}

func part1_solve(input string) (*fileStructure, error) {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	ret := new(fileStructure)
	listing := false
	for idx, line := range lines {
		_ = idx
		if strings.HasPrefix(line, "$ cd ") {
			listing = false
			arg := strings.TrimPrefix(line, "$ cd ")
			if arg == "/" {
				//cd / can occur only at the beginning
				ret.name = "/"
				ret.parent = nil
				ret.children = make([]*fileStructure, 0)
				ret.is_dir = true
			} else if arg == ".." {
				if ret.parent != nil {
					ret = ret.parent
				} else {
					return ret, errors.New("the current directory has no parent")
				}
			} else {
				found := false
				for _, child := range ret.children {
					if child.name == arg {
						if !child.is_dir {
							return ret, fmt.Errorf("%s: is not a directory", arg)
						}
						ret = child
						found = true
					}
				}
				if !found {
					return ret, fmt.Errorf("%s: No directory", arg)
				}
			}
		} else if strings.HasPrefix(line, "$ ls") {
			listing = true
		} else if listing {
			var args = strings.Split(line, " ")
			size := 0
			is_dir := false
			var children []*fileStructure = nil
			if len(args) != 2 {
				return ret, fmt.Errorf("line: %d, %s: is not a valid ls entry", idx, line)
			}
			if args[0] == "dir" {
				is_dir = true
			} else {
				var err error
				size, err = strconv.Atoi(args[0])
				if err != nil {
					return ret, fmt.Errorf("%s has an invalid size", line)
				}
				parent := ret
				for {
					if parent == nil {
						break
					}
					parent.size += size
					parent = parent.parent

				}
			}
			ret.children = append(ret.children, &fileStructure{

				name:     args[1],
				parent:   ret,
				children: children,
				is_dir:   is_dir,
				size:     size,
			})
		}
	}
	for {
		if ret.parent == nil {
			break
		}
		ret = ret.parent
	}
	return ret, nil
}

func traverse_pt1(f *fileStructure, size *int) {
	if f.is_dir {
		if f.size <= 100000 {
			*size += f.size
		}
		for _, file := range f.children {
			traverse_pt1(file, size)
		}
	}
}

func traverse_pt2(f *fileStructure, wanted_size int, min *int) {
	if f.is_dir {
		if f.size >= wanted_size && f.size < *min {
			*min = f.size
		}
		for _, i := range f.children {
			traverse_pt2(i, wanted_size, min)
		}
	}
}

func solve_pt2(f *fileStructure) int {
	var size int = f.size
	traverse_pt2(f, f.size-40000000, &size)
	return size
}

func main() {
	input, err := ioutil.ReadFile("./inputs/day7")
	if err != nil {
		fmt.Println("Failed to read input")
	}
	solution, err := part1_solve(string(input[:]))
	if err != nil {
		fmt.Printf("error: %v\n", err)
		return
	}
	var size = 0
	traverse_pt1(solution, &size)
	fmt.Println("The solution to part1 is: ", size)
	size = 0
	fmt.Println("The solution to part2 is: ", solve_pt2(solution))
}
