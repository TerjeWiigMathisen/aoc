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

struct CityDist
{
    city_map: HashMap<String, usize>,
    city_dist: Vec<Vec<usize>>,
}
impl CityDist
{
    fn new() -> CityDist
    {
        CityDist
        {
            city_map: HashMap::new(),
            city_dist: Vec::new(),
//            visited:Vec<u8>,
        }
    }
    fn city_index(&mut self, city:&str) -> usize
    {
        if self.city_map.contains_key(city)
        {
            return *self.city_map.get(city).unwrap();
        }
        let index = self.city_map.len();
        self.city_map.insert(city.to_string(), index);
        return index;
    }
    fn add_dist(&mut self, fra:&str, til:&str, dist:usize)
    {
        let fra_index = self.city_index(fra);
        let til_index = self.city_index(til);
        let max_index = if fra_index > til_index {fra_index} else {til_index};
        if self.city_dist.len() <= max_index
        {
            self.city_dist.resize(max_index+1, Vec::new());
        }
        for c in 0..self.city_dist.len()
        {
            if self.city_dist[c].len() <= max_index
            {
                self.city_dist[c].resize(max_index+1, 0);
            }
        }
        self.city_dist[fra_index][til_index] = dist;
        self.city_dist[til_index][fra_index] = dist;
    }
}

fn process(inp:String) -> (usize, usize)
{
    let lines = inp.split("\n").collect::<Vec<&str>>();
    let mut part1 = usize::MAX;
    let mut part2 = 0;
    let mut cd: CityDist = CityDist::new();
    for line in lines
    {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let fra = parts[0];
        let til = parts[2];
        let dist = parts[4].parse::<usize>().unwrap();
        cd.add_dist(fra, til, dist);
    }
    let cities = cd.city_map.len();
    // Generate all permutations of the cities
    let perms = (0..cities).permutations(cities);
    for p in perms 
    {
        let mut dist = 0;
        for i in 0..p.len()-1
        {
            dist += cd.city_dist[p[i]][p[i+1]];
        }
        if dist < part1
        {
            part1 = dist;
        }
        if dist > part2
        {
            part2 = dist;
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