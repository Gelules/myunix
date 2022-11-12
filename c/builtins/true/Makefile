CFLAGS = -std=c99 -pedantic -Wall -Wextra -Werror -Wvla
OBJ = src/true.o
BIN = src/true
TARGET = true

all: $(BIN)
	mv $< $(TARGET)

$(BIN): $(OBJ)

check:
	@echo Would you please write the testuite?

clean:
	$(RM) $(OBJ) $(BIN) $(TARGET)
