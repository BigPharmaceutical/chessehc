SOURCES := $(wildcard src/*.c lib/*/*.c)
OBJECTS := $(addprefix objs/,$(patsubst %.c,%.o,$(SOURCES)))

build: objs/font/fontblob.o $(OBJECTS)
	gcc -o chessehc $^ `sdl2-config --cflags --libs`

objs/%.o: %.c
	mkdir -p objs/$(dir $<)
	gcc -g -o objs/$(<:.c=.o) -c $< -Ilib/mongoose -Ilib/ed25519 -Isrc `sdl2-config --cflags --libs`

font/fontblob.c: font/raw/*.txt
	cd font && groovy buildFont.groovy && cd ..

run: build
	./chessehc

clean:
	rm -f ./chessehc
	rm -f ./font/fontblob.c
	rm -rf ./objs
	rm -f ./chessehc.key
