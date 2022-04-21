CC := gcc 
CFLAGS := -Wall -Werror -Wextra -pedantic -std=c17 -O2

all: 
	$(CC) $(CFLAGS) -o client client.c -L../../target/debug -lcalling_rust_from_c

clean:
	rm -rf *.o client