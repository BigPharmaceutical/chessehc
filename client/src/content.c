#include "content.h"
#include "graphics.h"

GuiElement* currentContainer;

void initContent() {
	SDL_Rect temp;
	temp.x = 10;
	temp.y = 10;
	temp.w = 24;
	temp.h = 36;
	
	GuiElement* a = createGuiElement(*fullRect, 0, GUI_ELEMENT_TYPE_CONTAINER, 0);
	GuiElement* b = createGuiElement(temp, 0, GUI_ELEMENT_TYPE_TEXT, "hello world!");
	temp.y = 50;
	GuiElement* c = createGuiElement(temp, 0, GUI_ELEMENT_TYPE_TEXT, "o4it[p4'34");
	temp.y = 100;
	GuiElement* d = createGuiElement(temp, 0, GUI_ELEMENT_TYPE_TEXTFIELD, (void*) 32);
	guiContainerLink(a, b);
	guiContainerLink(a, c);
	guiContainerLink(a, d);
	
	currentContainer = a;
}

void disposeContent() {


}
