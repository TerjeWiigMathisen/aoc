//use std::env;

#[inline(never)]
fn rint(x:f64)->i64
{
    let biased = x + (1024.0*1024.0*1024.0*1024.0*1024.0*6.0); //1.5*2^52
    let biased_bytes = biased.to_le_bytes();
    let mut n = i64::from_le_bytes(biased_bytes);
    n &= (1 << 52)-1;                   // Keeping mantissa only
    n -= 1024*1024*1024*1024*1024*2;    // Remove 0.5*2^52 bias
    return n;
}

fn bytes_to_string(b:Vec<u8>)->String
{
    unsafe {
        String::from_utf8_unchecked(b)
    }
}


fn main() {
    let bytes:Vec<u8> = vec![0xc0,0x80];
    let bytes_copy = bytes.clone();
    let s = bytes_to_string(bytes_copy);
    println!("{:?} {:?}", bytes, s.as_bytes());
}