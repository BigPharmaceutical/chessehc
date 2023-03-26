#include "util.h"
#include <stdlib.h>

LinkedList* linkedListPrepend(LinkedList* destination, void* value) {
	LinkedList* new = malloc(sizeof(LinkedList));
	new->value = value;
	new->next = destination;
	return new;
}

LinkedList* linkedListAppend(LinkedList* destination, void* value) {
	LinkedList* new = malloc(sizeof(LinkedList));
	new->value = value;
	if (destination) {
		while (destination->next) {
			destination = destination->next;
		}
		destination->next = new;
	}
	return new;
}

LinkedList* linkedListInsert(LinkedList* after, void* value) {
	LinkedList* new = malloc(sizeof(LinkedList));
	new->value = value;
	if (after) {
		new->next = after->next;
		after->next = new;
	}
	return new;
}

void* linkedListRemove(LinkedList* parent) {
	LinkedList* target = parent->next;
	if (!target) {
		return 0;
	}
	parent->next = target->next;
	void* data = target->value;
	free(target);
	return data;
}

void linkedListDispose(LinkedList* list) {
	LinkedList* next;
	while (list) {
		next = list;
		free(list);
		list = next;
	}
}

unsigned int linkedListLength(LinkedList* list) {
	unsigned int result = 0;
	while (list) {
		list = list->next;
		result++;
	}
	return result;
}
