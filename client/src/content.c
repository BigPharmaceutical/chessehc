#include "content.h"
#include "graphics.h"
#include "input.h"
#include "game.h"


GuiElement* currentContainer;

void tempPress(InputField* field) {
	printf("pressed!\r\n");
}

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
	temp.x = 100;
	temp.y = 200;
	temp.w = 48;
	temp.h = 72;
	InputButtonData bd;
	bd.text = "button!";
	bd.onPress = *tempPress;
	GuiElement* e = createGuiElement(temp, 0, GUI_ELEMENT_TYPE_BUTTON, &bd);
	guiContainerLink(a, b);
	guiContainerLink(a, c);
	guiContainerLink(a, d);
	guiContainerLink(a, e);
	
	GuiElement* g = createGuiElement(*fullRect, 0, GUI_ELEMENT_TYPE_CONTAINER, 0);
	ChessGame* game = createGame();
	gameContainerLink(g, game);	
	guiContainerLink(a, g);

	currentContainer = a;
}

void disposeContent() {


}
