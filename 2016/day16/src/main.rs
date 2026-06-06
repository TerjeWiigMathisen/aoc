// Fastest run 12812.6 us

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
//use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
//use rustc_hash::FxHashMap;

struct Bitmap {
    bits:Vec<u32>,
    len:usize,
}
impl Bitmap {
    fn new(len:usize) -> Bitmap { 
        Bitmap { bits:vec![0; 2+(len+31)/32], len:0 }
    }
    fn add_bits(&mut self, b:u32, bits:usize) {
        let bitpos = self.len & 31;
        let bot = (b as u64) << bitpos;
        let item = self.len/32;
        self.bits[item] |= bot as u32 ;
        self.bits[item+1] = (bot >> 32) as u32;
        self.len += bits;
    }
    fn parity(&self, start:usize, bits:usize) -> u8 {
        //assert!(start & 15 == 0);
        //assert!(bits & 15 == 0);
        assert!(start + bits <= self.len);
        let mut item = start/32;
        let mut bc = 0;
        let start_offset = start & 31;
        let mut bits = bits;
        if start_offset != 0 {
            bc += popcnt((self.bits[item] >> start_offset) as u64);
            item += 1;
            bits -= start_offset;
        }
        while bits >= 32 {
            bc += popcnt(self.bits[item] as u64);
            item += 1;
            bits -= 32;
        }
        if bits != 0 {
            bc += popcnt((self.bits[item] << (32-bits)) as u64);
        }
        (1 ^ (bc & 1)) as u8
    }
    fn checksum(&self, bits:usize) -> String {
        let mut odd_bits = bits;
        while odd_bits & 1 == 0 {
            odd_bits >>= 1;
        }
        let itemlen = bits / odd_bits;
        let mut check:Vec<u8> = vec![0;odd_bits];
        for i in 0..odd_bits {
            check[i] = b'0'+self.parity(i*itemlen,itemlen);
        }
        String::from_utf8(check.to_vec()).unwrap()
    }
    fn debug_bits(&self, bits:usize) {
        let mut o:Vec<u8> = Vec::with_capacity(bits);
        for i in 0..bits {
            let item = i/32;
            let bit = (self.bits[item] >> (i & 31)) & 1;
            o.push(b'0'+bit as u8);
        }
        let s = String::from_utf8(o).unwrap();
        let facit = "11011110011011101001000100110000100011011110011011101101000100110000100011011110011011101001000100110000100111011110011011101101000100110000100011011110011011101001000100110000100011011110011011101101000100110000100111011110011011101001000100110000100111011110011011101101".to_string();
        if facit != s {
            let mut diff = "".to_string();
            for i in 0..facit.as_bytes().len() {
                diff.push(if facit.as_bytes()[i] == s.as_bytes()[i] { ' ' } else { '^' });
            }
            println!("{facit}");
            println!("{s}");
            println!("{diff}");
        }
    }
}

fn popcnt(x:u64) -> u32 {
    let mut c = 0;
    let mut x = x;
    while x != 0 {
        c += 1;
        x &= x-1;
    }
    c
}

fn process(inp:&str) -> (String, String)
{
    let mut f:u32 = 0;
    let mut r:u32 = 0;
    let bytes = inp.as_bytes();
    let inplen = bytes.len();

    for c in 0..inplen {
        let b = (bytes[c] & 1) as u32;
        f += b << c;
        r += r + (1-b);
    }
    let mut dragon = Bitmap::new(35651584);
    let mut sep:Vec<u8> = vec![0;1];
    while sep.len() < 35651584/(inplen+1) {
        let mut i = sep.len();
        sep.push(0);
        while i > 0 {
            i -= 1;
            sep.push(sep[i]+1);
        }
    }
    //println!("separators =\n{sep:?}\n");

    let mut i = 0;
    while dragon.len < 35651584 {
        dragon.add_bits(f,inplen);
        dragon.add_bits((sep[i] & 1) as u32,1);
        dragon.add_bits(r,inplen);
        dragon.add_bits((sep[i+1] & 1) as u32,1);
        i += 2;
    }
//    dragon.debug_bits(272);
//    println!("");
    let part1 = dragon.checksum(272);
    let part2 = dragon.checksum(35651584);

    (part1, part2)
}

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // let mut fname = "input.txt".to_string();
    // if args.len() > 1 { fname = args[1].clone(); }

    // let mut input = fs::read_to_string(fname).expect("Error readin input file");
    // if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let input = "11011110011011101";

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}