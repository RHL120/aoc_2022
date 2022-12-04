all:
	rustc ./day1.rs --out-dir ./bins/
	rustc ./day2.rs --out-dir ./bins/
	rustc ./day3.rs --out-dir ./bins/
clean:
	rm -rf ./bins
