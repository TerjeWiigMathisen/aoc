// 113 us

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
    fn add_behind(&mut self, page:u32) {
        self.behind |= 1 << page;
    }
    fn add_infront(&mut self, page:u32) {
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
    fn add_rule(&mut self, front:u32, behind:u32) {
        self.pages[front as usize].add_behind(behind);
        self.pages[behind as usize].add_infront(front);
    }
    fn is_ordered(&self, pages:Vec<u32>) -> bool {
        let mut prev = pages[0];
        for p in 1..pages.len() {
            let curr = pages[p];
            if self.pages[prev as usize].infront & (1 << curr) != 0 {
                return false;
            }
            prev = curr;
        }
        true
    }
    fn order(&self, pages:Vec<u32>) -> i32 {
        //let mut ordered = Vec::new();
        let mut unordered = pages.clone();
        let mut pagemask:u128 = 0;
        let mut target_page = 1+(pages.len()>>1);
        for p in pages.iter() {
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
        target as i32
    }
}

fn twodigits_to_u32(s:&str) -> u32 {
    ((s.as_bytes()[0] - b'0')*10 + (s.as_bytes()[1] - b'0')) as u32
}

#[no_mangle]
fn process(inp:String) -> (i32, i32)
{
    let mut pages = Pages::new();
    let (_rules, _pages) = inp.split_once("\n\n").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;

    for rule in _rules.lines() {
        let (f,b) = (twodigits_to_u32(rule), twodigits_to_u32(rule.get(3..).unwrap()));
        pages.add_rule(f, b);
    }

    for line in _pages.lines() {
        let pagelist:Vec<u32> = line.split(",").map(|x| twodigits_to_u32(x)).collect();
        let plen = pagelist.len();
        if pages.is_ordered(pagelist.clone()) {
            part1 += pagelist[plen>>1] as i32;
        }
        else {
            part2 += pages.order(pagelist);
        }

    }

    (part1, part2)
}

pub fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let fname: &str = if args.len() < 2 { "input.txt" } else { args[1].as_str() };
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(1000, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}