//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn process(inp:&str) -> (usize, String)
{
    let sizex = 25;
    let sizey = 6;

    let bytes = inp.as_bytes();
    let len = bytes.len();

    let page = sizex*sizey;
    let mut min0 = usize::MAX;
    let mut prod = 0;
    let mut display:Vec<u8> = vec![b'2';page];
    for i in (0..len).step_by(page) {
        let mut digits:Vec<usize> = vec![0;10];
        for j in 0..page {
            let dig:usize = (bytes[j+i] - b'0') as usize;
            digits[dig] += 1;
            if display[j] == b'2' {
                display[j] = bytes[j+i];
            }
        }
        if digits[0] < min0 {
            min0 = digits[0];
            prod = digits[1]*digits[2];
            println!("new min0 = {min0}, prod = {prod}");
        }
    }
    let mut p = 0;
    let mut buf:Vec<u8> = Vec::with_capacity(page+sizey);
    for _y in 0..sizey {
        for _x in 0..sizex {
            //print!("{}",if display[p] == b'1' {'X'}else{' '});
            buf.push(if display[p] == b'1' {b'X'}else{b' '});
            p += 1;
        }
        //println!();
        buf.push(b'\n');
    }
    (prod, String::from_utf8(buf).unwrap())
}

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[target_feature(enable = "avx")]

#[inline]
fn bitplanes(bytes:&[u8]) -> (u64, u64)
{
    unsafe {
        let mut a = _mm256_lddqu_si256(bytes.as_ptr() as *const __m256i);
//        a = _mm256_and_si256(a, _mm256_set1_epi8(3));
        a = _mm256_slli_epi16(a, 6);
        let two = _mm256_movemask_epi8(a) as u32;
        a = _mm256_add_epi8(a,a);
        let one = _mm256_movemask_epi8(a) as u32;
        //let mut o = 0;
        //let mut t = 0;
        // for i in 0..32 {
        //     let b = bytes[i] as u32;
        //     o += (b & 1) << i;
        //     t += ((b >> 1) & 1) << i;
        // }
        // if o != one || t != two {
        //     println!("bytes = {}, o={o:x}, one = {one:x}, t = {t:x}, two = {two:x}", 
        //         String::from_utf8(bytes.to_vec()).unwrap());
        //     assert!(false);
        // }
        (one as u64, two as u64)
    }
}

fn _bitplane64(bytes:&[u8]) -> (u64, u64)
{
    unsafe {
        let mut a = _mm256_lddqu_si256(bytes.as_ptr() as *const __m256i);
        let mut b = _mm256_lddqu_si256(bytes.as_ptr().add(32) as *const __m256i);
//        a = _mm256_and_si256(a, _mm256_set1_epi8(3));
        a = _mm256_slli_epi16(a, 6);
        b = _mm256_slli_epi16(b, 6);
        let two = _mm256_movemask_epi8(a) as u64;
        let tb = _mm256_movemask_epi8(b) as u64;
        a = _mm256_add_epi8(a,a);
        b = _mm256_add_epi8(b,b);
        
        let one = _mm256_movemask_epi8(a) as u64;
        let ob = _mm256_movemask_epi8(b) as u64;
        (one + (ob<<32), two + (tb<<32))
    }
}

#[inline]
fn loadpage(bytes:&[u8]) -> (u64, u64, u64, u64, u64, u64)
{
    let o0:u64;
    let o1:u64;
    let o2:u64;
    let t0:u64;
    let t1:u64;
    let t2:u64;

    unsafe {
        let slice = &bytes[0..32]; //.as_slice();
        let (one,two) = bitplanes(slice); 
        let slice = &bytes[32..64];
        let (oo,tt) = bitplanes(slice);
        o0 = one + (oo << 32);
        t0 = two + (tt << 32);
    
        let slice = &bytes[64..96];
        let (one,two) = bitplanes(slice);
        let slice = &bytes[96..128];
        let (oo,tt) = bitplanes(slice);
        o1 = one + (oo << 32);
        t1 = two + (tt << 32);

        let slice = &bytes[118..150];
        let (one,two) = bitplanes(slice);
        o2 = one >> 10;
        t2 = two >> 10;
    }
    (o0,o1,o2,t0,t1,t2)
}

