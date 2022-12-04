all:
	rustc ./solutions/day1.rs --out-dir ./bins/
	rustc ./solutions/day2.rs --out-dir ./bins/
	rustc ./solutions/day3.rs --out-dir ./bins/
	rustc ./solutions/day4.rs --out-dir ./bins/
clean:
	rm -rf ./bins
