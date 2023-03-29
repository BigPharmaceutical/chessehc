#include "content.h"
#include "graphics.h"
#include "input.h"
#include "game.h"


struct GuiElement* currentContainer;

void tempPress(struct InputField* field) {
	printf("pressed!\r\n");
}

void initContent() {
	SDL_Rect temp;
	temp.x = 10;
	temp.y = 10;
	temp.w = 24;
	temp.h = 36;
	
	struct GuiElement* a = createGuiElement(*fullRect, 0, GUI_ELEMENT_TYPE_CONTAINER, 0);
	struct GuiElement* b = createGuiElement(temp, 0, GUI_ELEMENT_TYPE_TEXT, "hello world!");
	temp.y = 50;
	struct GuiElement* c = createGuiElement(temp, 0, GUI_ELEMENT_TYPE_TEXT, "o4it[p4'34");
	temp.y = 100;
	struct GuiElement* d = createGuiElement(temp, 0, GUI_ELEMENT_TYPE_TEXTFIELD, (void*) 32);
	temp.x = 100;
	temp.y = 200;
	temp.w = 48;
	temp.h = 72;
	struct InputButtonData bd;
	bd.text = "button!";
	bd.onPress = *tempPress;
	struct GuiElement* e = createGuiElement(temp, 0, GUI_ELEMENT_TYPE_BUTTON, &bd);
	guiContainerLink(a, b);
	guiContainerLink(a, c);
	guiContainerLink(a, d);
	guiContainerLink(a, e);
	
	struct GuiElement* g = createGuiElement(*fullRect, 0, GUI_ELEMENT_TYPE_CONTAINER, 0);
	struct ChessGame* game = createGame();
	gameContainerLink(g, game);	
	guiContainerLink(a, g);

	currentContainer = a;
}

void disposeContent() {


}
