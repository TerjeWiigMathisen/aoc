//use std::io;
//use std::env;
use std::fs;
//use substring::Substring;
//use std::collections::HashMap;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;

/*
fn place(start:usize, piecesleft:usize, pieces:&mut Vec<u64>)->usize
{
    let mut _count = 0;
    let mut p = pieces[start];
    for b in 0..64 {
        let bit = 1 << b;
        if p & bit != 0 {
            pieces[start] = p & !bit;
            let c:usize = start + b + place(b, piecesleft-1, pieces);
            if c > _count {
                _count = c;
//                println!("Found {c} with {piecesleft} pieces left")
            }
            pieces[start] = p;
        }
        else {
//            println!("no more: {piecesleft}");
        }
    }
    let bit = 1 << start;
    for b in 0..64 {
        p = pieces[b];
        if p & bit != 0 {
            pieces[b] = p & !bit;
            let c:usize = start + b + place(b, piecesleft-1, pieces);
            if c > _count {
                _count = c;
//                println!("Found {c} with {piecesleft} pieces left")
            }
            pieces[b] = p;
        }
        else {
//            println!("no more: {piecesleft}");
        }
    }
    return _count;
}

fn place2(startpins:usize, curr:usize, max:&mut usize, pieces:&mut Vec<u64>)->usize
{
    let mut _count = 0;
    let mut p = pieces[startpins];
    if curr > *max {
        *max = curr;
        //println!("Found {curr:x}")
    }
    for b in 0..64 {
        let bit = 1 << b;
        if p & bit != 0 {
            pieces[startpins] = p & !bit;
            _count = place2(b, curr + startpins + b + (1 << 32), max, pieces);
            pieces[startpins] = p;
        }
        else {
//            println!("no more: {piecesleft}");
        }
    }
    let bit = 1 << startpins;
    for b in 0..64 {
        p = pieces[b];
        if p & bit != 0 {
            pieces[b] = p & !bit;
            _count = place2(b, curr + startpins + b + (1 << 32), max, pieces);
            pieces[b] = p;
        }
        else {
//            println!("no more: {piecesleft}");
        }
    }
    return *max;
}

fn process(inp:String) -> (usize,usize)
{
    let input = inp.clone();
    let lines:Vec<&str> = input.split("\n").collect();
//    let mut rules:HashMap<Vec<u8>,Vec<u8>> = HashMap::new();

    let mut pieces:Vec<u64> = vec![0;64];

    let mut pcnt = 0;
    for li in lines {
        //println!("li={li}");
        let sd:Vec<usize> = li.split("/").map(|x| x.parse::<usize>().unwrap()).collect();
        let (p, bit) = (sd[0], 1 << sd[1]);
        if pieces[p] & bit != 0 { println!("Duplicate piece {li}");}
        pieces[p] |= bit;
        pcnt += 1;
    }
    let count = place(0,pcnt, &mut pieces);
    let mut p2:usize = 0;
    let mut s2 = place2(0, 0, &mut p2, &mut pieces);
    s2  &= 0xffffffff;
//    println!("Found a path with value {count}");
    return (count, s2);
}
*/
struct Piece {
    front:u16,
    back:u16,
    used:bool,
}

#[derive(Clone)]
struct Index {
    id:u16,
    other:u16,
}

fn put(pieces:&mut Vec<Piece>, idx:&Vec<Vec<Index>>, pins:usize, strength:usize, lenstrength:usize) -> (usize,usize)
{
    let ip = &idx[pins];
    let mut part1 = strength;
    let mut maxlenstrength = lenstrength;
    let strength = strength+pins;
    let lenstrength = lenstrength+pins+(1<<32);
    for p in 0..ip.len() {
        let pi = &ip[p];
        let id = pi.id as usize;
        if pieces[id].used {continue;}
        pieces[id].used = true;
        let pio = pi.other as usize;
        //println!(" {}-{} ",pins,pio);
        let (p1,p2) = put(pieces,idx,pio, strength+pio, lenstrength+pio);
        pieces[id].used = false;
        if p1 > part1 {part1 = p1;}
        if p2 > maxlenstrength { maxlenstrength = p2;}
    }
    //println!("{part1}, {}:{}", maxlenstrength >> 32, maxlenstrength & 0xffff);
    (part1, maxlenstrength)
}

