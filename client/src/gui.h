#ifndef H_GUI
#define H_GUI
#include <SDL2/SDL_rect.h>
#include <SDL2/SDL_surface.h>
#include "input.h"
#include "util.h"

#define GUI_ELEMENT_FLAG_INVALIDATED 1

#define GUI_ELEMENT_TYPE_CONTAINER 1
#define GUI_ELEMENT_TYPE_TEXT 2
#define GUI_ELEMENT_TYPE_TEXTFIELD 3
#define GUI_ELEMENT_TYPE_BUTTON 4
#define GUI_ELEMENT_TYPE_PROXY 5

struct GuiElement {
	SDL_Rect position;
	char flags;
	char type;
	void* data;
};

struct InputTextfieldData {
	char* chars;
	unsigned char length;
};

struct InputButtonData {
	char* text;
	void (*onPress)(struct InputField*);
};

struct GuiProxyData {
	void* (*onCreate)(struct GuiProxyData*);
	void (*onDispose)(struct GuiProxyData*);
	void (*onDraw)(struct GuiElement*, SDL_Surface*); 
};

struct GuiDataContainerType {
	struct LinkedList* children;
	SDL_Surface* surface;
	unsigned short w;
	unsigned short h;
};

struct GuiDataButtonType {
	char* text;
	struct InputField* inputField;
};

struct GuiDataProxyType {
	struct GuiProxyData* proxy;
	void* data;
};

struct GuiElement* createGuiElement(SDL_Rect position, char flags, char type, void* data);

void disposeGuiElement(struct GuiElement* element);

void guiContainerLink(struct GuiElement* container, struct GuiElement* child);

void guiContainerUnlink(struct GuiElement* container, struct GuiElement* child);

void drawGuiElement(struct GuiElement* element, SDL_Surface* surface);

#endif
