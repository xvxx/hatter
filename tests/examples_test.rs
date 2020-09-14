use {
    hatter,
    std::{fs, io, path},
};

#[test]
fn test_examples() -> io::Result<()> {
    test_dir("./examples/")
}

fn test_dir<P: AsRef<path::Path>>(dir: P) -> io::Result<()> {
    let dir = dir.as_ref();
    for test in fs::read_dir(dir)? {
        let test = test?;
        let path = test.path();
        if path.is_dir() {
            test_dir(path)?;
        } else {
            let source = fs::read_to_string(&path)?;
            let test_path = format!("{}", path.clone().into_os_string().into_string().unwrap())
                .replace("./examples/", "./tests/examples/")
                .replace(".hat", ".html");
            println!("Testing: {}", test_path);
            assert_eq!(
                fs::read_to_string(&test_path)?,
                hatter::to_html(&source)
                    .map_err(|err| {
                        hatter::print_error(path, source, err);
                        assert!(false);
                    })
                    .unwrap()
            );
            println!(">> {}", test_path);
        }
    }
    Ok(())
}
