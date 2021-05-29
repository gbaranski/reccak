use std::io::Read;
use smallvec::SmallVec;

fn main() {
    let mut input: SmallVec::<[u8; 32]> = SmallVec::new();
    std::io::stdin().read(&mut input).unwrap();
    let output = reccak::hash(input.into());
    print!("0x");
    for e in output.iter() {
        print!("{:X}", e);
    }
    println!("");
}
