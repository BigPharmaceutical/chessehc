#include "gui.h"
#include "util.h"
#include "font.h"
#include <stdlib.h>
#include <SDL2/SDL.h>

typedef struct GuiDataContainerType {
	LinkedList* children;
	SDL_Surface* surface;
	char invalidated;
} GuiDataContainerType;
GuiDataContainerType* createGuiDataContainer(SDL_Rect* param);
void disposeGuiDataContainer(GuiDataContainerType* data);

void* createGuiDataText(char* param);
void disposeGuiDataText(void* data);
void drawGuiElementText(GuiElement* text, SDL_Surface* surface);

//////// General ////////

GuiElement* createGuiElement(SDL_Rect area, char flags, char type, void* param) {
	GuiElement* element = malloc(sizeof(GuiElement));
	element->position = area;
	element->flags = flags;
	element->type = type;

	switch (type) {
		case GUI_ELEMENT_TYPE_CONTAINER:
			element->data = createGuiDataContainer(&area);
			break;	

		case GUI_ELEMENT_TYPE_TEXT:
			element->data = createGuiDataText(param);
			break;
	}
	return element;
}

void disposeGuiElement(GuiElement* element) {
	switch (element->type) {
		case GUI_ELEMENT_TYPE_CONTAINER:
			disposeGuiDataContainer(element->data);
			break;

		case GUI_ELEMENT_TYPE_TEXT:
			disposeGuiDataText(element->data);
			break;
	}
	free(element);
}

void drawGuiElement(GuiElement* element, SDL_Surface* surface) {
	switch (element->type) {
		case GUI_ELEMENT_TYPE_TEXT:
			drawGuiElementText(element, surface);
			break;
	}

}

//////// Container ////////

GuiDataContainerType* createGuiDataContainer(SDL_Rect* param) {
	GuiDataContainerType* new = malloc(sizeof(GuiDataContainerType));
	new->surface = SDL_CreateRGBSurface(0, param->w, param->h, 32, 0x00FF0000, 0x0000FF00, 0x000000FF, 0xFF000000);
	new->children = 0;
	new->invalidated = 1;
	return new;
}

void disposeGuiDataContainer(struct GuiDataContainerType* data) {
	SDL_FreeSurface(data->surface);
	free(data->surface);
	linkedListDispose(data->children);
	free(data);
}

void guiContainerLink(GuiElement* container, GuiElement* child) {
	GuiDataContainerType* data = container->data;
	data->children = linkedListPrepend(data->children, child);
}

void guiContainerUnlink(GuiElement* container, GuiElement* child) {
	GuiDataContainerType* data = container->data;
	LinkedList* previous = 0;
	LinkedList* current = data->children;
	
	while (current) {
		if (current->value == child) {
			if (previous) {
				linkedListRemove(previous);
				current = previous->next;
			} else {
				data->children = current->next;
				free(current);
				current = data->children;
			}
		} else {
			previous = current;
			current = previous->next;
		}
	}
}

SDL_Surface* guiContainerSurface(GuiElement* container) {
	GuiDataContainerType* data = container->data;
	if (data->invalidated) {
		LinkedList* target = data->children;
		while (target) {
			drawGuiElement(target->value, data->surface);
			target = target->next;
		}
		data->invalidated = 0;
	}
	return data->surface;
}

void guiContainerInvalidate(GuiElement* container) {
	((GuiDataContainerType*) container->data)->invalidated = 1;
}

//////// Text ////////
void* createGuiDataText(char* param) {
	char length = 0;
	while (param[length++] != '\0') {}
	char* data = malloc(length);
	memcpy(data, param, length); 
	return data;
}

void disposeGuiDataText(void* data) {
	free(data);
}

void drawGuiElementText(GuiElement* element, SDL_Surface* surface) {
	drawString(surface, element->data, &(element->position), 1);
}
