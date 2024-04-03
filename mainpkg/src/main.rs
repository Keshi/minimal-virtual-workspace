fn main() {
    println!("cwd: {:?}", std::env::current_dir().unwrap().as_os_str());
    let contents = std::fs::read_to_string("data/data.txt")
        .expect("Should have been able to read the file");

    println!("Loaded: {contents}");
}
