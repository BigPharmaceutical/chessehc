#ifndef H_NET
#define H_NET

#include "mongoose.h"

struct NetSession {
	struct mg_mgr eventManager;
	struct mg_connection* connection;
	char finished;
};

struct NetSession* netConnect(char* url);

void netDispose(struct NetSession* session);

#endif
