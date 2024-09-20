use rand::Rng;

pub fn modular_exponentiation(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1  {
            result = (result * base) % modulus 
        }
        exp = exp >> 1;
        base = (base * base) % modulus
    }
    result
}

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}
