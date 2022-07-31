#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Buffer {
  uint8_t *data;
  uint64_t len;
} Buffer;

int init_fts(void);

struct Buffer delete_all(void);

struct Buffer query(const char *query);

struct Buffer update(struct Buffer content_buf);

struct Buffer delete_by_id(unsigned long long id);

struct Buffer batch_add(struct Buffer content_buf);

void free_bytes(struct Buffer buf);
