#ifndef H_UTIL
#define H_UTIL

struct LinkedList {
	void* value;
	struct LinkedList* next;
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
