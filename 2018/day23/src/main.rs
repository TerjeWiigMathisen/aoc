//use std::collections::HashMap;
//use std::io;
use std::cmp;
//use std::env;
use std::fs;
use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;

#[derive(Debug,Clone,Copy,Default)]
struct Point3 {
    x:i64,
    y:i64,
    z:i64,
}
impl Point3 {
    fn dist3(&self, p:&Point3) -> usize
    {
        return ((self.x-p.x).abs() + (self.y-p.y).abs() + (self.z-p.z).abs()) as usize;
    }
    fn _dist3xyz(&self, x:i64,y:i64,z:i64) -> usize
    {
        return ((self.x-x).abs() + (self.y-y).abs() + (self.z-z).abs()) as usize;
    }
    fn min_point3(&mut self, p:&Point3)
    {
        if p.x < self.x {self.x = p.x}
        if p.y < self.y {self.y = p.y}
        if p.z < self.z {self.z = p.z}
    }
    fn max_point3(&mut self, p:&Point3)
    {
        if p.x >= self.x {self.x = p.x}
        if p.y >= self.y {self.y = p.y}
        if p.z >= self.z {self.z = p.z}
    }
}
#[derive(Debug,Clone,Copy)]
struct Bot {
    p:Point3,
    r:usize,
}

#[derive(Debug,Clone)]
struct Cube {
    pmin:Point3,
    pmax:Point3,
    maxp:usize,
    nearest:usize,
    mindist_pos:Point3,
    botlist:Vec<usize>,
    always_power:usize,
    max_possible:usize,
}
fn nearest2(x:i64,x0:i64,x1:i64)->i64
{
    return if x0 <= x && x <= x1 {x}else if (x-x0).abs() < (x-x1).abs() {x0}else{x1};
}
fn furthest2(x:i64,x0:i64,x1:i64)->i64
{
    return if x*2 < x1+x0 {x1}else{x0};
}
impl Cube {
    fn nearest(&self,p:Point3) -> Point3
    {
        let p = Point3{x:nearest2(p.x,self.pmin.x,self.pmax.x),
                            y:nearest2(p.y,self.pmin.y,self.pmax.y),
                            z:nearest2(p.z,self.pmin.z,self.pmax.z)};
        return p;
    }
    fn furthest(&self,p:Point3) -> Point3
    {
        let p = Point3{x:furthest2(p.x,self.pmin.x,self.pmax.x),
                            y:furthest2(p.y,self.pmin.y,self.pmax.y),
                            z:furthest2(p.z,self.pmin.z,self.pmax.z)};
        return p;
    }
    fn init_cube(&mut self, bots:&Vec<Bot>, botlist:&Vec<usize>)
    {
//        let (x0,y0,z0) = (self.pmin.x,self.pmin.y,self.pmin.x);
//        let (x1,y1,z1) = (self.pmax.x,self.pmax.y,self.pmax.z);
        for b in botlist.iter() {
            let p = bots[*b].p;
            let r = bots[*b].r;
            let near = self.nearest(p);
            if near.dist3(&p) <= r { // This corner is in range, so include the bot!
                let far = self.furthest(p);
                if far.dist3(&p) <= r { // All corners in range!
                    self.always_power += 1;
                }
                else {
                    self.botlist.push(*b);
                }
            }
        }
        let mut all = 0;
        let mut lst = 0;
        for b in bots.iter() {
            let p = b.p;
            let r = b.r;
            let near = self.nearest(p);
            if near.dist3(&p) <= r { // This corner is in range, so include the bot!
                let far = self.furthest(p);
                if far.dist3(&p) <= r { // All corners in range!
                    all += 1;
                }
                else {
                    lst += 1;
                }
            }
        }
        assert!(all == self.always_power);
        assert!(lst == self.botlist.len());
        (self.nearest, self.mindist_pos) = self.nearest_origo();
        self.maxp = self.always_power + self.botlist.len();
        self.max_possible = combine_power_distance(self.always_power+self.botlist.len(), self.nearest);
    }

    fn nearest_origo(&self) -> (usize, Point3)
    {
//        let origo = ;
        let p = self.nearest(Point3{x:0,y:0,z:0});
        return ((p.x.abs()+p.y.abs()+p.z.abs()) as usize, p);
    }
}

#[derive(Debug,Default)]
struct Stats {
    find_max_count:usize,
    init_count:usize,
    best_position:Point3,
    best_score:usize,
    cube_skip:usize,
    cube_full:usize,
    cube_scan:usize,
    cube_queue:usize,
}

fn combine_power_distance(power:usize,distance:usize)->usize
{
    return if power == 0 {0}else{(power<<32)-distance};
}
fn split_power_distance(combined:usize)->(usize,usize)
{
    return if combined==0 {(0,0)}else{((combined>>32)+1,(1<<32)-(combined&0xffffffff))};
}

const SPLIT:i64 = 2;
const MINCUBE:i64 = 3;

