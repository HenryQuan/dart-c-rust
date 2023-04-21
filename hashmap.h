/**
 * This header is written manually
 */
#ifndef _HASHMAP_RUST_
#define _HASHMAP_RUST_

#pragma once

extern const void* build_hashmap();
extern const void* get_hashmap_value(const void*, int);
extern void free_hashmap(const void*);
extern void free_foo(const void*);

typedef struct Bar {
    int x;
    int y;
} Bar;

typedef struct Foo {
    int x;
    Bar y;
} Foo;

#endif
