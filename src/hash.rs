use smallvec::SmallVec;
use std::io::Read;

fn main() {
    let mut input: reccak::Input = SmallVec::new();
    std::io::stdin().read(&mut input).unwrap();
    let output = reccak::hash(input.into());
    print!("0x");
    for e in output.iter() {
        print!("{:X}", e);
    }
    println!("");
}