fn find_max(bots:&Vec<Bot>, cube:&Cube, stats:&mut Stats)
{
    stats.find_max_count += 1;
//    if stats.find_max_count & 0xff == 0 {println!("{:?}", stats); }

    let origo = Point3::default();

    let maxdim = cmp::max(cmp::max(cmp::max(MINCUBE,
                            cube.pmax.x-cube.pmin.x+1),
                            cube.pmax.y-cube.pmin.y+1),
                            cube.pmax.z-cube.pmin.z+1);

//    println!("Size {} from {:?}",maxdim,cube.pmin);

    if maxdim > MINCUBE {
        let step = (maxdim + SPLIT-1)/SPLIT;
        let mut z = cube.pmin.z;
        let mut cubes:Vec<Cube> = vec![];
        while z <= cube.pmax.z {
            let z1 = cmp::min(z + step,cube.pmax.z);
            let mut y = cube.pmin.y;
            while y <= cube.pmax.y {
                let y1 = cmp::min(y + step,cube.pmax.y);
                let mut x = cube.pmin.x;
                while x <= cube.pmax.x {
                    let x1 = cmp::min(x + step,cube.pmax.x);
                    let mut c = Cube{pmin:Point3{x:x,y:y,z:z},pmax:Point3{x:x1,y:y1,z:z1},maxp:0,nearest:0,
                                mindist_pos:origo, botlist:vec![],always_power:cube.always_power,max_possible:0};
                    c.init_cube(&bots,&cube.botlist); // Will update maxp, botlist, nearest, max_possible
                    stats.init_count += 1;

                    if c.max_possible > stats.best_score {
                        if c.botlist.len() > 0 {
                            cubes.push(c);
                            stats.cube_queue += 1;
                        }
                        else {
                            stats.best_position = c.mindist_pos;
                            stats.best_score = c.max_possible;
//                            println!("New block (step={}) max {}, distance {} @ {:?}", step+1, c.maxp, c.nearest, c.mindist_pos );
                            stats.cube_full += 1;
//                            let check = bots.into_iter().fold(0,| sum, b| sum + (stats.best_position.dist3(&b.p) <= b.r) as usize);
//                            assert!(check == split_power_distance(stats.best_score).0);
                        }
                    }
                    else {
                        stats.cube_skip += 1;
                    }
                    x = x1+1;
                }
                y = y1+1;
            }
            z = z1+1;
        }
        cubes.sort_by(|a,b| (b.max_possible.cmp(&a.max_possible)));
        for c in cubes.iter() {
            stats.cube_queue -= 1;
            if c.max_possible <= stats.best_score {continue;} // Already found a better spot than anything possible here!

            find_max(bots, &c, stats);
//            println!("Got back {mpower} from dist{mdist} at pos {:?}",mpos);
        }
    }
    else { // Full search within this cube!
//        println!("Full search {:?} to {:?}",cube.pmin,cube.pmax);
        stats.cube_scan += 1;
        for z in cube.pmin.z..=cube.pmax.z {
            for y in cube.pmin.y..=cube.pmax.y {
                for x in cube.pmin.x..=cube.pmax.x {
                    if x == 12 && y == 12 && z == 12 {
//                        println!("Breakpoint");
                    }
                    let p = Point3{x,y,z};
                    let power = cube.botlist.iter().fold(cube.always_power, |sum,b| 
                            sum + (p.dist3(&bots[*b].p) <= bots[*b].r) as usize);
//                        if p.dist3(&bots[*b].p) <= bots[*b].r {
//                            power += 1;
//                        }
                    let d = origo.dist3(&p);
                    let pd = combine_power_distance(power, d);
                    if pd > stats.best_score {
                        stats.best_score = pd;
                        stats.best_position = p;
                        stats.best_score = pd;
//                        println!("New single max {power}, distance {} @ {:?}", split_power_distance(pd).1, p );
/*                        let check = bots.into_iter().fold(0,| sum, b| sum + (stats.best_position.dist3(&b.p) <= b.r) as usize);
                        if check != power {
                            println!("{:?}\n{:?}", stats, cube);
                        }
                        assert!(check == power);                    
*/
                    }
                }
            }
        }
    }
//    println!("Return power = {maxpower} from dist={mindist}");
}

fn process(inp:String) -> (i64,i64)
{
    let lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();
    let p = parser!("pos=<" x:i64 "," y:i64 "," z:i64 ">, r=" r:usize =>
        Bot {p:Point3{x,y,z}, r:r});
    let mut bots:Vec<Bot> = vec![];
    let mut pmin = Point3{x:i64::MAX, y:i64::MAX, z:i64::MAX};
    let mut pmax = Point3{x:i64::MIN, y:i64::MIN, z:i64::MIN};
    for li in lines {
//        println!("{li}");
        let b = p.parse(&li).unwrap();
        pmin.min_point3(&b.p);
        pmax.max_point3(&b.p);
        bots.push(b);
    }
//    println!("Volume from ({},{},{}) to ({},{},{})", pmin.x,pmin.y,pmin.z,pmax.x,pmax.y,pmax.z);
    bots.sort_by(|a, b| b.r.cmp(&a.r)); // Largest first!
    let sp = &bots[0].p;
    let radius = bots[0].r;

    let mut p1 = 0; // Count itself!
    for b in bots.iter() {
        if sp.dist3(&b.p) <= radius {
            p1 += 1;
        }
    }
    let mut stats:Stats = Stats::default();
    let cube = Cube{pmin:pmin,pmax:pmax,maxp:0,nearest:0,botlist:Vec::from_iter(0..bots.len()),always_power:0,max_possible:0,mindist_pos:Point3::default()};
    find_max(&bots, &cube, &mut stats);
    let (_pow,p2) = split_power_distance(stats.best_score);
//    let check = bots.into_iter().fold(0,| sum, b| sum + (stats.best_position.dist3(&b.p) <= b.r) as usize);
//    assert!(check == pow);
    println!("Statistics: {:?}",stats);
//    println!("Returning {p1}, {p2}");
    return (p1, p2 as i64);
}


fn main() {
    assert!(process(
"pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5".to_owned()) == (6,36));

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(4, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}