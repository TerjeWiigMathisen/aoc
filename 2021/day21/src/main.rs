// Fastest run (Acer): 4.5 us with u8 vars
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

#[derive(Copy, Clone, Debug)]
struct Winlose {
    win:usize,
    lose:usize,
}

const STEPS:[usize;10] = [0,0,0,1,3,6,7,6,3,1];

fn step(pos1:u32, rest1:u32, pos2:u32, rest2:u32, cache:&mut [Winlose;10*10*21*21]) -> Winlose
{
    let key = (pos1-1 + (pos2-1) * 10 +
            (rest1-1) * 100 + (rest2-1) * 2100) as usize;
    // if level > 20 {
    //     for _ in 0..level {
    //         print!(" ");
    //     }
    //     println!("{level} -> {pos1}:{rest1} - {pos2}:{rest2}");
    // }
    if cache[key].win != usize::MAX {return cache[key];}

    let mut wl = Winlose{win:0,lose:0};
    for s in 3..=9 {
        let sc = STEPS[s];
        let mut npos = pos1 + s as u32; // - 1) % 10 + 1;
        if npos > 10 { npos -= 10;}
        if npos >= rest1 {
            wl.win += sc;
//            println!("Add {sc}");
        }
        else {
            let lw = step(pos2, rest2, npos, rest1-npos, cache);
            wl.win += lw.lose * sc;
            wl.lose += lw.win * sc;
        }
    }
    cache[key] = wl;
    wl
}

fn process(inp:&str) -> (usize, usize)
{
    let mut pos:Vec<u32> = Vec::new();
    for line in inp.lines() {
        let tokens = line.split_ascii_whitespace();
        pos.push(tokens.skip(4).next().unwrap().parse::<u32>().unwrap());
    }
    let save1 = pos[0];
    let save2 = pos[1];
    let mut part1 = 0;
    let mut die = 1;
    let mut rolls = 0;
    let mut sum:[u32;2] = [0;2];
    loop {
        for player in 0..2 {
            let mut p = pos[player];
            for _d in 0..3 {
                p = (p + die - 1) % 10 + 1;
                die = (die % 100) + 1;
                rolls += 1;
            }
            pos[player] = p;
            sum[player] += p;
            if sum[player] >= 1000 {
                part1 = (sum[1-player] * rolls) as usize;
                break;
            }
        }
        if part1 != 0 {break;}
    }
    pos[0] = save1;
    pos[1] = save2;
    let mut cache:[Winlose;10*10*21*21] = [Winlose{win:usize::MAX, lose:usize::MAX};10*10*21*21];
    let wl = step(pos[0],21, pos[1], 21, &mut cache);
    let part2 = if wl.win >= wl.lose {wl.win} else {wl.lose};
    (part1, part2)
}

fn main() {
//    let bench_result = run_benchmark(10, |_| {bench_permute();}); bench_result.print_stats();

    //panic!("Stop now");

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(&input); }); bench_result.print_stats();

    //process(&input);
    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}