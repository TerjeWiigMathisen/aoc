// Fastest run with bitshift lookup: 35.0 us
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

fn _nice_string(s: &str) -> i32 {
    let s = s.as_bytes();
    let mut vowels = 0;
    let mut double = false;
    let mut last = 0;
    for c in s.iter() {
        match c {
            b'a' | b'e' | b'i' | b'o' | b'u' => vowels += 1,
            b'b' | b'd' | b'q' | b'y' => if last+1 == *c {return 0;}
           _ => {}
        }
        if *c == last { double = true; }
        last = *c;
    }
    if vowels >= 3 && double { 1 } else { 0 }
}

//                                a b c d e f g h i j k l m n o p q r s t u v w x y z _ _ _ _ _
const CHARTYPE_TABLE:[u8;32] = [0,1,2,0,2,1,0,0,0,1,0,0,0,0,0,1,0,2,0,0,0,1,0,0,0,2,0,0,0,0,0,0];

const VOWELS:u32 = 1<<(b'a'&31) | 1<<(b'e'&31) | 1<<(b'i'&31) | 1<<(b'o'&31) | 1<<(b'u'&31);
const STEP1:u32 = 1<<(b'b'&31) | 1<<(b'd'&31) | 1<<(b'q'&31) | 1<<(b'y'&31);

fn _nice_string_shift(s: &str) -> i32 {
    let s = s.as_bytes();
    let mut vowels = 0;
    let mut double = false;
    let mut last = 0;
    for i in 0..s.len() {
        let c = s[i] & 31;
        vowels += (VOWELS >> c) & 1;
        if (STEP1 >> c) & 1 != 0 && last+1 == c { return 0; }
        if c == last { double = true; }
        last = c;
    }
    (vowels >= 3 && double) as i32
}


fn _nice_string_table(s: &str) -> i32 {
    let s = s.as_bytes();
    let mut vowels = 0;
    let mut double = false;
    let mut last = 0;
    for i in 0..s.len() {
        let c = s[i] & 31;
        let ct = CHARTYPE_TABLE[c as usize];
        vowels += ct & 1;
        if (ct & 2) != 0 && last+1 == c { return 0; }
        if c == last { double = true; }
        last = c;
    }
    (vowels >= 3 && double) as i32
}

fn _nice_string2(s: &str) -> i32 {
    let s = s.as_bytes();
    let mut a_a = false;
    for i in 2..s.len() {
        if s[i] == s[i-2] { 
            a_a = true; 
            break;
        }
    }
    if !a_a { return 0; }
    for i in 0..s.len()-3 {
        let s2:u16 = s[i] as u16 + ((s[i+1] as u16)<<8);
        for j in i+2..s.len()-1 {
            let j2:u16 = s[j] as u16 + ((s[j+1] as u16)<<8);
            if s2 == j2 { 
                return 1;
            }
        }
    }
    0
}

fn _nice_string12(s: &str) -> (i32,i32) {
    let s = s.as_bytes();
    let mut vowels = 0;
    let mut double = false;
    let mut bad = false;
    let mut pair_skip1 = false;
    let mut twopairs = false;
    let mut prepre = 0;
    let mut pre = 0;
    for i in 0..s.len() {
        let c = s[i];
        if c == prepre { pair_skip1 = true; }
        match c {
            b'a' | b'e' | b'i' | b'o' | b'u' => vowels += 1,
            b'b' | b'd' | b'q' | b'y' => if pre+1 == c {bad = true;}
           _ => {}
        }
        if c == pre { double = true; }
        for j in i+1..s.len()-1 {
            if pre == s[j] && c == s[j+1] { twopairs = true; }
        }
        prepre = pre;
        pre = c;
    }
    ((vowels >= 3 && double && !bad) as i32, (pair_skip1 && twopairs) as i32)
}


fn process(inp:String) -> (i32, i32)
{
    let mut part1 = 0;
    let mut part2 = 0;
    let lines = inp.lines();
    for line in lines {
        //part1 += _nice_string_table(&line);
        part1 += _nice_string_shift(&line);
        part2 += _nice_string2(&line);
        // let (p1,p2) = nice_string12(&line);
        // part1 += p1;
        // part2 += p2;
    }
    (part1,part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}