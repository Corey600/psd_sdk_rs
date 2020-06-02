// bindgen --default-enum-style=rust --distrust-clang-mangling --rust-target=1.40 wrap.hpp -- -x c++ -I./vendor > src/ffi.rs

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
