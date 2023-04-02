build: src/* lib/*/* font/fontblob.c
	gcc -g -o chessehc src/* font/fontblob.c lib/mongoose/*.c -Ilib/mongoose `sdl2-config --cflags --libs` -lm

font/fontblob.c: font/raw/*.txt
	cd font && groovy buildFont.groovy && cd ..

clean:
	rm ./chessehc
	rm ./font/fontblob.c

