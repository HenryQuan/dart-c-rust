CC=gcc

# build rust lib
lib:
	cd pointer; cargo build --release
