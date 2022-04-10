#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef uintptr_t (*RequestFn)(const char *method_ptr, uintptr_t method_len, const uint8_t *request_ptr, uintptr_t request_len, uint8_t **response_ptr);

/**
 * Get the name of the service the plugin exposes. A pointer to the name is written into the given
 * `name`. The caller must not deallocate the name. The length of the name is returned.
 */
uintptr_t name(const uint8_t **name);

/**
 * Returns the DCS-gRPC version the plugin is compatible with. The most significant 16 bits are the
 * major version number. The least significant 16 bits are the minor version number.
 */
int32_t api_version(void);

void start(RequestFn request_fn);

void stop(void);

uintptr_t call(const char *method_ptr,
               uintptr_t method_len,
               const uint8_t *request_ptr,
               uintptr_t request_len,
               uint8_t **response_ptr);