fn put1(pieces:&mut Vec<Piece>, idx:&Vec<Vec<Index>>, pins:usize, len:usize, strength:usize) -> (usize,usize)
{
    let ip = &idx[pins];
    let mut maxlen = len;
    let mut maxstrength = strength;
    let strength = strength+pins;
    for p in 0..ip.len() {
        let pi = &ip[p];
        let id = pi.id as usize;
        if pieces[id].used {continue;}
        pieces[id].used = true;
        let pio = pi.other as usize;
        let (l,s) = put1(pieces,idx,pio, len+1, strength+pio);
        pieces[id].used = false;
        if l > maxlen {maxlen = l;}
        if s > maxstrength { maxstrength = s;}
    }
    //println!("{len}: {maxlen},{maxstrength}");
    (maxlen,maxstrength)
}

fn put2(pieces:&mut Vec<Piece>, idx:&Vec<Vec<Index>>, pins:usize, len:usize, strength:usize) -> (usize,usize)
{
    let ip = &idx[pins];
    let mut maxlen = len;
    let mut maxstrength = strength;
    let startstrength = strength+pins;
    for p in 0..ip.len() {
        let pi = &ip[p];
        let id = pi.id as usize;
        if pieces[id].used {continue;}
        pieces[id].used = true;
        let pio = pi.other as usize;
        let (l,s) = put2(pieces,idx,pio, len+1, strength+pio);
        pieces[id].used = false;
        if l > maxlen {maxlen = l; maxstrength = s;}
        else if l == maxlen && s > maxstrength { maxstrength = s;}
    }
    //println!("{len}: {maxlen},{maxstrength}");
    (maxlen,maxstrength)
}

fn pro(inp:String) -> (usize,usize)
{
    let input = inp.clone();
    let lines:Vec<&str> = input.split("\n").collect();
//    let mut rules:HashMap<Vec<u8>,Vec<u8>> = HashMap::new();

    let mut pieces:Vec<Piece> = Vec::new(); // = vec![Piece{id:0,};0];

    let mut maxpins = 0;
    for li in lines {
        let sd:Vec<u16> = li.split("/").map(|x| x.parse::<u16>().unwrap()).collect();
        let (f,b) = (sd[0],sd[1]);
        //println!("{}: {f},{b}",pieces.len()); 
        let p  = Piece{front:f, back:b, used:false};
        pieces.push(p);
        if f > maxpins {maxpins = f;}
        if b > maxpins {maxpins = b;}
    }
    let mut idx:Vec<Vec<Index>> = vec![vec![Index{id:0,other:0};0]; maxpins as usize +1];

    for p in 0..pieces.len() {
        let pi = &pieces[p];
        let ix = Index{id:p as u16,other:pi.back};
        idx[pi.front as usize].push(ix);
        if pi.back == pi.front {continue;}
        let ix = Index{id:p as u16, other:pi.front};
        idx[pi.back as usize].push(ix);
    }
    for i in 0..0{ //idx.len() {
        print!("{i}: ");
        for j in 0..idx[i].len() {
            print!(" {} ",idx[i][j].id);
        }
        println!("");
    }
    let (part1, part2len) = put(&mut pieces,&idx,0,0,0);
//    let (_l1, part1) = put1(&mut pieces,&idx,0,0,0);
//    let (_l2, part2) = put2(&mut pieces,&idx,0,0,0);
let part2 = part2len & 0xffff;
    (part1, part2)

}

fn main() {

    assert!(pro(
"0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10".to_owned()) == (31,19));
    let fname = "input.txt"; // instead of args[1]
    let input = fs::read_to_string(fname).expect("Error readin input file");
    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| {
        pro(input.clone());
    });
    bench_result.print_stats();

    devtime.start();
    let (part1,part2) = pro(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}