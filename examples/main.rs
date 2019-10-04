use tgaimage::*;

fn main() {
    let s = std::mem::size_of::<TGAHeader>();
    println!("size of TGAHeader: {}", s);
}
