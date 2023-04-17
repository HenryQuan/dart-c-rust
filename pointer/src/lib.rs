use rand::Rng;
use std::{collections::HashMap, os::raw::c_void};

// Dummy nested structs

#[derive(Debug)]
#[repr(C)] // ensure C layout
struct Foo {
    _x: i32,
    _y: Bar,
}

#[derive(Debug)]
#[repr(C)]
struct Bar {
    _x: i32,
    _y: i32,
}

/// Creates a random hashmap and returns a pointer to it.
#[no_mangle]
extern fn build_hashmap() -> *mut c_void {
    let mut hashmap = HashMap::new();
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let key = rng.gen_range(0..100);
        let x = rng.gen_range(0..100);
        let bar_x = rng.gen_range(0..100);
        let bar_y = rng.gen_range(0..100);
        let value = Foo {
            _x: x,
            _y: Bar {
                _x: bar_x,
                _y: bar_y,
            },
        };
        hashmap.insert(key, value);
    }
    println!("Rust: {:?}", hashmap);
    let pointer = Box::into_raw(Box::new(hashmap)) as *mut c_void;
    println!("Rust: pointer: {:p}", pointer);
    pointer
}

/// Gets a value from the hashmap.
/// Returns null if the key is not found, or the pointer to the value.
#[no_mangle]
extern fn get_hashmap_value(pointer: *mut c_void, key: i32) -> *mut c_void {
    println!("Rust: pointer: {:p}", pointer);
    let hashmap = unsafe { &*(pointer as *mut HashMap<i32, Foo>) };
    match hashmap.get(&key) {
        Some(value) => {
            let pointer = Box::into_raw(Box::new(value)) as *mut c_void;
            println!("Rust: pointer: {:p}", pointer);
            pointer
        }
        None => {
            println!("Rust: key {} not found", key);
            std::ptr::null_mut()
        }
    }
}

/// Frees the hashmap.
#[no_mangle]
extern fn free_hashmap(pointer: *mut c_void) {
    println!("Rust: pointer: {:p}", pointer);
    unsafe {
        // make sure the HashMap type is the same as the one used in build_hashmap
        let hashmap = Box::from_raw(pointer as *mut HashMap<i32, Foo>);
        println!("Rust: {:?}", hashmap);
    }
    println!("Rust: freed");
}

/// Frees the Foo struct.
#[no_mangle]
extern fn free_foo(pointer: *mut c_void) {
    println!("Rust: pointer: {:p}", pointer);
    unsafe {
        // make sure the Foo type is the same as the one used in get_hashmap_value
        let foo = Box::from_raw(pointer as *mut Foo);
        println!("Rust: {:?}", foo);
    }
    println!("Rust: freed");
}

#[cfg(test)]
mod tests {
    use crate::{build_hashmap, free_hashmap};

    #[test]
    fn main() {
        let pointer = build_hashmap();
        free_hashmap(pointer);
    }
}
