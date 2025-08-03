pub use sys::Fs;

#[cfg(target_family = "wasm")]
mod sys {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(raw_module = "../fs.ts")]
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
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};
    use wasm_bindgen::prelude::*;

    // This is a dummy struct to mimic the behavior of the wasm version
    #[wasm_bindgen]
    pub struct Fs {}

    impl Fs {
        pub fn new() -> Self {
            Self {}
        }

        pub fn read(&self, path: &str) -> Option<String> {
            let mut file = File::open(path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            Some(content)
        }

        pub fn write(&mut self, path: &str, content: &[u8]) -> std::io::Result<()> {
            let mut file = fs::File::create(path)?;
            file.write_all(content)?;
            Ok(())
        }

        pub fn delete(&mut self, path: &str) -> std::io::Result<()> {
            fs::remove_file(path)?;
            Ok(())
        }

        pub fn files(&self) -> Vec<String> {
            unimplemented!()
        }
    }
}
