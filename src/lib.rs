#![feature(str_from_utf16_endian)]

#[cfg(target_family = "wasm")]
mod sys {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "fs")]
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

        pub fn write(&mut self, _path: &str, _content: impl AsRef<[u8]>) {
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

pub struct Fs(sys::Fs);

impl Fs {
    #[inline(always)]
    pub fn new() -> Self {
        Self(sys::Fs::new())
    }

    #[inline(always)]
    pub fn read(&self, path: impl AsRef<str>) -> Option<String> {
        self.0.read(path.as_ref())
    }

    #[inline(always)]
    pub fn write(&mut self, path: impl AsRef<str>, content: impl AsRef<[u8]>) {
        self.0.write(path.as_ref(), content.as_ref())
    }

    #[inline(always)]
    pub fn delete(&mut self, path: impl AsRef<str>) {
        self.0.delete(path.as_ref())
    }

    #[inline(always)]
    pub fn files(&mut self) -> Vec<String> {
        self.0.files()
    }
}
