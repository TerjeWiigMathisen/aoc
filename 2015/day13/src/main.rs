//use std::collections::VecDeque;
use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
use itertools::Itertools;
use substring::Substring;

struct Happyiness
{
    name_map: HashMap<String, usize>,
    happy: Vec<Vec<i32>>,
}
impl Happyiness
{
    fn new() -> Happyiness
    {
        Happyiness
        {
            name_map: HashMap::new(),
            happy: Vec::new(),
        }
    }
    fn name_index(&mut self, name:&str) -> usize
    {
        if self.name_map.contains_key(name)
        {
            return *self.name_map.get(name).unwrap();
        }
        let index = self.name_map.len();
        self.name_map.insert(name.to_string(), index);
        return index;
    }
    fn add_happy(&mut self, fra:&str, til:&str, h:i32)
    {
        let fra_index = self.name_index(fra);
        let til_index = self.name_index(til);
        let max_index = if fra_index > til_index {fra_index} else {til_index};
        if self.happy.len() <= max_index
        {
            self.happy.resize(max_index+1, Vec::new());
        }
        for c in 0..self.happy.len()
        {
            if self.happy[c].len() <= max_index
            {
                self.happy[c].resize(max_index+1, 0);
            }
        }
        self.happy[fra_index][til_index] = h;
    }
}

fn process(inp:String) -> (i32, i32)
{
    let lines = inp.split("\n").collect::<Vec<&str>>();
    let mut part1 = 0;
    let mut part2 = 0;
    let mut cd: Happyiness = Happyiness::new();
    for line in lines
    {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let fra = parts[0];
        let til = parts[10].substring(0,parts[10].len()-1);
        let mut h = parts[3].parse::<i32>().unwrap();
        if parts[2] == "lose"
        {
            h = -h;
        }
        cd.add_happy(fra, til, h);
    }
    let guests = cd.name_map.len();
    // Generate all permutations of the cities
    let perms = (0..guests).permutations(guests);
    for p in perms 
    {
        let mut p1 = 0;
        for i in 0..p.len()-1
        {
            p1 += cd.happy[p[i]][p[i+1]];
            p1 += cd.happy[p[i+1]][p[i]];
        }
        let p2 = p1;
        p1 += cd.happy[p[p.len()-1]][p[0]];
        p1 += cd.happy[p[0]][p[p.len()-1]];
        if p1 > part1
        {
            part1 = p1;
        }
        if p2 > part2
        {
            part2 = p2;
        }
    }

    (part1, part2)
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