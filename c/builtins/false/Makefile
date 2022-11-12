CFLAGS = -std=c99 -pedantic -Wall -Wextra -Werror -Wvla
OBJ = src/false.o
BIN = src/false
TARGET = false

.PHONY: doc

all: $(BIN)
	mv $< $(TARGET)

$(BIN): $(OBJ)

check: all
	./tests/testsuite.sh

doc:
	doxygen Doxyfile

clean:
	$(RM) $(OBJ) $(BIN) $(TARGET) -r doc/
