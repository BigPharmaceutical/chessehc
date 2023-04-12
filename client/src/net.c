#include "net.h"
#include "persistent.h"

/////////////////////////////////
/// TODO safety /////////////////
/////////////////////////////////

void pleaseBreakpoint() {}

void netResponse(unsigned char opcode, void* data) {
	unsigned int len = 0;
	switch (opcode) {
		case (NET_RES_OK_USERNAME):
			for(;((char*)data)[len];len++);
			break;
		case (NET_RES_OK_ACCOUNT_ID):
			len = 8;
			setAccountId(*(long long*) data);
			break;
		case (NET_RES_OK_LOG_IN_CHALLENGE):
			
			break;
		case (NET_RES_OK_CONFIRMATION):
			len = 1;
			break;
		case (NET_RES_OK_ACCOUNT):
			//todo
			break;
		case (NET_RES_OK_GAME_TOKEN):
			for(;((char*)data)[len];len++);
			break;
		default:
			// I will deal with this later
			//exit(1);
			// I guess it became a problem anyway
			pleaseBreakpoint();
			break;
	}
}

void netHandler(struct mg_connection* connection, int event, void* eventData, void* funcData) {
	switch (event) {
		case (MG_EV_OPEN):
			connection->is_hexdumping = 1;
			break;
		case (MG_EV_WS_OPEN):
			printf("Websocket is open.\n");
			break;
		case (MG_EV_ERROR):
			printf("Websocket opening error.\n");
			break;
		case (MG_EV_WS_MSG): {
			struct mg_ws_message* message = eventData;
			netResponse(*message->data.ptr, (void*) message->data.ptr + 1);
		} break;
	}
	struct NetSessionResponse* rdata = funcData;
	if (event == MG_EV_ERROR || event == MG_EV_CLOSE || event == MG_EV_WS_MSG || event == MG_EV_WS_OPEN) {
		if (rdata->data) {
		   	if (*(char*)eventData == *(char*)rdata->data) {
				// Filter matches response
				rdata->data = eventData;
				rdata->finished = 1;
			}
		} else {
			rdata->finished = 1;
		}
	}
}

struct NetSession* netConnect(char* url) {
	struct NetSession* session = malloc(sizeof(struct NetSession));
	session->resp.finished = 0;
	session->resp.data = 0;
	mg_mgr_init(&session->eventManager);
	mg_log_set(MG_LL_DEBUG);
	session->connection = mg_ws_connect(&session->eventManager, url, &netHandler, &session->resp, 0);

	while (session->connection && !session->resp.finished) mg_mgr_poll(&session->eventManager, 1000);
	return session;
}

void netDispose(struct NetSession* session) {
	mg_mgr_free(&session->eventManager);
	free(session);
}

void* netAwait(struct NetSession* session, unsigned char operation) {
	session->resp.finished = 0;
	if (operation) {
		session->resp.data = &operation;
	} else {
		session->resp.data = 0;
	}
	while (session->connection && !session->resp.finished) mg_mgr_poll(&session->eventManager, 50);
	return session->resp.data;
}

void* netRequest(struct NetSession* session, unsigned char operation, void* data, void (*responseHandler)(void*)) {
	unsigned int len = 0;
	unsigned char expectedResponse = 0;
	switch (operation) {
		case (NET_REQ_GET_USERNAME):
			len = 8;
			break;
		case (NET_REQ_LOOKUP_USERNAME):
			for(;((char*)data)[len];len++);
			break;
		case (NET_REQ_CREATE_ACCOUNT):
			for(;((char*)data)[len++];);
			len += 32;
			expectedResponse = NET_RES_OK_CONFIRMATION;
			break;
		case (NET_REQ_REQUEST_CHALLENGE):
			len = 8;
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

	mg_ws_send(session->connection, sendData, len + 1, WEBSOCKET_OP_BINARY);
	return netAwait(session, expectedResponse);
}

void netCreateAccount(struct NetSession* session, char* username) {
	// Should the string be null-terminated?
	unsigned short len = 0;
	for(;username[len++];);
	unsigned char data[len + 32];
	memcpy(data, username, len * sizeof(unsigned char));
	memcpy(data + len, getKeyPublic(), 32 * sizeof(unsigned char));
	netRequest(session, NET_REQ_CREATE_ACCOUNT, data, 0);
	// we will ASSERT that it worked :)
	// Then we need the username
	netRequest(session, NET_REQ_LOOKUP_USERNAME, username, 0);
}
