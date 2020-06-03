#![allow(non_snake_case)]
// bindgen --default-enum-style=rust --distrust-clang-mangling --rust-target=1.40 wrap.hpp -- -x c++ -I./vendor > src/ffi.rs

#[allow(non_camel_case_types)]
mod ffi;
pub use ffi::*;
use std::mem;
use std::ptr;

pub fn readPsd() {
    unsafe {
        let mut allocator = psd_Allocator { vtable_: ptr::null() };
        let mut file = psd_File::new(&mut allocator);
        let filename = "../example.psd";
        let v: Vec<u32> = filename
            .as_bytes()
            .chunks(4)
            .map(|c| {
                mem::transmute::<[u8; 4], u32>([c[0], c[1], c[2], c[3]])
            }).collect();
        file.OpenRead(v.as_ptr());
        let doc = psd_CreateDocument(&mut file, &mut allocator);
        println!("{}", (*doc).channelCount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        readPsd();
        assert_eq!(2 + 2, 4);
    }
}
