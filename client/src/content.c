#include "content.h"
#include "graphics.h"
#include "input.h"
#include "game.h"
#include "net.h"

struct GuiElement* currentContainer;

struct GuiElement* containerMenu;
struct GuiElement* containerGame;

struct GuiElement* inputIp;
struct GuiElement* inputCode;

void buttonPlayPressed(struct InputField* field) {
	struct InputTextfieldData* data = field->data;
	netConnect(data->chars);
}

void initContent() {
	SDL_Rect r;
	struct PixelRGB c;


	// // Menu Container
	containerMenu = createGuiElement(*fullRect, 0, GUI_ELEMENT_TYPE_CONTAINER, 0);
	struct GuiElement* inpContainer = createGuiElement(*fullRect, 0, GUI_ELEMENT_TYPE_CONTAINER, 0);
	c.r = 125;
	c.g = 125;
	c.b = 255;
	guiContainerDye(inpContainer, c);
	guiContainerLink(containerMenu, inpContainer);
	// Big title
	r.x = 28;
	r.y = 60;
	r.w = 72;
	r.h = 72;
	guiContainerLink(containerMenu, createGuiElement(r, 0, GUI_ELEMENT_TYPE_TEXT, "CHESSEHC"));
	// TAB hint
	r.x = 5;
	r.y = 5;
	r.w = 150;
	r.h = 16;
	r.w = 8;
	guiContainerLink(inpContainer, createGuiElement(r, 0, GUI_ELEMENT_TYPE_TEXT, "[TAB] to select"));
	// IP input
	r.x = 60;
	r.y = 180;
	r.w = 24;
	r.h = 36;
	guiContainerLink(containerMenu, createGuiElement(r, 0, GUI_ELEMENT_TYPE_TEXT, "IP:"));
	r.x = 140;
	inputIp = createGuiElement(r, 0, GUI_ELEMENT_TYPE_TEXTFIELD, (void*) 21 /* lovely */);
	guiContainerLink(inpContainer, inputIp);
	// Game code input
	r.x = 60;
	r.y = 220;
	guiContainerLink(containerMenu, createGuiElement(r, 0, GUI_ELEMENT_TYPE_TEXT, "Code:"));
	r.x = 190;
	inputCode = createGuiElement(r, 0, GUI_ELEMENT_TYPE_TEXTFIELD, (void*) 6);
	guiContainerLink(inpContainer, inputCode);
	// Name input
	r.x = 60;
	r.y = 260;
	guiContainerLink(containerMenu, createGuiElement(r, 0, GUI_ELEMENT_TYPE_TEXT, "Name:"));
	r.x = 190;
	inputCode = createGuiElement(r, 0, GUI_ELEMENT_TYPE_TEXTFIELD, (void*) 16);
	guiContainerLink(inpContainer, inputCode);
	// Join button
	r.x = 220;
	r.y = 360;
	r.w = 40;
	r.h = 60;
	struct InputButtonData bdJoin;
	bdJoin.text = "PLAY";
	bdJoin.onPress = &buttonPlayPressed;
	guiContainerLink(inpContainer, createGuiElement(r, 0, GUI_ELEMENT_TYPE_BUTTON, &bdJoin));

	currentContainer = containerMenu;
}

void disposeContent() {


}
