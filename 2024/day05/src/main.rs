// 10.867 us Acer, Fastest average over 1000 runs

use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

#[derive(Clone)]
struct Pagemap {
    behind:u128,
    infront:u128,
}

impl Pagemap {
    fn new() -> Pagemap {
        Pagemap {
            behind: 0,
            infront: 0,
        }
    }
    fn add_behind(&mut self, page:u8) {
        self.behind |= 1 << page;
    }
    fn add_infront(&mut self, page:u8) {
        self.infront |= 1 << page;
    }
}

struct Pages {
    pages:Vec<Pagemap>,
}

impl Pages {
    fn new() -> Pages {
        Pages {
            pages: vec![Pagemap::new();100],
        }
    }
    fn add_rule(&mut self, front:u8, behind:u8) {
        self.pages[front as usize].add_behind(behind);
        self.pages[behind as usize].add_infront(front);
    }
    fn is_ordered(&self, pagelist:&Vec<u8>) -> bool {
        let mut prev = pagelist[0];
        for p in 1..pagelist.len() {
            let curr = pagelist[p];
            if self.pages[prev as usize].infront & (1 << curr) != 0 {
                return false;
            }
            prev = curr;
        }
        true
    }
    fn order(&self, pagelist:&Vec<u8>) -> u8 {
        //let mut ordered = Vec::new();
        let mut unordered = pagelist.clone();
        let mut pagemask:u128 = 0;
        let mut target_page = 1+(pagelist.len()>>1);
        for p in pagelist.iter() {
            pagemask |= 1 << p;
        }
        let mut target = 0;
        while target_page > 0 {
            for index in 0..unordered.len() {
                let page = unordered[index];
                if self.pages[page as usize].behind & pagemask == 0 {
                    pagemask &= !(1 << page);
                    unordered.swap_remove(index);
                    target = page;
                    break;
                }
            }
            target_page -= 1;
        }
        target as u8
    }
    fn median(&self, pagelist:&mut Vec<u8>) -> u8 {
        let mut l0 = 0;
        let mut r0 = pagelist.len()-1;
        let med = (r0>>1) + 1;
//        println!("pagelist:{:?}", pagelist);
        while l0 < r0 {
            let (mut l, mut r) = (l0+1,r0);
            let pivot = pagelist[l0];
//            let pivot = pagelist[(l+r)>>1];
            let behind = self.pages[pivot as usize].behind;
            loop {
                while l < r && ((1 as u128) << pagelist[l]) & behind == 0 { l += 1;}
                while l < r && ((1 as u128) << pagelist[r]) & behind != 0 { r -= 1;}
                if l >= r {break;}
//                println!("Pivot = {pivot} Swapping {l} and {r}");
                let t = pagelist[l];
                pagelist[l] = pagelist[r];
                pagelist[r] = t;
            }
            if l >= med { r0 = l-1} else {l0 = r+1}
//            println!("l0:{l0}, r0:{r0}, pagelist:{:?}", pagelist);
        }
        pagelist[med]
    }
}

fn twodig_to_u8(s:&[u8]) -> u8 {
//    assert!(s[0] >= b'0' && s[0] <= b'9', "First char {}", s[0]);
    (s[0]-b'0')*10 + s[1] - b'0'
}

#[unsafe(no_mangle)]
fn process(inp:&str) -> (i32, i32)
{
    let mut pages = Pages::new();
    let mut part1 = 0;
    let mut part2 = 0;

    let input = inp.as_bytes();
    let mut i = 0;
    while input[i] != b'\n' { // First part ends with a blank line
        let (f,b) = (twodig_to_u8(&input[i..i+2]), twodig_to_u8(&input[i+3..i+5]));
        pages.add_rule(f, b);
        i += 6;         // Jump to next line
    }
    i += 1; // Skip the blank line

    let mut pagelist:Vec<u8> = vec![];
    let llen = input.len() - 2;
    while i < llen {
        loop {
            let n = twodig_to_u8(&input[i..i+2]);
            pagelist.push(n);
            i += 3;
            if input[i-1] == b'\n' {break;}
        }

//    for line in _pages.lines() {
//        let pagelist:Vec<u32> = line.split(",").map(|x| twodigits_to_u32(x)).collect();
        let plen = pagelist.len();
        if pages.is_ordered(&pagelist) {
            part1 += pagelist[plen>>1] as i32;
        }
        else {
            part2 += pages.order(&pagelist) as i32;
//            part2 += pages.median(&mut pagelist) as i32;
        }
        pagelist.clear();
    }

    (part1, part2)
}

fn process1k(i:&str) -> (i32,i32)
{
    for _i in 0..1000 {
        process(i);
    }
    (0,0)
}

pub fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let fname: &str = if args.len() < 2 { "input.txt" } else { args[1].as_str() };
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] != b'\n' {input.push('\n');}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { process1k(&input); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}