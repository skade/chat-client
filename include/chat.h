#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Client Client;

void free_client(Client *client);

char *read_line(const Client *c);

Client *start_client(const char *addr, const char *name);

void write_msg(const Client *c, const char *msg);
