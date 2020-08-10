CFLAGS = -Werror -Weverything -std=c99

.PHONY: run clean

run: main
	./main

clean:
	rm -rf main

main: src/main.c src/lib.rs src/lib.h
	cargo build
	$(CC) $(CFLAGS) -l rust_from_c -L ../target/debug -o main src/main.c
