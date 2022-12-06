all: day1 day2 day3 day4 day5 day6
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
clean:
	rm -rf ./bins
