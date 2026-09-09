// day07
// Surface:
// Acer:   
use std::fs;
use devtimer::run_benchmark;

const CARD_RANGE:usize = 16;

//const RANK:[u8;256] = [0;49,2,3,4,5,6,7,8,9,0;7,14,0;8,11,13,0;5,12,0,0,10,0;255-b'T' as usize];
//const WILD:[u8;256] = [0;49,2,3,4,5,6,7,8,9,0;7,14,0;8, 1,13,0;5,12,0,0,10,0;255-b'T' as usize];

fn counts_to_rank(card_count:&[u8;CARD_RANGE]) -> usize
{
    let mut fst = 0;
    let mut scn = 0;
    let mut pos = 0;
    let wild = card_count[1] as usize;
    for i in 2..=14 {
        let cnt = card_count[i] as usize;
        if cnt == 0 {continue;}

        if cnt > fst {
            scn = fst;
            fst = cnt;
        }
    }
    ((fst+wild) << 56) + (scn << 52)
}

fn hand_rank(hand:&[u8], rank:&[u8;256]) -> (usize, usize)
{
	let mut card_count:[u8;CARD_RANGE] = [0;CARD_RANGE];
    let mut rank1 = 0;
    let mut rank2 = 0;
	for h in 0..5 { 
        let r = rank[hand[h] as usize] as usize;
        rank1 += r << (48-h*4);
        card_count[r] += 1; 
        let r = if r == 11 {1} else {r};
        rank2 += r << (48-h*4);
        wild_count[r] += 1; 
    }
    rank1 += counts_to_rank(&card_count);
    rank2 += counts_to_rank(&wild_count);
    (rank1, rank2)
}

fn win(ranked_hands:&Vec<usize>) -> usize
{
    let mut w = 0;
    for r in 0..ranked_hands.len() {
        w += (r+1) * (ranked_hands[r] & 0xffff);
    }
    w
}

pub fn process(inp:&str) -> (usize, usize)
{
    let input = inp.as_bytes();
    let mut i = 0;
    let mut rank:[u8;256] = [0;256];
    for b in b'2'..=b'9' {
        rank[b as usize] = b - b'0';
    }
    rank[b'T' as usize] = 10;
    rank[b'J' as usize] = 11;
    rank[b'Q' as usize] = 12;
    rank[b'K' as usize] = 13;
    rank[b'A' as usize] = 14;
    let mut regular:Vec<usize> = vec![];
    let mut jokers_wild:Vec<usize> = vec![];
    while i < input.len() {
        let (r1,r2) = hand_rank(&input[i..i+5], &rank);
        i += 6;
        let mut bid = (input[i] - b'0') as usize; i += 1;
        while input[i] >= b'0' {
            bid = bid * 10 + (input[i] - b'0') as usize; 
            i += 1;
        }
        regular.push(r1 + bid);
        jokers_wild.push(r2 + bid);
        i += 1;
    }
    regular.sort_unstable();
    jokers_wild.sort_unstable();
    let part1 = win(&regular);
    let part2 = win(&jokers_wild);
    (part1, part2)
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
