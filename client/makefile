build: src/* lib/*/* font/fontblob.c
	gcc -g -o chessehc src/*.c font/*.c -Isrc/*.h -Ilib/*/*.h `sdl2-config --cflags --libs` -lm

font/fontblob.c: font/raw/*.txt
	cd font && groovy buildFont.groovy && cd ..

clean:
	rm ./chessehc
	rm ./font/fontblob.c

