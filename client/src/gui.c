#include "gui.h"
#include "font.h"
#include "input.h"
#include "graphics.h"
#include <stdlib.h>
#include <SDL2/SDL.h>

struct GuiDataContainerType* createGuiDataContainer(SDL_Rect* param);
void disposeGuiDataContainer(struct GuiDataContainerType* data);
void drawGuiElementContainer(struct GuiElement* text, SDL_Surface* surface);
SDL_Surface* guiContainerSurface(struct GuiElement* container);

void* createGuiDataText(char* param);
void disposeGuiDataText(void* data);
void drawGuiElementText(struct GuiElement* text, SDL_Surface* surface);

void* createGuiDataTextfield(char param);
void disposeGuiDataTextfield(void* data);
void drawGuiElementTextfield(struct GuiElement* text, SDL_Surface* surface);

struct GuiDataButtonType* createGuiDataButton(struct InputButtonData* param);
void disposeGuiDataButton(struct GuiDataButtonType* data);
void drawGuiElementButton(struct GuiElement* element, SDL_Surface* surface);

struct GuiDataProxyType* createGuiDataProxy(struct GuiProxyData* param);
void disposeGuiDataProxy(struct GuiDataProxyType* data);
void drawGuiElementProxy(struct GuiElement* element, SDL_Surface* surface);


//////// General ////////

struct GuiElement* createGuiElement(SDL_Rect area, char flags, char type, void* param) {
	struct GuiElement* element = malloc(sizeof(struct GuiElement));
	element->position = area;
	element->flags = flags | GUI_ELEMENT_FLAG_INVALIDATED;
	element->type = type;

	switch (type) {
		case GUI_ELEMENT_TYPE_CONTAINER:
			element->data = createGuiDataContainer(&area);
			break;	

		case GUI_ELEMENT_TYPE_TEXT:
			element->data = createGuiDataText(param);
			break;

		case GUI_ELEMENT_TYPE_TEXTFIELD: {
			struct InputField* field = createGuiDataTextfield((long) param);
			field->guiElementFlags = &element->flags;
			element->data = field;
		} break;
		
		case GUI_ELEMENT_TYPE_BUTTON: {
			struct GuiDataButtonType* data = createGuiDataButton(param);
			data->inputField->guiElementFlags = &element->flags;
			element->data = data;
		} break;

		case GUI_ELEMENT_TYPE_PROXY:
			element->data = createGuiDataProxy(param);
			break;
	}
	return element;
}

void disposeGuiElement(struct GuiElement* element) {
	switch (element->type) {
		case GUI_ELEMENT_TYPE_CONTAINER:
			disposeGuiDataContainer(element->data);
			break;

		case GUI_ELEMENT_TYPE_TEXT:
			disposeGuiDataText(element->data);
			break;

		case GUI_ELEMENT_TYPE_TEXTFIELD:
			disposeGuiDataTextfield(element->data);
			break;

		case GUI_ELEMENT_TYPE_BUTTON:
			disposeGuiDataButton(element->data);
			break;

		case GUI_ELEMENT_TYPE_PROXY:
			disposeGuiDataProxy(element->data);
			break;
	}
	free(element);
}

void drawGuiElement(struct GuiElement* element, SDL_Surface* surface) {
	switch (element->type) {
		case GUI_ELEMENT_TYPE_CONTAINER:
			drawGuiElementContainer(element, surface);
			break;

		case GUI_ELEMENT_TYPE_TEXT:
			drawGuiElementText(element, surface);
			break;
	
		case GUI_ELEMENT_TYPE_TEXTFIELD:
			drawGuiElementTextfield(element, surface);
			break;

		case GUI_ELEMENT_TYPE_BUTTON:
			drawGuiElementButton(element, surface);
			break;

		case GUI_ELEMENT_TYPE_PROXY:
			drawGuiElementProxy(element, surface);
			break;
	}

	element->flags &= ~GUI_ELEMENT_FLAG_INVALIDATED;
}

//////// Container ////////

struct GuiDataContainerType* createGuiDataContainer(SDL_Rect* param) {
	struct GuiDataContainerType* new = malloc(sizeof(struct GuiDataContainerType));
	new->surface = SDL_CreateRGBSurface(0, param->w, param->h, 32, 0xFF000000, 0x00FF0000, 0x0000FF00, 0x000000FF);
	SDL_SetSurfaceBlendMode(new->surface, SDL_BLENDMODE_BLEND);
	new->children = 0;
	new->w = param->w;
	new->h = param->h;
	
	new->color.r = 255;
	new->color.g = 255;
	new->color.b = 255;
	return new;
}

void disposeGuiDataContainer(struct GuiDataContainerType* data) {
	SDL_FreeSurface(data->surface);
	free(data->surface);
	linkedListDispose(data->children);
	free(data);
}

void drawGuiElementContainer(struct GuiElement* element, SDL_Surface* surface) {
	struct GuiDataContainerType* data = element->data;
	SDL_Rect source;
	source.x = 0;
	source.y = 0;
	source.w = data->w;
	source.h = data->h;
	SDL_Surface* containerSurface = guiContainerSurface(element);	
	SDL_BlitSurface(containerSurface, &source, surface, &element->position);	
}

