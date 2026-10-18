.PHONY: all run

run:
	./qmatrix

all:
	cc main.c -Wall -Wextra -Werror -g -lm -O3 -o qmatrix

