#include "persistent.h"
#include "stdio.h"
#include "ed25519.h"
#include "stdlib.h"

unsigned char keyPublic[32];
unsigned char keyPrivate[64];
long long accountid = 0;

void initPersistence() {
	FILE* file = fopen("chessehc.key", "a+b");
	fseek(file, 0, SEEK_END);
	long size = ftell(file);
	fseek(file, 0, SEEK_SET);

	if (size) {
		fread(keyPublic, sizeof(char), 32, file);
		fread(keyPrivate, sizeof(char), 64, file);
	} else {
		unsigned char seed[32];
		if (ed25519_create_seed(seed)) {
			printf("Error generating seed.");
			exit(1);
		}
		ed25519_create_keypair(keyPublic, keyPrivate, seed);
		fwrite(keyPublic, sizeof(char), 32, file);
		fwrite(keyPrivate, sizeof(char), 64, file);
	}

	if (size > 96) {
		fread(&accountid, sizeof(long long), 1, file);
	}

	fclose(file);
}



long long getAccountId() {
	return accountid;
}

unsigned char* getKeyPublic() {
	return keyPublic;
}

unsigned char* getKeyPrivate() {
	return keyPrivate;
}

void setAccountId(long long value) {
	accountid = value;
	FILE* file = fopen("chessehc.key", "a+b");
	fseek(file, 96, SEEK_SET);
	fwrite(&value, sizeof(long long), 1, file);
	fclose(file);
}
