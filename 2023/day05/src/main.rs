// day05
// Surface:
// Acer:        8.1 us
use std::fs;
use devtimer::run_benchmark;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MapRange {
    src:u32,
    dst:u32,
    len:u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SeedRange {
    src:u32,
    len:u32,
}

pub fn process(inp:&String)->(u32, u32)
{
    let input = inp.as_bytes();
    let ilen = input.len();
    let mut i = 7;
    let mut seeds:Vec<u32> = Vec::new();
    loop {
        let mut n = (input[i] - b'0') as u32; i += 1;
        while input[i] >= b'0' && input[i] <= b'9' {
            n = n * 10 + (input[i] - b'0') as u32;
            i += 1;
        }
        seeds.push(n);
        if input[i] == b'\n' {break;}
        i += 1;
    }
    let mut seeds2:Vec<SeedRange> = Vec::with_capacity(seeds.len()/2);
    let mut s = 0;
    while s < seeds.len() {
        seeds2.push(SeedRange { src: seeds[s], len: seeds[s+1] });
        s += 2;
    }
    seeds.sort_unstable();
//    println!("seeds={:?}", seeds);
    seeds2.sort_unstable();

    loop {
        while input[i] != b':' { i += 1; }
        i += 2; // Skip ":\n"
        let mut ranges:Vec<MapRange> = Vec::new();
        loop {
            let mut d;
            let mut s = 0;
            let mut l = 0;
            loop {
                let mut n = (input[i] - b'0') as u32; i += 1;
                while input[i] >= b'0' && input[i] <= b'9' {
                    n = n * 10 + (input[i] - b'0') as u32;
                    i += 1;
                }
                d = s; s = l; l = n;
                if input[i] == b'\n' {i += 1; break;}
                i += 1; // Skip space
            }
            ranges.push(MapRange { src: s, dst: d, len: l });
//            println!("range={:?}", ranges.last().unwrap());
            if i >= ilen || input[i] == b'\n' {break}
        }
        ranges.sort_unstable();
        ranges.push(MapRange { src: u32::MAX, dst: u32::MAX, len: 0 }); // Guard!

        let mut mappedseeds:Vec<u32> = Vec::new();
        let mut r = 0;
        let mut s = 0;
        while s < seeds.len() {
            while seeds[s] >= ranges[r].src + ranges[r].len {
                r += 1;
            }
            if seeds[s] >= ranges[r].src {
                mappedseeds.push(seeds[s] - ranges[r].src + ranges[r].dst);
            } else {
                mappedseeds.push(seeds[s]);
            }
            s += 1;
        }
        seeds = mappedseeds;
        seeds.sort_unstable();

        let mut mappedseeds2:Vec<SeedRange> = Vec::new();
        let mut r = 0;
        let mut s = 0;
        while s < seeds2.len() {
            while seeds2[s].src >= ranges[r].src + ranges[r].len {
                r += 1;
            }
            if seeds2[s].src >= ranges[r].src {
                if seeds2[s].src+seeds2[s].len > ranges[r].src + ranges[r].len {
                    // "Seed range overlaps mapping range, so split it into two ranges"
                    let overlap = seeds2[s].src + seeds2[s].len - (ranges[r].src + ranges[r].len);
                    mappedseeds2.push(SeedRange { src: seeds2[s].src - ranges[r].src + ranges[r].dst, len: seeds2[s].len - overlap });
                    seeds2[s].src = ranges[r].src + ranges[r].len;
                    seeds2[s].len = overlap;
                }
                else { // "Seed range is fully contained in mapping range, so map it"
                    mappedseeds2.push(SeedRange { src: seeds2[s].src - ranges[r].src + ranges[r].dst, len: seeds2[s].len });
                    s += 1;
                }
            } else { // "Seed range is before mapping range, so keep it as is"
                mappedseeds2.push(SeedRange { src: seeds2[s].src, len: seeds2[s].len });
                s += 1;
            }
        }
        seeds2 = mappedseeds2;
        seeds2.sort_unstable();

        //        println!("mapped seeds={:?}", seeds);
        i += 1; // Skip empty line
        if i >= ilen || input[i] == b'\n' {break;}
    }
    let part1 = seeds[0];
    let part2 = seeds2[0].src;
    return (part1, part2);
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
//    let fname = "test.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != b'\n' as u8 {input.push('\n');}

    let bench_result = run_benchmark(1000, |_| {
        process(&input);
    });
    bench_result.print_stats();

    let res = process(&input);
    println!("part1={}\npart2={}", res.0, res.1);
}