fn popcnt(n:u64) -> u16
{
    let mut c = 0;
    let mut n = n;
    while n != 0 {
        c += 1;
        n &= n-1;
    }
    c
}

//fn process_avx(inp:&str) -> (usize, String)
fn process_avx(inp:&str) -> (usize, u64, u64, u64)
{
    assert!(is_x86_feature_detected!("avx"));
    let sizex = 25;
    let sizey = 6;

    let bytes = inp.as_bytes();
    let len = bytes.len();
    //println!("len = {len}, sizex = {sizex}, sizey = {sizey}");
    //assert!(len == 100 * sizex * sizey);

    //let page = sizex*sizey;
    let mut prod;
    let mut o0;
    let mut o1;
    let mut o2;
    let mut t0; 
    let mut t1;
    let mut t2;
    (o0,o1,o2,t0,t1,t2) = loadpage(&bytes[0..150]);
    let ones = popcnt(o0) + popcnt(o1) + popcnt(o2);
    let twos = popcnt(t0) + popcnt(t1) + popcnt(t2);
    let mut max12 = ones+twos;
    prod = ones*twos;

    let mut page_start = 150;
    loop {
        let (one0,one1,one2, two0, two1, two2) = loadpage(&bytes[page_start..page_start+150]);
        let onetwo = popcnt(one0+two0)+popcnt(one1+two1)+popcnt(one2+two2);
        if onetwo > max12 {
            let ones = popcnt(one0) + popcnt(one1) + popcnt(one2);
            let twos = onetwo-ones;
            max12 = onetwo;
            prod = ones*twos;
        }
        o0 |= t0 & one0; t0 &= two0;
        o1 |= t1 & one1; t1 &= two1;
        o2 |= t2 & one2; t2 &= two2;
        page_start += 150;
        if page_start >= len {break;}
    }
    // let mut p = 0;
    // let mut buf:Vec<u8> = Vec::with_capacity(page+sizey);
    // for one in [o0, o1, o2].iter() {
    //     let mut o = *one;
    //     for _ in 0..64 {
    //         buf.push(if o & 1 != 0 {b'X'} else {b' '} );
    //         p += 1;
    //         if p == 25 {
    //             buf.push(b'\n');
    //             p = 0;
    //         }
    //         o >>= 1;
    //     }
    // }
    // (prod as usize, String::from_utf8(buf).unwrap())
    (prod as usize, o0,o1,o2)
}

fn process1k(input:&str) -> (usize, u64, u64, u64)
{
    let mut p = 0;
    for _ in 0..1000 {
        let (prod, o0,o1,o2) = process_avx(input);
        p += prod;
    }
    (p,0,0,0)
}
fn main() {
//    assert!(process("3,3,1108,-1,8,3,4,3,99",vec![8]).(0) == 1);
//    assert!(process("3,3,1108,-1,8,3,4,3,99",vec![7]) == 0);
//    assert!(process("3,3,1107,-1,8,3,4,3,99",vec![7]) == 1);

//    let bench_result = run_benchmark(10, |_| {bench_permute();}); bench_result.print_stats();

    //panic!("Stop now");

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error reading input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process1k(&input); }); bench_result.print_stats();

    devtime.start();
//    let (part1, part2) = process_avx(&input);
    let (part1, o0,o1,o2) = process_avx(&input);
    //let part2 = 0; //process2(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    let mut buf:Vec<u8> = Vec::with_capacity(156);
    let mut p = 0;
    for one in [o0, o1, o2].iter() {
        let mut o = *one;
        for _ in 0..64 {
            buf.push(if o & 1 != 0 {b'X'} else {b' '} );
            p += 1;
            if p == 25 {
                buf.push(b'\n');
                p = 0;
            }
            o >>= 1;
        }
    }
    println!("Part2 = \n{}", String::from_utf8(buf).unwrap());
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}