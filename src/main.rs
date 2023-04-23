fn main() {
    let mut arguments = std::env::args().skip(1); // args is an itterator were skipping the first one.
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    print!("the key is '{}' and the value is '{}'", key, value);
    // docs for standard rust is https://doc.rust-lang.org/stable/std
}
