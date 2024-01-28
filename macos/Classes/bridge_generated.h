#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_fit_width(int64_t port_,
                    struct wire_uint_8_list *path_str,
                    int32_t *compress_format,
                    int32_t *max_width,
                    uint8_t *quality);

void wire_fit_height(int64_t port_,
                     struct wire_uint_8_list *path_str,
                     int32_t *compress_format,
                     int32_t *max_height,
                     uint8_t *quality);

void wire_contain(int64_t port_,
                  struct wire_uint_8_list *path_str,
                  int32_t *compress_format,
                  int32_t *max_width,
                  int32_t *max_height,
                  uint8_t *quality);

int32_t *new_box_autoadd_compress_format_0(int32_t value);

int32_t *new_box_autoadd_i32_0(int32_t value);

uint8_t *new_box_autoadd_u8_0(uint8_t value);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_fit_width);
    dummy_var ^= ((int64_t) (void*) wire_fit_height);
    dummy_var ^= ((int64_t) (void*) wire_contain);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_compress_format_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_i32_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_u8_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
