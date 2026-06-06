// Fastest run 10.738 ms binary
// Fastest run 11.986 ms linear scan
//use std::collections::HashMap;
use std::collections::VecDeque;
//use std::io;
//use std::env;
//use std::str;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

struct Cell {
    c:u8,
    hit:u8,
}

struct Dist {
    pos:u16,
    dist:u16,
}

const LIVE_BIT:i64 = 0;
const EG_BIT:i64 = 1;
const MOVED_BIT:i64 = 2;
const KILLED_BIT:i64 = 3;


struct Lgrid {
    y_step:usize,
    grid:Vec<Cell>,
}
impl Lgrid {
    fn lt(self:&Lgrid, pos:usize)->usize
    {
        return pos -1;
    }
    fn rt(self:&Lgrid, pos:usize)->usize
    {
        return pos +1;
    }
    fn up(self:&Lgrid, pos:usize)->usize
    {
        return pos - self.y_step;
    }
    fn dn(self:&Lgrid, pos:usize)->usize
    {
        return pos + self.y_step;
    }
    fn move_dir(self:&Lgrid, pos:usize, dir:usize)->usize
    {
        let dx:Vec<i32> = vec![0,-1,1,0];
        let dy:Vec<i32> = vec![-1,0,0,1];
        
        return (pos as i32 + dx[dir] + (dy[dir]*self.y_step as i32)) as usize;
    }

    fn disp(self:&Lgrid)
    {
        let mut line = self.y_step;
        let mut points:String = "".to_owned();
        for i in 1..=self.grid.len() {
            let c = self.grid[i-1].c;
            print!("{}",c as char);
            if c | 2 == 'G' as u8 {
                let h = self.grid[i-1].hit;
                points = format!("{points} {}({h})", c as char).to_owned();
            }
            if i >= line {
                println!("{points}");
                line += self.y_step;
                points = "".to_owned();
            }
        }
        println!("");
    }

    fn do_round(self:&mut Lgrid, pos:usize, elf_power:u8)->usize
    {
        let target = self.grid[pos].c ^ 2;   // Flip 'E' <-> 'G'
        let mut target_at_distance = u16::MAX;
        let mut targets:Vec<usize> = vec![];
        let mut dist:Vec<u16> = vec!(u16::MAX;self.grid.len());
        dist[pos] = 0;
        let mut moves:VecDeque<Dist> = VecDeque::new();
        moves.push_back(Dist{pos:self.up(pos) as u16,dist:0 as u16});
        moves.push_back(Dist{pos:self.lt(pos) as u16,dist:1 as u16});
        moves.push_back(Dist{pos:self.rt(pos) as u16,dist:2 as u16});
        moves.push_back(Dist{pos:self.dn(pos) as u16,dist:3 as u16});
        while moves.len() > 0 {
            let d = moves.pop_front().expect("front pop error!");
            if (d.dist >> 2) > (target_at_distance >> 2) {break}

            let idx = d.pos as usize;
            let g = &self.grid[idx];
            if g.c == '.' as u8 { // Free location to move into!
                if dist[idx] > d.dist {
                    dist[idx] = d.dist;
                    let next_dist = d.dist+4;
                    moves.push_back(Dist{pos:self.up(idx) as u16,dist:next_dist});
                    moves.push_back(Dist{pos:self.lt(idx) as u16,dist:next_dist});
                    moves.push_back(Dist{pos:self.rt(idx) as u16,dist:next_dist});
                    moves.push_back(Dist{pos:self.dn(idx) as u16,dist:next_dist});
                }
                // Else discard this path, we have reached it before!
            }
            else if g.c == target {
                targets.push(idx * 4 + (d.dist & 3) as usize);
                if d.dist < target_at_distance {
                    target_at_distance = d.dist; // Nearest target found!
                }
            }
        }
        if targets.len() == 0 {
//            println!(" -> Blocked!");
            return 0;
        }
        let mut ret = 1 << LIVE_BIT; // We found a live target
        if target_at_distance >= 4 { // Move closer!
            //Make a move in the best (reading order) direction:
            let newpos = self.move_dir(pos, (target_at_distance & 3) as usize);
            self.grid[newpos].c = self.grid[pos].c;
            self.grid[newpos].hit = self.grid[pos].hit;
            self.grid[pos].c = '.' as u8;
//            println!(" --> to ({},{}) ({})",newpos % self.y_step, newpos / self.y_step, newpos);
            target_at_distance -= 4;
            ret |= 1 << MOVED_BIT;
        }
        if target_at_distance < 4 {
            // Pick the best target
            let mut minpos = targets[0];
            let mut minhit = self.grid[minpos >> 2].hit as usize;
            for i in 1..targets.len() {
                let p = targets[i];
                let h = self.grid[p>>2].hit as usize;
                if (h << 16) + p < (minhit << 16) + minpos {
                    minpos = p;
                    minhit = h;
                }
            }
            // Do the attack:
            minpos >>= 2;
            let hit_power = if self.grid[minpos].c == 'G' as u8 {elf_power} else {3};
            if minhit <= hit_power as usize { // Kill this player!
//                println!("Killed {}({},{}) ({})", self.grid[minpos].c as char,
//                        minpos % self.y_step, minpos / self.y_step, minpos);
                ret |= (1 << KILLED_BIT) + (self.grid[minpos].c & 2) as usize;
                self.grid[minpos].c = '.' as u8;
            }
            else {
                self.grid[minpos].hit -= hit_power;
//                println!("Attacked ({},{}) ({})", minpos % self.y_step, minpos / self.y_step, minpos);
            }
        }
        return ret;
    }
}

