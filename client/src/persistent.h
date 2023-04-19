#ifndef H_PERSISTENT
#define H_PERSISTENT

void initPersistence();

long long getAccountId();

unsigned char* getKeyPublic();

unsigned char* getKeyPrivate();

void setAccountId(long long value);

#endif
