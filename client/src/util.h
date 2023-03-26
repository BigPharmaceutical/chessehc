#ifndef H_UTIL
#define H_UTIL

typedef struct LinkedList {
	void* value;
	struct LinkedList* next;
} LinkedList;

LinkedList* linkedListPrepend(LinkedList* destination, void* value);

LinkedList* linkedListAppend(LinkedList* destination, void* value);

LinkedList* linkedListInsert(LinkedList* after, void* value);

void* linkedListRemove(LinkedList* parent);

void linkedListDispose(LinkedList* list);

unsigned int linkedListLength(LinkedList* list);

#endif
