#ifndef _TESTS_H_
#define _TESTS_H_

#include <stdlib.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// https://github.com/rust-lang/rfcs/blob/master/text/2195-really-tagged-unions.md
typedef enum {
    SCHEME_TYPE_TAG_IP,
    SCHEME_TYPE_TAG_BYTES,
    SCHEME_TYPE_TAG_INT,
    SCHEME_TYPE_TAG_BOOL,
    SCHEME_TYPE_TAG_ARRAY,
    SCHEME_TYPE_TAG_MAP,
} scheme_type_tag_t;

typedef struct {
    scheme_type_tag_t tag;
    void *data[2];
} scheme_type_t;

static const scheme_type_t SCHEME_TYPE_IP = {.tag = SCHEME_TYPE_TAG_IP, .data = {NULL, NULL}};
static const scheme_type_t SCHEME_TYPE_BYTES = {.tag = SCHEME_TYPE_TAG_BYTES, .data = {NULL, NULL}};
static const scheme_type_t SCHEME_TYPE_INT = {.tag = SCHEME_TYPE_TAG_INT, .data = {NULL, NULL}};
static const scheme_type_t SCHEME_TYPE_BOOL = {.tag = SCHEME_TYPE_TAG_BOOL, .data = {NULL, NULL}};

typedef struct scheme scheme_t;

scheme_t *scheme_new();
void scheme_free(scheme_t *);

void scheme_set_type(scheme_t *scheme, scheme_type_t ty);

void scheme_print(scheme_t *scheme);

#ifdef __cplusplus
}
#endif

#endif // _TESTS_H_
