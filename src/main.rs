fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let output = reccak::hash(input.as_bytes());
    print!("0x");
    for e in output.iter() {
        print!("{:X}", e);
    }
    println!("");
}
