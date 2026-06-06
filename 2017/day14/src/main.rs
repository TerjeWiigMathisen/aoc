//use std::collections::HashMap;
//use std::io;
//use std::env;
//use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use hex::encode;

fn knot_hash(seq:&Vec<u8>) -> Vec<u8>
{
    let mut bytes:Vec<u8> = vec![];
    let mut lseq = seq.to_owned();
    lseq.extend(vec![17, 31, 73, 47, 23]);
    for i in 0..256 { bytes.push(i as u8); }

    let mut pos: usize = 0;
    let mut skip: usize = 0;
    for _ in 0..64 {
        for len in lseq.clone() {
            let mut le = len as i32;
            let mut b = pos;
            let mut e = (pos + le as usize + 255) & 255;
            if e > b {
                // while le >= 8  {
                //     let b8 = u64::from_be_bytes(bytes[b..b+8].try_into().unwrap());
                //     let e8 = u64::from_be_bytes(bytes[e-7..=e].try_into().unwrap());
                //     bytes[b..b+8].copy_from_slice(&e8.to_le_bytes());
                //     bytes[e-7..=e].copy_from_slice(&b8.to_le_bytes());

                //     b = (b+8) & 255;
                //     e = (e+248) & 255;
                //     le = -16;
                // }
                while le >= 4  {
                    let b4 = u32::from_be_bytes(bytes[b..b+4].try_into().unwrap());
                    let e4 = u32::from_be_bytes(bytes[e-3..=e].try_into().unwrap());
                    bytes[b..b+4].copy_from_slice(&e4.to_le_bytes());
                    bytes[e-3..=e].copy_from_slice(&b4.to_le_bytes());

                    b = (b+4) & 255;
                    e = (e+252) & 255;
                    le -= 8;
                }
            }
            while le > 1 {
                let n = bytes[b];
                bytes[b] = bytes[e];
                bytes[e] = n;
                b = (b+1) & 255;
                e = (e+255) & 255;  // e--
                le -= 2;
            }
            pos = (pos + len as usize + skip) & 255;
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
    return comp;
}

fn bits(hash:Vec<u8>)->usize
{
    let mut b=0;
    for i in 0..hash.len() {
        let mut h = hash[i];
        while h != 0 {
            h &= h-1;
            b += 1;
        }
    }
    return b;
}

fn is_set(bitmap:&mut Vec<Vec<u8>>, line:usize, col:usize)->bool
{
    return bitmap[line][col >> 3] & (0x80 >> (col&7)) != 0;
}

fn bit_clear(bitmap:&mut Vec<Vec<u8>>, line:usize, col:usize)
{
    bitmap[line][col >> 3] ^= 0x80>>(col&7);
}

fn flood_clear(bitmap:&mut Vec<Vec<u8>>, line:usize, col:usize)
{
    if is_set(bitmap, line, col) {
        bit_clear(bitmap, line, col);
        if col > 0 {flood_clear(bitmap, line, col-1);}
        if col < 127 {flood_clear(bitmap, line, col+1);}
        if line > 0 {flood_clear(bitmap, line-1, col);}
        if line < 127 {flood_clear(bitmap, line+1, col);}
    }
}

fn process1(key:String) -> (usize,usize)
{
    let mut p1=0;
    let mut bitmap:Vec<Vec<u8>> = vec![vec![0;32];128];

    for i in 0..128 {
        let ln = format!("{key}-{i}");
        let hash_key = ln.as_bytes().to_vec();
        let hash = knot_hash(&hash_key);
        for j in 0..16 {
            bitmap[i][j] = hash[j];
        }
        p1 += bits(hash);
    }
    let mut p2 = 0;
    for line in 0..128 {
        for col in 0..128 {
            if is_set(&mut bitmap, line,col) {
                p2 += 1;
                flood_clear(&mut bitmap,line,col);
            }
        }
    }
    return (p1,p2);
}

fn main() {
    assert!(process1("flqrgnkx".to_owned().to_string()) == (8108,1242));

    let input = String::from("ljoxqyyw");

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process1(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process1(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} ns",devtime.time_in_nanos().unwrap());
}