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

typedef struct GuiElement {
	SDL_Rect position;
	char flags;
	char type;
	void* data;
} GuiElement;

typedef struct InputTextfieldData {
	char* chars;
	unsigned char length;
} InputTextfieldData;

typedef struct InputButtonData {
	char* text;
	void (*onPress)(InputField*);
} InputButtonData;

typedef struct GuiProxyData {
	void* (*onCreate)(struct GuiProxyData*);
	void (*onDispose)(struct GuiProxyData*);
	void (*onDraw)(GuiElement*, SDL_Surface*); 
} GuiProxyData;


typedef struct GuiDataContainerType {
	LinkedList* children;
	SDL_Surface* surface;
	short w;
	short h;
} GuiDataContainerType;

typedef struct GuiDataButtonType {
	char* text;
	InputField* inputField;
} GuiDataButtonType;

typedef struct GuiDataProxyType {
	GuiProxyData* proxy;
	void* data;
} GuiDataProxyType;

GuiElement* createGuiElement(SDL_Rect position, char flags,  char type, void* data);

void disposeGuiElement(GuiElement* element);

void guiContainerLink(GuiElement* container, GuiElement* child);

void guiContainerUnlink(GuiElement* container, GuiElement* child);

void drawGuiElement(GuiElement* element, SDL_Surface* surface);

#endif
