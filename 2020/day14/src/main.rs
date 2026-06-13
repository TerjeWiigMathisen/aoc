// Fastest run Surface:  3.447 ms
//              Acer:    8.658 us for part1 
//                       4.727 ms both parts, sw pdep 
//                       1.967 ms, _pdep_u64 for floating bits distribution
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;
use rustc_hash::FxHashMap;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_pdep_u64;

fn _pext(n:u64, mask:u64) -> u64
{
    let mut ubit = 1;
    let mut res = 0;
    for i in 0..64 {
        let ibit = 1 << i;
        if (mask & ibit) != 0 {
            if n & ibit != 0 { res |= ubit;}
            ubit <<= 1;
        }
    }
    res
}

fn _pdep(n:u64, mask:u64) -> u64
{
    let mut ubit = 1;
    let mut res = 0;
    for i in 0..64 {
        let ibit = 1 << i;
        if mask & ibit != 0 {
            if n & ubit != 0 {
                res |= ibit;
            }
            ubit <<= 1;
        }
    }
    res
}

struct Mask {
    zero:u64,
    one:u64,
    x:u64,
}

fn parsemask(inp:&[u8]) -> (usize, Mask)
{
    let mut mask = Mask{zero:0,one:0,x:0};
    let mut retpos = 0;
    for i in 7.. {
        if inp[i] == b'\n' {retpos = i+1; break}
        mask.zero <<= 1;
        mask.one <<= 1;
        mask.x <<= 1;
        match inp[i] {
            b'0' => mask.zero |= 1,
            b'1' => mask.one |= 1,
            b'X' => mask.x |= 1,
            _ => panic!("Bad input mask"),
        }
    }
    (retpos, mask)
}

fn parsemem(inp:&[u8]) -> (usize, u64, u64)
{
    let mut i = 0;
    let mut adr = 0;
    i += 4;
    while inp[i] != b']' {
        adr = adr*10 + (inp[i] - b'0') as u64;
        i += 1;
    }
    i += 4;
    let mut val = 0;
    while inp[i] != b'\n' {
        val = val*10 + (inp[i] - b'0') as u64;
        i += 1;
    }
    (i+1, adr, val)
}

fn process(inp:&str) -> (u64, u64)
{
    let bytes = inp.as_bytes();
    let mut mask = Mask{zero:0, one:0, x:0};
    let mut adr;
    let mut val;
    let mut i = 0;
    let mut di;
    let mut map: FxHashMap<u64, u64> = FxHashMap::default();
    let mut map2: FxHashMap<u64, u64> = FxHashMap::default();
    let mut perm = 0;
    while i < bytes.len() {
        if bytes[i+1] == b'a' {
            // mask = 0X11XX1X010X01101000X01X011101100000
            (di, mask) = parsemask(&bytes[i..]);
            i += di;
            perm = 1 << mask.x.count_ones();
//            println!("zero:{:b}, one:{:b}, x:{:b}", mask.zero, mask.one, mask.x);
        }
        else if bytes[i+1] == b'e' {
            // mem[4634] = 907
            (di, adr, val) = parsemem(&bytes[i..]);
            i += di;
//            println!("adr = {adr:b}, val = {val:b}");
            let val1 = (val | mask.one) & !mask.zero;
//            println!("masked value = {val:b}");
            map.insert(adr, val1);

            // part2
            adr &= mask.zero;
            adr |= mask.one;
//            println!("adr = {adr:b}, perm: {perm}");
            for i in 0..perm {
//                let floating = pdep(i, mask.x);
                let floating = unsafe { _pdep_u64(i as u64, mask.x as u64) } as u64;
                let a = adr | floating;
//                println!("floating addr: {a:b}");
                //if map2.contains_key(&a) { panic!("Duplicate address {a}")}
                map2.insert(a, val);
            }
        }
        else {panic!("Bad input at offset {i}");}
    }
//    println!("part2 permutations: {perm}");
    let mut sum = 0;
    for (_key, val) in &map {
        sum += val;
    }
    let part1 = sum;
    sum = 0;
    for (_key, val) in &map2 {
        sum += val;
    }
    let part2 = sum;
    (part1,part2)
}

fn _process_1000(inp:&str) -> (u64, u64)
{
    for _ in 0..1000 {
        process(&inp);
    }
    (0,0)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
//    let fname = "test.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}