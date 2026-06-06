//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
use substring::Substring;

struct Ingredient
{
    _name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}
impl Ingredient
{
    fn new(name: &str, capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Ingredient
    {
        Ingredient{_name: name.to_string(), capacity, durability, flavor, texture, calories}
    }
}

fn process(inp:String) -> (i32, i32)
{
    let lines = inp.split("\n").collect::<Vec<&str>>();
    let mut ingr = Vec::new();
    for line in lines
    {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let name = parts[0].substring(0, parts[0].len()-1);
        let capacity = parts[2].substring(0, parts[2].len()-1).parse::<i32>().unwrap();
        let durability = parts[4].substring(0, parts[4].len()-1).parse::<i32>().unwrap();
        let flavor = parts[6].substring(0, parts[6].len()-1).parse::<i32>().unwrap();
        let texture = parts[8].substring(0, parts[8].len()-1).parse::<i32>().unwrap();
        let calories = parts[10].parse::<i32>().unwrap();
        ingr.push(Ingredient::new(name, capacity, durability, flavor, texture, calories));
    }
    assert!(ingr.len() == 4);
    let mut max_score = 0;
    let mut max_score_cal = 0;
    for i in 0..100 {
        for j in 0..100-i {
            for k in 0..100-i-j {
                let l = 100-i-j-k;
//                assert!(i+j+k+l == 100);
                let mut cap = i*ingr[0].capacity + j*ingr[1].capacity + k*ingr[2].capacity + l*ingr[3].capacity;
                let mut dur = i*ingr[0].durability + j*ingr[1].durability + k*ingr[2].durability + l*ingr[3].durability;
                let mut fla = i*ingr[0].flavor + j*ingr[1].flavor + k*ingr[2].flavor + l*ingr[3].flavor;
                let mut tex = i*ingr[0].texture + j*ingr[1].texture + k*ingr[2].texture + l*ingr[3].texture;
                let mut cal = i*ingr[0].calories + j*ingr[1].calories + k*ingr[2].calories + l*ingr[3].calories;
                if cap < 0 {cap = 0;}
                if dur < 0 {dur = 0;}
                if fla < 0 {fla = 0;}
                if tex < 0 {tex = 0;}
                if cal < 0 {cal = 0;}
                let score = cap*dur*fla*tex;
                if score > max_score {
                    max_score = score;
//                    println!("{} {} {} {} {} {}", i, j, k, l, score, cal);
                }
                if cal == 500 && score > max_score_cal {
                    max_score_cal = score;
//                    println!("500 cal: {} {} {} {} {}", i, j, k, l, score);
                }
            }
        }
    }
    (max_score,max_score_cal)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}