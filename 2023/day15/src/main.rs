// day15
// Surface:
// Acer:   95.9 us

use std::fs;
use devtimer::run_benchmark;

fn hash(b:u8, hash: usize) -> usize
{
    let mut hash = hash;
    hash = (hash + b as usize) * 17 % 256;
    hash
}

fn hash_sum(inp:&[u8]) -> usize
{
    let mut i = 0;
    let mut part1 = 0;
    while i < inp.len() {
        let mut h = 0;
        loop {
            let b = inp[i]; i += 1;
            if b == b',' { break; }
            h = hash(b, h);
        }
        part1 += h;
    }
    part1
}

#[derive(Clone, Copy)]
struct Tag {
    o:&'static str,
    val:usize,
}

#[derive(Clone)]
struct Onebox {
    tags:Vec<Tag>,
}

fn hash_map(inp:&'static str) -> usize
{
    let mut i:usize = 0;
    let mut part2 = 0;
    let mut boxes:Vec<Onebox> = vec![];
    for _ in 0..256 {
        boxes.push(Onebox {tags:vec![]});
    }
    let input = inp.as_bytes();
    while i < input.len() {
//        println!("At {i}, next input: {}", input[i] as char);
        let mut h = 0;
        let start = i;
        loop {
            let b = input[i]; i += 1;
            if b == b'-' || b == b'=' { break; }
            h = hash(b, h);
        }
        let op = input[i-1];
        let tag = &inp[start..i-1]; // Use the slice in place
        let tags = &mut boxes[h].tags;
        if op == b'-' { // Must be '-', remove matching lens if found
            for t in 0..tags.len() {
                if tag == tags[t].o {
                    tags.remove(t);
                    break;
                }
            }
            i += 1;
        }
        else { // op == b'='
            let v = (input[i] - b'0') as usize; i += 2;
            let mut replaced = false;
            for t in 0..tags.len() {
                if tag == tags[t].o {
                    tags[t].val = v;
                    replaced = true;
                    break;
                }
            }
            if !replaced { tags.push(Tag{o:tag, val:v})}
        }
    }
    for b in 1..=256 {
        let tags = &boxes[b-1].tags;
        for slot in 1..=tags.len() {
            let focus = b*slot*tags[slot-1].val;
            part2 += focus;
        }
    }
    part2
}

fn process(inp:&'static str)->(usize, usize)
{
    let input = inp.as_bytes();
    let part1 = hash_sum(&input);
    let part2 = hash_map(&inp);
    (part1, part2)
}

fn main() {
//    let fname = "test.txt"; // instead of args[1]
    let fname = "input.txt"; // instead of args[1]
    let mut input:String = fs::read_to_string(fname).expect("Error reading input file");
    if input.ends_with('\n') { input.pop(); }
    if !input.ends_with(',') { input.push(','); }

    let input:&'static str = Box::leak(input.into_boxed_str());

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let res = process(input);
    println!("part1={}\npart2={}", res.0, res.1);
}
