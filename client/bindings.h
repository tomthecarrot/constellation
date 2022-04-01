#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Manages reading and writing to the `Realm`.
 *
 * # Threading architecture
 * The Engine has a queue of pending collactions that indend to mutate the
 * [`Realm`], as well as working copy of the `Realm` state. To avoid data races
 * the `Realm` is never simultaneously readable and writable at the same time.
 *
 * The `Engine` cannot be simultaneously written and read from. For this
 * reason, typically things are done in two steps: a writer phase where
 * collactions are dequeued and applied as mutations on the `Realm` state, and
 * a reader phase where all reads of the data take place, free of any mutation.
 * Handling the transitions between these phases is the responsibility of the
 * API Client(s).
 */
typedef struct Engine Engine;

typedef struct FfiTestingContract FfiTestingContract;

typedef struct Object Object;

typedef Index<Object> ObjectHandle;

typedef Index<State<uint8_t>> StateHandle_u8;

struct Engine *teleportal_engine_init(void);

const struct FfiTestingContract *teleportal_engine_get_contract_ffi_testing(struct Engine *engine);

const ObjectHandle *teleportal_engine_create_object(struct Engine *engine,
                                                    const struct FfiTestingContract *contract);

const StateHandle_u8 *teleportal_engine_get_state_handle_u8(const struct Engine *engine,
                                                            const ObjectHandle *object_handle,
                                                            uintptr_t state_idx);

uint8_t teleportal_engine_get_state_value_u8(const struct Engine *engine,
                                             const StateHandle_u8 *state_handle);
