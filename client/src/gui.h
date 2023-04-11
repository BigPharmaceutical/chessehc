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

struct GuiInfoButton {
	char* text;
	void (*onPress)(struct InputField*);
};

struct GuiDataProxy {
	struct GuiInfoProxy* proxy;
	void* data;
};

struct GuiInfoProxy {
	void* (*onCreate)(struct GuiInfoProxy*);
	void (*onDispose)(struct GuiDataProxy*);
	void (*onDraw)(struct GuiElement*, SDL_Surface*); 
	char (*toggleInputs)(struct GuiElement* element, char newStatus);
};

struct GuiDataContainer {
	struct LinkedList* children;
	SDL_Surface* surface;
	unsigned short w;
	unsigned short h;
	struct PixelRGB color;
};

struct GuiDataButton {
	char* text;
	struct InputField* inputField;
};

struct GuiElement* createGuiElement(SDL_Rect position, char flags, char type, void* data);

void disposeGuiElement(struct GuiElement* element);

void guiContainerLink(struct GuiElement* container, struct GuiElement* child);

void guiContainerUnlink(struct GuiElement* container, struct GuiElement* child);

void guiContainerDye(struct GuiElement* container, struct PixelRGB color);

void drawGuiElement(struct GuiElement* element, SDL_Surface* surface);

void guiTreeToggleInputs(struct GuiElement* tree, char newStatus);

struct GuiElement* guiSwitchInputs(struct GuiElement* from, struct GuiElement* to);

#endif
