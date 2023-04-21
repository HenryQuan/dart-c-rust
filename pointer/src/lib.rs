use libc::{c_int, c_void};
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;

// Dummy nested structs
const VALUE_LIMIT: i32 = 100;

// Clone is necessary here, somehow without it, the struct passed to C will be messed up
#[derive(Serialize, Debug, Clone)]
#[repr(C)] // ensure C layout
pub struct Foo {
    pub x: c_int,
    pub y: Bar,
}

#[derive(Serialize, Debug, Clone)]
#[repr(C)]
pub struct Bar {
    pub x: c_int,
    pub y: c_int,
}

/// Creates a random hashmap and returns a pointer to it.
#[no_mangle]
pub extern "C" fn build_hashmap() -> *const c_void {
    let mut hashmap = HashMap::new();
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let key = rng.gen_range(0..VALUE_LIMIT);
        let x = rng.gen_range(0..VALUE_LIMIT);
        let bar_x = rng.gen_range(0..VALUE_LIMIT);
        let bar_y = rng.gen_range(0..VALUE_LIMIT);
        let value = Foo {
            x,
            y: Bar { x: bar_x, y: bar_y },
        };
        hashmap.insert(key, value);
    }
    println!("Rust: {:?}", hashmap);
    let pointer = Box::into_raw(Box::new(hashmap)) as *const c_void;
    println!("Rust: pointer: {:p}", pointer);
    pointer
}

/// Gets a value from the hashmap.
/// Returns null if the key is not found, or the pointer to the value.
#[no_mangle]
pub extern "C" fn get_hashmap_value(pointer: *const c_void, key: i32) -> *const c_void {
    println!("Rust: hashmap pointer: {:p}", pointer);
    let hashmap = unsafe { &*(pointer as *const HashMap<i32, Foo>) };
    match hashmap.get(&key) {
        Some(value) => {
            let value = value.clone();
            println!("Rust: {:?}", value);
            // use bincode to convert to bytes
            let pointer = Box::into_raw(Box::new(value)) as *const c_void;
            println!("Rust: value pointer: {:p}", pointer);
            // let value_bytes = bincode::serialize(&value).unwrap();
            // println!("Rust: bytes: {:?}", value_bytes);
            pointer
        }
        None => {
            println!("Rust: key {} not found", key);
            std::ptr::null()
        }
    }
}

/// Frees the hashmap.
#[no_mangle]
pub extern "C" fn free_hashmap(pointer: *const c_void) {
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
pub extern "C" fn free_foo(pointer: *const c_void) {
    println!("Rust: pointer: {:p}", pointer);
    unsafe {
        // make sure the Foo type is the same as the one used in get_hashmap_value
        let foo = Box::from_raw(pointer as *mut Foo);
        println!("Rust: {:?}", foo);
        let foo_bytes = bincode::serialize(&foo).unwrap();
        println!("Rust: bytes: {:?}", foo_bytes);
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
