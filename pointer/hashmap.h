#ifndef _HASHMAP_RUST_
#define _HASHMAP_RUST_

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Bar {
  int x;
  int y;
} Bar;

typedef struct Foo {
  int x;
  struct Bar y;
} Foo;

/**
 * Creates a random hashmap and returns a pointer to it.
 */
const void *build_hashmap(void);

/**
 * Gets a value from the hashmap.
 * Returns null if the key is not found, or the pointer to the value.
 */
const void *get_hashmap_value(const void *pointer, int32_t key);

/**
 * Frees the hashmap.
 */
void free_hashmap(const void *pointer);

/**
 * Frees the Foo struct.
 */
void free_foo(const void *pointer);

#endif /* _HASHMAP_RUST_ */
