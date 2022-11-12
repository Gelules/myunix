CFLAGS = -std=c99 -pedantic -Wall -Wextra -Werror -Wvla
OBJ = src/cd.o
BIN = src/cd
TARGET = cd

.PHONY: doc

all: $(BIN)
	mv $< $(TARGET)

$(BIN): $(OBJ)

check: all
	@echo Shell or Criterion?

doc:
	doxygen Doxyfile

clean:
	$(RM) $(OBJ) $(BIN) $(TARGET) -r doc/
