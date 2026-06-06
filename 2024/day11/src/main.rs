// 844 us

//use hashbrown::DefaultHashBuilder;
use std::collections::HashMap;
use std::hash::RandomState;
//use std::io;
//use std::env;
use std::fs;
use devtimer::DevTime;
use devtimer::run_benchmark;

struct Stone {
    stonevalue:u64,
    left:u32,
    right:u32,
}

struct StoneList {
    stones:Vec<Stone>,
    stone_cnt:Vec<u64>,
    stone_index:HashMap<u64, u32>,
}

impl StoneList {
    fn new() -> StoneList {
        StoneList { stones:Vec::with_capacity(5000), stone_cnt:Vec::with_capacity(5000), stone_index:HashMap::with_capacity(5000) }
    }

//    #[inline(always)]
    fn get_idx(&mut self, value:u64) -> u32 {
        let pidx = self.stone_index.get(&value);
        if pidx.is_some() {
            let idx = *pidx.unwrap();
            return idx
        }

        let idx = self.stones.len() as u32;
        self.stones.push(Stone{stonevalue:value,left:0,right:0});
        self.stone_cnt.push(0);
        self.stone_index.insert(value, idx);
        assert!(self.stones.len() == self.stone_cnt.len() && self.stones.len() == self.stone_index.len());
        idx
    }

    fn update(&mut self, ix:usize ) -> (u64,u32,u32){
        let cnt = self.stone_cnt[ix];
        let mut left = self.stones[ix].left;
        let mut right = self.stones[ix].right;
        if left == 0 {
            let value = self.stones[ix].stonevalue;
            let stonestr = format!("{}", value);
            let l = stonestr.len();
            if l & 1 == 0 {
                let half = l>>1;
                let left_half = &stonestr[0..half];
                let right_half = &stonestr[half..];
                left = left_half.parse::<u32>().unwrap();
                right = right_half.parse::<u32>().unwrap();
                left = self.get_idx(left as u64);
                assert!(left > 0);
                right = self.get_idx(right as u64);
                assert!(right > 0);
                self.stones[ix].left = left;
                self.stones[ix].right = right;
            } else if value == 0 {
                left = self.get_idx(1);
                assert!(left > 0);
                self.stones[ix].left = left;
                assert!(self.stones[ix].right == 0);
            } else {
                left = self.get_idx(value*2024);
                assert!(left > 0);
                self.stones[ix].left = left;
                assert!(self.stones[ix].right == 0);
            }
        }
        (cnt,left,right)
    }
}

fn process(inp:String) -> (u64,u64) 
{
    let nums = inp.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut sl = StoneList::new();
    sl.get_idx(u64::MAX); // Fill up the zero slot!
    for n in nums.iter() {
        let idx = sl.get_idx(*n);
        sl.stone_cnt[idx as usize] = 1;
    }
    let mut part1 = 0;
    let mut part2 = 0;
    for gen in 1..=75 {
//        println!("stone_cnt: {:?}", sl.stone_cnt);
        let slen = sl.stone_cnt.len();
        assert!(slen == sl.stones.len());
        assert!(slen == sl.stone_index.len());
        let mut newcnt = vec![0;slen];
        //let mut _alive = 0;
        //let mut _growth = 0;
        for idx in 1..slen {
            if sl.stone_cnt[idx] == 0 { continue;}
            //_alive += 1;

            let (cnt, left, right) = sl.update(idx);
//            println!("update idx {idx}, value={}, cnt={cnt}, left={left}, right={right}",sl.stones[idx].stonevalue);
            while newcnt.len() < sl.stone_cnt.len() { newcnt.push(0);}
            assert!(left > 0);
            newcnt[left as usize] += cnt;
            // if right > 0 { newcnt[right as usize] += cnt; }
            newcnt[right as usize] += cnt;
            
        }
        
//        println!("Gen {gen}: {:?}\nslen={}, alive={alive}, total={part2}, growth={growth}", newcnt, sl.stone_cnt.len());
        if gen == 25 || gen == 75 {
            part2 = 0;
            //let mut alive = 0;
            //let mut max = 0;
            for idx in 1..newcnt.len() {
                let n = newcnt[idx]; 
                //if n == 0 { continue;}
                part2 += n;
                //alive += 1;
                //if n > max { max = n;}
            }
            //println!("Gen {gen}: len={}, alive={alive}, total={part2}, max={max}", sl.stone_cnt.len());
            if gen == 25 {
                part1 = part2;
            }
        }
        
        sl.stone_cnt = newcnt;
    }
    //println!("Final stone count = {}", sl.stones.len());
    (part1, part2)
}

fn main() {
    let fname = "input.txt"; // instead of args[1]
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