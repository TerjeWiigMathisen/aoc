//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use hex::encode;

fn process(seq:&Vec<u8>) -> i32
{
    let mut bytes:[u8;256] = [0;256];
    for i in 0..256 { bytes[i] = i as u8; }

    let mut pos: usize = 0;
    let mut skip: usize = 0;
    for len in seq {
        let mut l = *len as usize;
        let mut b = pos;
        let mut e = (pos + l) & 255;
        while l > 1 {
            e = (e+255) & 255;  // e--
            let n = bytes[b];
            bytes[b] = bytes[e];
            bytes[e] = n;
            b = (b+1) & 255;
            l -= 2;
        }
        pos = (pos + *len as usize + skip) & 255;
        skip += 1;
    }

    let p1 = bytes[0] as i32 * bytes[1] as i32;
    return p1;
}

fn process5(blen:usize, seq:Vec<u8>) -> i32
{
    let mut bytes:Vec<u8> = vec![];
    for i in 0..blen { bytes.push(i as u8); }

    let mut pos: usize = 0;
    let mut skip: usize = 0;
    for len in seq {
        let mut l = len as usize;
        let mut b = pos;
        let mut e = (pos + l) % blen;
        while l > 1 {
            e = (e+blen-1) % blen;  // e--
            let n = bytes[b];
            bytes[b] = bytes[e];
            bytes[e] = n;
            b = (b+1) % blen;
            l -= 2;
        }
        pos = (pos + len as usize + skip) % blen;
        skip += 1;
    }

    let p1 = bytes[0] as i32 * bytes[1] as i32;
    return p1
}

fn round2(seq:&[u8]) -> String
{
    let mut bytes:[u8;256] = [0;256];
    for i in 0..256 { bytes[i] = i as u8; }

    let mut pos: usize = 0;
    let mut skip: usize = 0;
    for _ in 0..64 {
        for len in seq {
            let mut l = *len as usize;
            let mut b = pos;
            let mut e = (pos + l) & 255;
            while l > 1 {
                e = (e+255) & 255;  // e--
                let n = bytes[b];
                bytes[b] = bytes[e];
                bytes[e] = n;
                b = (b+1) & 255;
                l -= 2;
            }
            pos = (pos + *len as usize + skip) & 255;
            skip += 1;
        }
    }
    // Reduce the final value
    let mut comp:Vec<u8> = vec![];
    for block in 0..16 {
        let mut x = 0;
        for b in 0..16 {
            x = x ^ bytes[block*16+b];
        }
        comp.push(x);
    }
    let p2 = hex::encode(comp);

    return p2;
}

fn main() {
    let test:Vec<u8> = vec![3, 4, 1, 5];
    assert!(process5(5, test.clone()) == 12);

    let fname = "input.txt"; // instead of args[1]
    let inp = fs::read_to_string(fname).expect("Error readin input file");


    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { 
        let input:Vec<u8> = inp.split(",").map(|x| x.parse::<u8>().unwrap()).collect();
        // vec![14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244];
        process(&input); 
        let inp:Vec<u8> = inp.as_bytes().to_vec();
        //"14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244".as_bytes().to_vec();
        let mut r2:Vec<u8> = inp;
        let app:Vec<u8> = vec![17, 31, 73, 47, 23];
        r2.extend(&app);
        round2(&r2);
    }); bench_result.print_stats();

    devtime.start();
    let input:Vec<u8> = inp.split(",").map(|x| x.parse::<u8>().unwrap()).collect();
    // vec![14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244];
    let part1 = process(&input);
    let inp:Vec<u8> = inp.as_bytes().to_vec();
    //"14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244".as_bytes().to_vec();
    let mut r2:Vec<u8> = inp;
    let app:Vec<u8> = vec![17, 31, 73, 47, 23];
    r2.extend(&app);
    let part2 = round2(&r2);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} ns",devtime.time_in_nanos().unwrap());
}