fn process(inp:&str) -> (i64,i64)
{
    let bytes = inp.as_bytes().to_owned();
    let mut idx:usize = 0;
    let mut lg:Lgrid = Lgrid{y_step:usize::MAX,grid:vec![]};
    let mut counts:Vec<usize> = vec![0,0];
    for b in bytes {
        if b >= ' ' as u8 {
            lg.grid.push(Cell{c:b,hit:200});
            if b | 2 == 'G' as u8 {
                counts[((b>>1)&1) as usize] += 1;
            }
        }
        else { // newline
            if idx < lg.y_step {
                lg.y_step = idx;
            }
        }
        idx += 1;
    }
//    lg.disp();
    let mut full_rounds:i64 = 0;
    loop {
        let mut to_move:Vec<usize> = vec![];
        for idx in 0..lg.grid.len() {
            if lg.grid[idx].c | 2 == 'G' as u8 { 
                to_move.push(idx);
            }
        }
        let mut moves = 0;
        for (e,m) in to_move.iter().enumerate() {
//            print!("Moving {} @ ({},{} (pos={})",lg.grid[*m].c as char,
//                m % lg.y_step, m / lg.y_step, m);
            let mut r = lg.do_round(*m,3);
            moves += r;
            if r & (1<<KILLED_BIT) != 0 {
                r = (r >> EG_BIT) & 1;
                counts[r] -= 1;
                if counts[r] == 0 {
                    let mut total_hit_points:i64 = 0;
                    for idx in 0..lg.grid.len() {
                        if lg.grid[idx].c | 2 == 'G' as u8 {
                            let h = lg.grid[idx].hit as i64;
                            total_hit_points += h;
                        }
                    }
                    if e+1 == to_move.len() {
                        full_rounds += 1;
                    }
                    let result = full_rounds * total_hit_points;
                    return (result as i64, r as i64);
                }
            }

        }
        full_rounds += 1;
//        lg.disp();
        if moves == 0 {break}
    }
    return (0,0);
}

fn process2(inp:&str) -> (i64,i64)
{
    let mut low = 3;
    let mut high = 200;
    let mut min_power = u8::MAX;
    let mut min_result = i64::MAX;
    'outer: // for hit_power in 4..255 {
    loop {
        let hit_power = low + (high-low) / 2;
        let bytes = inp.as_bytes().to_owned();
        let mut idx:usize = 0;
        let mut lg:Lgrid = Lgrid{y_step:usize::MAX,grid:vec![]};
        let mut counts:Vec<usize> = vec![0,0];
        for b in bytes {
            if b >= ' ' as u8 {
                lg.grid.push(Cell{c:b,hit:200});
                if b | 2 == 'G' as u8 {
                    counts[((b>>1)&1) as usize] += 1;
                }
            }
            else { // newline
                if idx < lg.y_step {
                    lg.y_step = idx;
                    let _ys = idx;
                }
            }
            idx += 1;
        }
//        lg.disp();
        let mut full_rounds:i64 = 0;
        loop {
            let mut to_move:Vec<usize> = vec![];
            for idx in 0..lg.grid.len() {
                if lg.grid[idx].c | 2 == 'G' as u8 { 
                    to_move.push(idx);
                }
            }
            let mut moves = 0;
            for (e,m) in to_move.iter().enumerate() {
 //               print!("Moving {} @ ({},{} (pos={})",lg.grid[*m].c as char,
 //                   m % lg.y_step, m / lg.y_step, m);
                let mut r = lg.do_round(*m,hit_power);
                moves += r;
                if r & (1<<KILLED_BIT) != 0 {
                    r = (r >> EG_BIT) & 1;
                    if r == 0 {
                        //println!("Tried range {low} - {hit_power} - {high}, Elf died!");
                        low = hit_power;
                        if low+1 == min_power {
                            return (min_result as i64, 0);
                        }
                        continue 'outer; // An Elf died, that is NOT OK, so inc hit_points and try again!
                    }
                    counts[r] -= 1;
                    if counts[r] == 0 {
                        if hit_power < min_power {
                            min_power = hit_power;
                            let mut total_hit_points:i64 = 0;
                            for idx in 0..lg.grid.len() {
                                if lg.grid[idx].c | 2 == 'G' as u8 {
                                    let h = lg.grid[idx].hit as i64;
                                    total_hit_points += h;
                                }
                            }
                            if e+1 == to_move.len() {
                                full_rounds += 1;
                            }
                            let result = full_rounds * total_hit_points;
                            //println!("No Elfs die with hit power = {hit_power}");
                            if low+1 == hit_power {
                                return (result as i64, 0);
                            }
                            min_result = result;
                        }
                        high = hit_power;
                        continue 'outer;
                    }
                }

            }
            full_rounds += 1;
//            lg.disp();
            if moves == 0 {break}
        }
    }
    return (0,0);
}


fn main() {
    assert!(process(
"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######").0 == 27730);

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(100, |_| { 
        process(&input); 
        process2(&input);
    }); bench_result.print_stats();

    devtime.start();
    let (part1,_p1) = process(&input);
    let (part2,_p2) = process2(&input);
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}