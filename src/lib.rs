#![feature(str_from_utf16_endian)]

pub use sys::Fs;

#[cfg(target_family = "wasm")]
mod sys {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(raw_module = "../scripts/worker/resources/fs.js")]
    extern "C" {
        pub type Fs;

        #[wasm_bindgen(constructor)]
        pub fn new() -> Fs;

        #[wasm_bindgen(method)]
        pub fn read(fs: &Fs, path: &str) -> Option<String>;

        #[wasm_bindgen(method)]
        pub fn write(fs: &Fs, path: &str, content: &[u8]);

        #[wasm_bindgen(method)]
        pub fn delete(fs: &Fs, path: &str);

        #[wasm_bindgen(method)]
        pub fn files(fs: &Fs) -> Vec<String>;
    }
}

#[cfg(not(target_family = "wasm"))]
mod sys {
    pub struct Fs {}

    impl Fs {
        pub fn new() -> Self {
            Self {}
        }

        pub fn read(&self, _path: &str) -> Option<String> {
            unimplemented!()
        }

        pub fn write(&mut self, _path: &str, _content: &[u8]) {
            unimplemented!()
        }

        pub fn delete(&mut self, _path: &str) {
            unimplemented!()
        }

        pub fn files(&self) -> Vec<String> {
            unimplemented!()
        }
    }
}
