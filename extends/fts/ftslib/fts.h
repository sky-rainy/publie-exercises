#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

int init_fts(void);

const char *delete_all(void);

const char *query(const char *query);

const char *update(const char *content);

const char *delete_by_id(unsigned long long id);

const char *batch_add(const char *contents);

void free_cstring(char *s);
