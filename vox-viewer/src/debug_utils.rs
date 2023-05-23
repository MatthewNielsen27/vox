use rand::Rng;
use image::{Rgb};

pub fn random_col() -> Rgb<u8> {
    let mut rng = rand::thread_rng();
    let r: u8 = rng.gen();
    let g: u8 = rng.gen();
    let b: u8 = rng.gen();
    Rgb::from([r, g, b])
}
