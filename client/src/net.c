#include "net.h"

void netHandler(struct mg_connection* connection, int event, void* eventData, void* funcData) {

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
