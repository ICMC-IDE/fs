use fs::Fs;

#[test]
fn test() {
    let mut f = Fs::new();

    f.write("test", "Hello, World!".as_bytes()).unwrap();
    assert_eq!(f.read("test").unwrap(), "Hello, World!");
    f.delete("test").unwrap();
}
