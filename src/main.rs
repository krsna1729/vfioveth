use std::io::{self, Read};
mod cni;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let c: cni::CNISpec = cni::parse(buffer);
    println!("{}",cni::ipam(&c));
    cni::add_pair_ns(&c)
}
