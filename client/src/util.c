#include "util.h"
#include <stdlib.h>

struct LinkedList* linkedListPrepend(struct LinkedList* destination, void* value) {
	struct LinkedList* new = malloc(sizeof(struct LinkedList));
	new->value = value;
	new->next = destination;
	return new;
}

struct LinkedList* linkedListAppend(struct LinkedList* destination, void* value) {
	struct LinkedList* new = malloc(sizeof(struct LinkedList));
	new->value = value;
	new->next = 0;
	if (destination) {
		while (destination->next) {
			destination = destination->next;
		}
		destination->next = new;
	}
	return new;
}

struct LinkedList* linkedListInsert(struct LinkedList* after, void* value) {
	struct LinkedList* new = malloc(sizeof(struct LinkedList));
	new->value = value;
	if (after) {
		new->next = after->next;
		after->next = new;
	} else {
		new->next = 0;
	}
	return new;
}

void* linkedListRemove(struct LinkedList* parent) {
	struct LinkedList* target = parent->next;
	if (!target) {
		return 0;
	}
	parent->next = target->next;
	void* data = target->value;
	free(target);
	return data;
}

void linkedListDispose(struct LinkedList* list) {
	struct LinkedList* next;
	while (list) {
		next = list;
		free(list);
		list = next;
	}
}

unsigned int linkedListLength(struct LinkedList* list) {
	unsigned int result = 0;
	while (list) {
		list = list->next;
		result++;
	}
	return result;
}
