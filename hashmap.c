#include <stdio.h>
#include <stdlib.h>

extern void* build_hashmap();
extern void* get_hashmap_value(void*, int);
extern void free_hashmap(void*);
extern void free_foo(void*);

typedef struct Bar {
    int x;
    int y;
} Bar;

typedef struct Foo {
    int x;
    Bar y;
} Foo;


int main(int argc, char* argv[]) {
    printf("C: Hello, world!\n");
    void* map = build_hashmap();
    printf("C: hashmap: %p\n", map);
    // ask for a key while q is pressed
    char buffer[256];
    while (1) {
        printf("C: key: ");
        // this should be a number, ignore other characters unless it is q
        fgets(buffer, 256, stdin);
        if (buffer[0] == 'q') {
            break;
        }

        int key = atoi(buffer);
        Foo* foo = get_hashmap_value(map, key);
        if (foo == NULL) {
            printf("C: value is null\n");
        } else {
            printf("C: value pointer: %p\n", foo);
            printf("C: Foo: {x: %d, y: {x: %d, y: %d}}\n", foo->x, foo->y.x, foo->y.y);
            free_foo(foo);
            printf("C: value freed\n");
        }
    }
    free_hashmap(map);
    printf("C: hashmap freed\n");
    return 0;
}
