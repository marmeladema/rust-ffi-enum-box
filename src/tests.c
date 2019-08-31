#include "tests.h"

void scheme_ctest_01() {
	scheme_t *scheme = scheme_new();
	scheme_print(scheme);
	scheme_set_type(scheme, SCHEME_TYPE_INT);
	scheme_print(scheme);
	scheme_free(scheme);
}

void scheme_ctest_02() {
	scheme_t *scheme = scheme_new();
	scheme_print(scheme);
	scheme_type_t *ty = (scheme_type_t *)malloc(sizeof(scheme_type_t));
	*ty = SCHEME_TYPE_INT;
	scheme_set_type(scheme, *ty);
	free(ty);
	scheme_print(scheme);
	scheme_free(scheme);
}
