#include "net.h"

void netResponse(unsigned char opcode, void* data) {
	//todo
	switch (opcode) {
		case (NET_RES_OK_USERNAME):
			break;
		case (NET_RES_OK_ACCOUNT_ID):
			break;
		case (NET_RES_OK_LOG_IN_CHALLENGE):
			break;
		case (NET_RES_OK_CONFIRMATION):
			break;
		case (NET_RES_OK_ACCOUNT):
			break;
		case (NET_RES_OK_GAME_TOKEN):
			break;
		default:
			// I will deal with this later
			exit(1);
			break;
	}
}

void netHandler(struct mg_connection* connection, int event, void* eventData, void* funcData) {
	switch (event) {
		case (MG_EV_WS_OPEN):
			printf("Websocket is open.\n");
			break;
		case (MG_EV_ERROR):
			printf("Websocket opening error.\n");
			break;
		case (MG_EV_WS_MSG):
			netResponse(*(char*)eventData, eventData + 1);
			break;
	}
	if (event == MG_EV_ERROR || event == MG_EV_CLOSE || event == MG_EV_WS_MSG) {
		*(char*)funcData = 1;
	}
}

struct NetSession* netConnect(char* url) {
	struct NetSession* session = malloc(sizeof(struct NetSession));
	session->finished = 0;

	mg_mgr_init(&session->eventManager);
	mg_log_set(MG_LL_DEBUG);
	session->connection = mg_ws_connect(&session->eventManager, url, &netHandler, &session->finished, 0);

	while (session->connection && !session->finished) mg_mgr_poll(&session->eventManager, 1000);
	return session;
}

void netDispose(struct NetSession* session) {
	mg_mgr_free(&session->eventManager);
	free(session);
}

void netRequest(struct NetSession* session, unsigned char operation, void* data) {
	unsigned int len = 0;
	switch (operation) {
		case (NET_REQ_GET_USERNAME):
			len = 4;
			break;
		case (NET_REQ_LOOKUP_USERNAME):
			for(;((char*)data)[len];len++);
			break;
		case (NET_REQ_CREATE_ACCOUNT):
			for(;((char*)data)[len];len++);
			len += 33;
			break;
		case (NET_REQ_REQUEST_CHALLENGE):
			len = 4;
			break;
		case (NET_REQ_CHALLENGE_RESPONSE):
			len = 64;
			break;
		case (NET_REQ_LOG_OUT):
			break;
		case (NET_REQ_CHANGE_USERNAME):
			for(;((char*)data)[len];len++);
			break;
		case (NET_REQ_CHANGE_KEY):
			len = 32;
			break;
		case (NET_REQ_DELETE_ACCOUNT):
			break;
		case (NET_REQ_CREATE_GAME):
			break;
		case (NET_REQ_JOIN_GAME):
			for(;((char*)data)[len];len++);
			break;
	}
	char sendData[len + 1];
	sendData[0] = operation;
	memcpy(sendData + 1, data, len);

	mg_ws_send(session->connection, data, len + 1, WEBSOCKET_OP_BINARY);
}	
