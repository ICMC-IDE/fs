#![feature(str_from_utf16_endian)]

#[cfg(target_family = "wasm")]
mod sys {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(raw_module = "../../fs.js")]
    extern "C" {
        pub type Fs;

        #[wasm_bindgen(constructor)]
        pub fn new() -> Fs;

        #[wasm_bindgen(method)]
        pub fn read_binary(fs: &Fs, path: &str) -> Option<Box<[u8]>>;

        #[wasm_bindgen(method)]
        pub fn write(fs: &Fs, path: &str, content: &[u8]);
    }

    impl Fs {
        pub fn read(&self, path: &str) -> Option<String> {
            self.read_binary(path)
                .map(|bytes| String::from_utf16le_lossy(&bytes))
        }
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

        pub fn read_binary(&self, _path: impl AsRef<str>) -> Option<Box<[u8]>> {
            unimplemented!()
        }

        pub fn write(&mut self, _path: &str, _content: impl AsRef<[u8]>) {
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
    pub fn read_binary(&self, path: impl AsRef<str>) -> Option<Box<[u8]>> {
        self.0.read_binary(path.as_ref())
    }

    #[inline(always)]
    pub fn write(&mut self, path: impl AsRef<str>, content: impl AsRef<[u8]>) {
        self.0.write(path.as_ref(), content.as_ref())
    }
}
