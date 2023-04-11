#ifndef H_PERSISTENT
#define H_PERSISTENT

void initPersistence();

long long getAccountId();

unsigned char* getKeyPublic();

void setAccountId(long long value);

#endif
