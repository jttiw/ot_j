use curve25519_dalek::{scalar::Scalar};
use sha2::{Digest, Sha512};

fn main() {
    
    let x1 = [
            0xef, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58,
            0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde, 0x14,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10
        ];
    let x2 = [
            0xef, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58,
            0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde, 0x14,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10
        ];
    let s1 = Sender::new(x1, x2);
    //let mut hasher = curve25519_dalek::scalar::Scalar::
    //hasher.update(b"ASDASD");
    //let a: Scalar = Scalar::from_hash(hasher);
    let mut v: Vec<i32> = Vec::new();
    [1,2][..].clone_into(&mut v);
    
    println!("{:?}",v)
}
//xor byte arrays (discards trailing bytes if uneven length)
pub fn xor(x: &[u8], y: &[u8]) -> Vec<u8>{
    x.iter().zip(y.iter())
        .map(|(xb, yb)| xb ^ yb)
        .collect()
}
#[derive(Debug)]
pub struct Sender {
    x1: [u8; 32],
    x2: [u8; 32]
}

impl Sender {
    pub fn new(x1: [u8; 32], x2: [u8; 32]) -> Sender{
        Sender {
            x1,
            x2
        }
    }
}

