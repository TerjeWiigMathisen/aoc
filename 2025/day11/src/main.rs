// Fastest run HP 42.3 us, 2026-11-11

//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
//use rustc_hash::FxHashMap;

struct Node
{
    links: Vec<u16>,
}

struct Graph
{
    nodes: Vec<Node>,
    name2id: [u16;26*26*26],
}

impl Graph
{
    fn new() -> Graph
    {
        Graph{nodes: Vec::new(), name2id: [0;26*26*26]}
    }
    fn new_simple() -> Graph
    {
        let mut g = Graph::new();
        g.nodes.push(Node{links: Vec::new()}); // dummy node for 0 id
        g
    }
    fn get_index(&self, name:&[u8;3]) -> u16
    {
/*         println!("get_index: {}", name.iter().map(|&c| c as char).collect::<String>());
        assert!(name.len() == 3);
        assert!(name[0] >= b'a' && name[0] <= b'z');
        assert!(name[1] >= b'a' && name[1] <= b'z');
        assert!(name[2] >= b'a' && name[2] <= b'z');
 */        (name[0]-b'a') as u16 * 26*26 + (name[1]-b'a') as u16 * 26 + (name[2]-b'a') as u16
    }
    fn get_node_id(&mut self, name:&[u8;3]) -> u16
    {
        let idx = self.get_index(name);
        let mut id = self.name2id[idx as usize];
        if id == 0 {
            id = self.nodes.len() as u16;
            self.name2id[idx as usize] = id;
            self.nodes.push(Node{links: Vec::new()});
        }
        id
    }
    fn find(&self, start: u16, end: u16, cache: &mut Vec<u64>) -> u64
    {
//        println!("find: {} -> {}", start, end);
        if cache[start as usize] != u64::MAX {
            return cache[start as usize];
        }
        let mut paths = 0;
        let links = &self.nodes[start as usize].links;
        // println!("links: {:?}", links);
        for l in 0..links.len() {
            if links[l] == end {
                paths += 1;
                continue;
            }
            let cnt = self.find(links[l], end, cache);
            paths += cnt;
        }
        cache[start as usize] = paths;
        paths
    }
    fn find_except(&self, start: u16, end: u16, skip: u16, cache: &mut Vec<u64>) -> u64
    {
        // println!("find_except: {} -> {}, skipping {}", start, end, skip);
        if cache[start as usize] != u64::MAX {
            return cache[start as usize];
        }
        let mut paths = 0;
        let links = &self.nodes[start as usize].links;
        for l in 0..links.len() {
            if links[l] == end {
                paths += 1;
                continue;
            }
            if links[l] == skip {
                // println!("Skipping node {}", links[l]);
                continue;
            }
            let cnt = self.find_except(links[l], end, skip, cache);
            paths += cnt;
        }
        cache[start as usize] = paths;
        paths
    }
}

const SVR: [u8;3] = *b"svr";
const YOU: [u8;3] = *b"you";
const FFT: [u8;3] = *b"fft";
const DAC: [u8;3] = *b"dac";
const OUT: [u8;3] = *b"out";

fn process(inp:&str) -> (u64, u64)
{

//    let mut name2id:[u16;26*26*26] = [0;26*26*26];
    let mut graph:Graph = Graph::new_simple();
    let svr = graph.get_node_id(&SVR); // 1
    let you = graph.get_node_id(&YOU); // 2
    let fft = graph.get_node_id(&FFT); // 3
    let dac = graph.get_node_id(&DAC); // 4
    let out = graph.get_node_id(&OUT); // 5
//    println!("svr = {}, you = {}, fft = {}, dac = {}, out = {}: {}", svr, you, fft, dac, out, svr == 1 && you == 2 && fft == 3 && dac == 4 && out == 5);

    let mut i = 0;
    let input = inp.as_bytes();
    while i < input.len()
    {
        let _idx = graph.get_node_id(input[i..i+3].try_into().unwrap());
        let mut links: Vec<u16> = Vec::new();
        i += 4;
        while i < input.len() && input[i] != b'\n'
        {
            let tgt = graph.get_node_id(input[i+1..i+4].try_into().unwrap());
            links.push(tgt);
            i += 4;
        }
//        println!("Node {}: {:?}", _idx, links);
        // if _idx <= 5 {
        //     println!("Node {}: {} {:?}", _idx, input[i..i+3].iter().map(|&c| c as char).collect::<String>(), links);
        // }
        graph.nodes[_idx as usize] = Node{links};
        i += 1; // skip '\n'
    }
    let mut cache:Vec<u64> = vec![u64::MAX; graph.nodes.len()];
    let part1 = graph.find(you, out, &mut cache); // find paths from YOU to OUT

//    return (part1, 0);

    cache = vec![u64::MAX; graph.nodes.len()];
    let dac_fft = graph.find(dac, fft, &mut cache); // find paths from DAC to FFT
//    println!("dac_fft = {}", dac_fft);
    let part2;
    if dac_fft > 0 {
        cache = vec![u64::MAX; graph.nodes.len()];
        let svr_dac = graph.find_except(svr, dac, fft, &mut cache); // find paths from SVR to DAC excluding FFT
        cache = vec![u64::MAX; graph.nodes.len()];
        let fft_out = graph.find_except(fft, out, dac, &mut cache); // find paths from FFT to OUT excluding DAC
        part2 = svr_dac * dac_fft * fft_out;    // paths from SVR via DAC and FFT to OUT
    }
    else {
        cache = vec![u64::MAX; graph.nodes.len()];
        let fft_dac = graph.find(fft, dac, &mut cache); // find paths from FFT to DAC
        // println!("fft_dac = {}", fft_dac);
        cache = vec![u64::MAX; graph.nodes.len()];
        let svr_fft = graph.find_except(svr, fft, dac, &mut cache); // find paths from SVR to FFT excluding DAC
        // println!("svr_fft = {}", svr_fft);
        cache = vec![u64::MAX; graph.nodes.len()];
        let dac_out = graph.find_except(dac, out, fft, &mut cache); // find paths from DAC to OUT excluding FFT
        // println!("dac_out = {}", dac_out);
        part2 = svr_fft * fft_dac * dac_out;    // paths from SVR via FFT and DAC to OUT
    }
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != '\n' as u8 {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}