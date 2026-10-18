default: all run

all:
	cc main.c -Wall -Wextra -Werror -g -lm -O3 -o qmatrix

run:
	./qmatrix

clean:
	rm qmatrix
