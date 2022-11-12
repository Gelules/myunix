CFLAGS = -std=c99 -pedantic -Wall -Wextra -Werror -Wvla
OBJ = src/true.o
BIN = src/true
TARGET = true

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
