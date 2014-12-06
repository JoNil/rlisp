CC = gcc
CCFLAGS = -ansi -g -fPIC -std=c99 -pedantic -pedantic-errors -Wall -Werror

all: ${OUT_DIR}/libmpc.a

${OUT_DIR}/mpc.o: mpc.c mpc.h
	$(CC) $(CCFLAGS) -c mpc.c -o ${OUT_DIR}/mpc.o

${OUT_DIR}/libmpc.a: ${OUT_DIR}/mpc.o
	ar crus ${OUT_DIR}/libmpc.a ${OUT_DIR}/mpc.o

.PHONY: all