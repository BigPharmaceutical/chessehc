#ifndef H_UTIL
#define H_UTIL

struct LinkedList {
	void* value;
	struct LinkedList* next;
};

// So it seems that SDL is rather inconsistent with the order? Alpha is always on the other side than set in masks.
struct PixelARGB {
	unsigned char a;
	unsigned char b;
	unsigned char g;
	unsigned char r;
};

struct PixelRGB {
	unsigned char r;
	unsigned char g;
	unsigned char b;
};

struct LinkedList* linkedListPrepend(struct LinkedList* destination, void* value);

struct LinkedList* linkedListAppend(struct LinkedList* destination, void* value);

struct LinkedList* linkedListInsert(struct LinkedList* after, void* value);

void* linkedListRemove(struct LinkedList* parent);

void linkedListDispose(struct LinkedList* list);

unsigned int linkedListLength(struct LinkedList* list);

#endif
