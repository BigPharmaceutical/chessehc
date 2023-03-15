#ifndef H_GUI
#define H_GUI
#include <SDL2/SDL_rect.h>
#include <SDL2/SDL_surface.h>

#define GUI_ELEMENT_TYPE_CONTAINER 1
#define GUI_ELEMENT_TYPE_TEXT 2
#define GUI_ELEMENT_TYPE_TEXTFIELD 3


typedef struct GuiElement {
	SDL_Rect position;
	unsigned char flags;
	char type;
	void* data;
} GuiElement;

typedef struct InputTextData {
	char* chars;
	unsigned char length;
} InputTextData;

GuiElement* createGuiElement(SDL_Rect position, char flags,  char type, void* data);

void disposeGuiElement(GuiElement* element);


void guiContainerLink(GuiElement* container, GuiElement* child);

void guiContainerUnlink(GuiElement* container, GuiElement* child);

SDL_Surface* guiContainerSurface(GuiElement* container);

void guiContainerInvalidate(GuiElement* container);

#endif