void guiContainerLink(struct GuiElement* container, struct GuiElement* child) {
	struct GuiDataContainerType* data = container->data;
	data->children = linkedListPrepend(data->children, child);
}

void guiContainerUnlink(struct GuiElement* container, struct GuiElement* child) {
	struct GuiDataContainerType* data = container->data;
	struct LinkedList* previous = 0;
	struct LinkedList* current = data->children;
	
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

void guiContainerDye(struct GuiElement* container, struct PixelRGB color) {
	((struct GuiDataContainerType*) container->data)->color = color;
}

char isElementTreeInvalidated(struct GuiElement* element) {
	if (element->flags & GUI_ELEMENT_FLAG_INVALIDATED) {
		return 1;
	}
	
	if (element->type == GUI_ELEMENT_TYPE_CONTAINER) {
		struct GuiDataContainerType* data = element->data;
		struct LinkedList* target = data->children;
		while (target) {
			if (isElementTreeInvalidated(target->value)) {
				return 1;
			}
			target = target->next;
		}
	}

	return 0;
}


SDL_Surface* guiContainerSurface(struct GuiElement* container) {
	struct GuiDataContainerType* data = container->data;

	if (isElementTreeInvalidated(container)) {
		struct LinkedList* target = data->children;
		SDL_FillRect(data->surface, 0, 0);
		while (target) {
			drawGuiElement(target->value, data->surface);
			target = target->next;
		}
		if (data->color.r != 255 || data->color.g != 255 || data->color.b != 255) {
			graphicsDyeSurface(data->surface, &data->color);
		}
	}
	return data->surface;
}

//////// Text ////////
void* createGuiDataText(char* param) {
	unsigned char length = 0;
	while (param[length++] != '\0') {}
	char* data = malloc(length);
	memcpy(data, param, length); 
	return data;
}

void disposeGuiDataText(void* data) {
	free(data);
}

void drawGuiElementText(struct GuiElement* element, SDL_Surface* surface) {
	drawString(surface, element->data, &(element->position), 1, 0);
}


//////// Text Field ////////

void* createGuiDataTextfield(char param) {
	return createInputText(param, INPUT_FLAGS_ENABLED | INPUT_FLAGS_SELECTABLE);
}

void disposeGuiDataTextfield(void* data) {
	disposeOneInputByField(data);
}

void drawGuiElementTextfield(struct GuiElement* element, SDL_Surface* surface) {
	struct InputField* field = element->data;
	struct InputTextfieldData* fieldData = field->data;
	if (field == getInputFocused()) {
		SDL_Rect backRect;
		backRect.x = element->position.x - 1;
		backRect.y = element->position.y - 1;
		backRect.w = (1 + element->position.w) * fieldData->length + 2;
		backRect.h = element->position.h + 2;
		graphicsDrawRectOutline(surface, &backRect, 2, 0x101010FF);
	}
	drawString(surface, fieldData->chars, &(element->position), 1, fieldData->length);
}


//////// Button ////////

struct GuiDataButtonType* createGuiDataButton(struct InputButtonData* param) {
	struct GuiDataButtonType* data = malloc(sizeof(struct GuiDataButtonType));
	data->inputField = createInputButton(param->onPress, INPUT_FLAGS_ENABLED | INPUT_FLAGS_SELECTABLE);

	unsigned char length = 0;
	while (param->text[length++] != '\0') {}
	data->text = malloc(length);
	memcpy(data->text, param->text, length); 
	return data;
}

void disposeGuiDataButton(struct GuiDataButtonType* data) {
	disposeOneInputByField(data->inputField);
	free(data);
}

void drawGuiElementButton(struct GuiElement* element, SDL_Surface* surface) {
	struct GuiDataButtonType* data = element->data;
	if (data->inputField == getInputFocused()) {
		unsigned char length = 0;
		for(;(data->text)[length];length++);
		SDL_Rect backRect;
		backRect.x = element->position.x - 1;
		backRect.y = element->position.y - 1;
		backRect.w = (1 + element->position.w) * length + 2;
		backRect.h = element->position.h + 2;
		graphicsDrawRectOutline(surface, &backRect, 2, 0x101010FF);
	}
	drawString(surface, data->text, &(element->position), 1, 0);
}

//////// Proxy ////////

struct GuiDataProxyType* createGuiDataProxy(struct GuiProxyData* param) {
	struct GuiDataProxyType* data = malloc(sizeof(struct GuiDataProxyType));
	data->proxy = param;
	if (param->onCreate) {
		data->data = param->onCreate(param);
	}
	return data;
}

void disposeGuiDataProxy(struct GuiDataProxyType* data) {
	if (data->proxy->onDispose) {
		data->proxy->onDispose(data->data);
	}
	free(data->proxy);
	free(data);
}

void drawGuiElementProxy(struct GuiElement* element, SDL_Surface* surface) {
	struct GuiDataProxyType* data = element->data;
	if (data->proxy->onDraw) {
		data->proxy->onDraw(element, surface);
	}
}
