all: day1 day2 day3 day4 day5 day6 day7 day8 day9 day10 day11 day12
day1:
	rustc ./solutions/day1.rs --out-dir ./bins/
day2:
	rustc ./solutions/day2.rs --out-dir ./bins/
day3:
	rustc ./solutions/day3.rs --out-dir ./bins/
day4:
	rustc ./solutions/day4.rs --out-dir ./bins/
day5:
	rustc ./solutions/day5.rs --out-dir ./bins/
day6:
	rustc ./solutions/day6.rs --out-dir ./bins/
day7:
	go build -o ./bins/ ./solutions/day7.go
day8:
	rustc ./solutions/day8.rs --out-dir ./bins/
day9:
	rustc ./solutions/day9.rs --out-dir ./bins/
day10:
	rustc ./solutions/day10.rs --out-dir ./bins/
day11:
	rustc ./solutions/day11.rs --out-dir ./bins/
day12:
	rustc ./solutions/day12.rs --out-dir ./bins/
clean:
	rm -rf ./bins
