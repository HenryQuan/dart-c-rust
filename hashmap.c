#include <stdio.h>
#include <stdlib.h>

typedef void* Hashmap;
extern Hashmap build_hashmap();
extern void* get_hashmap_value(Hashmap, int);
extern void free_hashmap(Hashmap);
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
    Hashmap map = build_hashmap();
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
            printf("C: bytes: [ ");
            // print the next 12 bytes
            for (int i = 0; i < 12; i++) {
                printf("%d ", ((unsigned char*)foo)[i]);
            }
            printf("]\n");
            printf("C: value pointer: %p\n", foo);
            printf("C: Foo: { x: %d, y: Bar { x: %d, y: %d } }\n", foo->x, foo->y.x, foo->y.y);
            free_foo(foo);
            printf("C: value freed\n");
        }
    }
    free_hashmap(map);
    printf("C: hashmap freed\n");
    return 0;
}
