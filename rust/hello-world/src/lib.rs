pub fn hello(name: Option<&str>) -> String {
    let hello_msg = "Hello, ".to_string();

    match name {
        None => hello_msg + "World!",
        Some(name) => hello_msg + name + "!",
    }
}
