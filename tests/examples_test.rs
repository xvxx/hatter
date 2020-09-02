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
    println!("{:?}", dir);
    for test in fs::read_dir(dir)? {
        println!("{:?}", test);
        let test = test?;
        let path = test.path();
        if path.is_dir() {
            println!("{:?}", path);
            test_dir(path)?;
        } else {
            let source = fs::read_to_string(&path)?;
            let _ = hatter::scan(&source)
                .and_then(|tokens| hatter::parse(tokens))
                .map_err(|err| {
                    hatter::print_error(path, source, err);
                    assert!(false);
                });
            assert!(true);
        }
    }
    Ok(())
}
