// Fastest 5.8 us

use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

fn _process(inp:String) -> i32
{
    let mut locks:Vec<u32> = Vec::new();
    let mut keys:Vec<u32> = Vec::new();
    let blocks = inp.split("\n\n").collect::<Vec<&str>>();
    for block in blocks {
        let mut h:[i32;5] = [-1;5];
        let rows:Vec<String> = block.lines().map(|x| x.to_string()).collect();
        let first = rows[0].clone();
        for line in rows {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {h[i] += 1;}
            }
        }
        let mut bits:u32 = 0;
        for c in h.iter() {
            bits = (bits << 4) + *c as u32;
        }
        if first == "#####" {
            locks.push(bits);
        }
        else {
            keys.push(bits + 0x22222);
        }
    }

    let mut part1 = 0;

    for l in locks.iter() {
        for k in keys.iter() {
            let sum = l + k;
            if sum & 0x88888 == 0 {
                part1 += 1;
            }
        }
    }

    part1
}

#[target_feature(enable = "avx")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn _parse_kl_sse41(src: &[u8]) ->u32 {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let ascii_hash = _mm256_set1_epi8(b'#' as i8);

    let invec = _mm256_loadu_si256(src.as_ptr() as *const _);

    let ishash = _mm256_cmpeq_epi8(invec, ascii_hash);
    let mask = _mm256_movemask_epi8(ishash) as u32;
    mask
}

fn parse_kl(inp:&[u8]) -> u32
{
    let mut bits = 0;
    for i in 0..32 {
        let c:u32 = (inp[i] == b'#') as u32;
        bits |= c << i;
    }
    bits
}

pub fn _processu8(inp:String) -> i32
{
    let mut locks:Vec<u32> = Vec::new();
    let mut keys:Vec<u32> = Vec::new();
    let bytes = inp.as_bytes();
    let mut bend = 6*6;
    while bend < bytes.len() {
        let bstart = bend - 32;
        let bits = parse_kl(bytes[bstart..bend].as_ref());
    
        if bits & 1 == 1 {
            locks.push(bits);
        }
        else {
            keys.push(bits);
        }
        bend += 6*7+1;
    }

    let mut part1 = 0;
    for l in locks.iter() {
        for k in keys.iter() {
            let sum = l & k;
            if sum == 0 {
                part1 += 1;
            }
        }
    }

    part1
}

pub fn process_single(inp:String) -> i32
{
    let bytes = inp.as_bytes();
    let mut keylocks:Vec<u32> = vec![0;(bytes.len()+36)/43];
    let mut ki = 0;
    let mut li = keylocks.len();
    let mut bend = 6*6;
    while bend < bytes.len() {
        let bstart = bend - 32;
    
        let bits = parse_kl(bytes[bstart..bend].as_ref());
        if bits & 1 == 1 {
            li -= 1;
            keylocks[li] = bits;
        }
        else {
            keylocks[ki] = bits;
            ki += 1;
        }
        bend += 6*7+1;
    }
    assert!(ki == li);

    let mut part1 = 0;
    for l in li..keylocks.len() {
        let lock = keylocks[l];
        for k in 0..li {
            let sum = lock & keylocks[k];
            if sum == 0 {
                part1 += 1;
            }
        }
    }

    part1
}

pub fn process_four(inp:String) -> i32
{
    let bytes = inp.as_bytes();
    let locks_plus_keys = (bytes.len()+36)/43 + 7 & !7;
    let mut keylocks:Vec<u32> = vec![u32::MAX;locks_plus_keys];
    let mut ki = 0;
    let mut li = keylocks.len();
    let mut bend = 6*6;
    while bend < bytes.len() {
        let bstart = bend - 32;
    
        let bits = parse_kl(bytes[bstart..bend].as_ref());
        if bits & 1 == 1 {
            li -= 1;
            keylocks[li] = bits;
        }
        else {
            keylocks[ki] = bits;
            ki += 1;
        }
        bend += 6*7+1;
    }
    while (li & 3) != 0 {
        //ki += 1;
        li -= 1;
    }
    //assert!(ki == li);
    //assert!((li & 15) == 0);

    let mut part1 = 0;
    let mut part2 = 0;
    let mut part3 = 0;
    let mut part4 = 0;
    let mut l = li;
    while l < 502 {
        let lock1 = keylocks[l];
        let lock2 = keylocks[l+1];
        let lock3 = keylocks[l+2];
        let lock4 = keylocks[l+3];
        for k in 0..li {
            let key = keylocks[k];
            part1 += ((lock1 & key) == 0) as i32;
            part2 += ((lock2 & key) == 0) as i32;
            part3 += ((lock3 & key) == 0) as i32;
            part4 += ((lock4 & key) == 0) as i32;
        }
        l += 4;
    }
    part1 += part2 + part3 + part4;

    part1
}

pub fn process_eight(inp:String) -> i32
{
    let bytes = inp.as_bytes();
    let locks_plus_keys = (bytes.len()+36)/43 + 15 & !15;
    let mut keylocks:Vec<u32> = vec![u32::MAX;locks_plus_keys];
    let mut ki = 0;
    let mut li = keylocks.len();
    let mut bend = 6*6;
    while bend < bytes.len() {
        let bstart = bend - 32;
    
        let bits = parse_kl(bytes[bstart..bend].as_ref());
        if bits & 1 == 1 {
            li -= 1;
            keylocks[li] = bits;
        }
        else {
            keylocks[ki] = bits;
            ki += 1;
        }
        bend += 6*7+1;
    }
    while (li & 3) != 0 {
        //ki += 1;
        li -= 1;
    }
    //assert!(ki == li);
    //assert!((li & 15) == 0);

    let mut part1 = 0;
    let mut part2 = 0;
    let mut part3 = 0;
    let mut part4 = 0;
    let mut part5 = 0;
    let mut part6 = 0;
    let mut part7 = 0;
    let mut part8 = 0;
    let mut l = li;
    while l < 502 {
        let lock1 = keylocks[l];
        let lock2 = keylocks[l+1];
        let lock3 = keylocks[l+2];
        let lock4 = keylocks[l+3];
        let lock5 = keylocks[l+4];
        let lock6 = keylocks[l+5];
        let lock7 = keylocks[l+6];
        let lock8 = keylocks[l+7];
        for k in 0..li {
            let key = keylocks[k];
            part1 += ((lock1 & key) == 0) as i32;
            part2 += ((lock2 & key) == 0) as i32;
            part3 += ((lock3 & key) == 0) as i32;
            part4 += ((lock4 & key) == 0) as i32;
            part5 += ((lock5 & key) == 0) as i32;
            part6 += ((lock6 & key) == 0) as i32;
            part7 += ((lock7 & key) == 0) as i32;
            part8 += ((lock8 & key) == 0) as i32;
        }
        l += 8;
    }
    part1 += part2 + part3 + part4 + part5 + part6 + part7 + part8;


    part1
}

pub fn process_1000(inp:String) -> i32
{
    let mut p = 0;
    for _ in 0..1000 {
        p =process_eight(inp.clone());
    }
    p
}
pub fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    println!("Running 1000 iterations inside each timing test");
    let bench_result = run_benchmark(100, |_| { process_1000(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let part1 = process_four(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}