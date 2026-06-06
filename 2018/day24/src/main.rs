//use std::collections::HashMap;
//use std::io;
//use std::env;
use std::fs;
//use aoc_parse::{parser, prelude::*};
use devtimer::DevTime;
use devtimer::run_benchmark;
//use substring::Substring;
use regex::Regex;

const FIRE:u32 = 1;
const SLASH:u32 = 2;
const BLUDGEON:u32 = 4;
const COLD:u32 = 8;
const RADIATION:u32 = 16;

fn attack_to_mask(at:&str)->u32
{
    match at {
        "fire" => return FIRE,
        "slashing" => return SLASH,
        "bludgeoning" => return BLUDGEON,
        "cold" => return COLD,
        "radiation" => return RADIATION,
        _ => {println!("{at} "); panic!("Bad attack type")},
    }
}

#[derive(Debug,Clone)]
struct Army {
    team:usize,
    units:usize,
    hitpoints:usize,
    weakness_bitmap:u32,
    immunity_bitmap:u32,
    attack_damage:usize,
    attack_mask:u32,
    initiative:usize,
    will_attack:i32,
    attacked_by:i32,
}
impl Army {
    fn target_attack_damage(&self, b:&Army)->usize
    {
        if self.team == b.team {return 0};
        let damage = self.effective_power();
        if b.weakness_bitmap & self.attack_mask != 0 {return damage*2};
        if b.immunity_bitmap & self.attack_mask != 0 {return 0};
        return damage;
    }
    fn effective_power(&self)->usize
    {
        return self.units*self.attack_damage;
    }
    fn target_selection_order(&self)->usize
    {
        if self.units == 0 {return 0};
        if self.will_attack >= 0 {return 0};
        return self.effective_power()*100 + self.initiative;
    }
}

fn process_boost(input_armies:&Vec<Army>, immune_boost:usize) -> (usize,usize)
{
    let mut armies:Vec<Army> = vec![];
    for a in input_armies.iter() { 
        let mut b = a.clone();
        if b.team == 0 {
            b.attack_damage += immune_boost;
        }
        armies.push(b);
    }
    loop {
        let mut filtered_armies:Vec<Army>=vec![];
        let mut attack_pr_team = [0,0];
        for a in armies.iter() { 
            if a.units > 0 {
                let mut fa = a.clone();
                fa.will_attack = -1; 
                fa.attacked_by = -1;
                filtered_armies.push(fa);
                attack_pr_team[a.team] += 1;
            }
        }
        armies = filtered_armies;

//        for a in armies.iter() {println!("{:?}",a); }

        if attack_pr_team[0]*attack_pr_team[1] == 0 {break;} // Exit when one team is dead!

        let mut selection_order:Vec<usize> = Vec::from_iter(0..armies.len());
        selection_order.sort_by(|a,b| armies[*b].target_selection_order().
                cmp(&(armies[*a].target_selection_order())));
        for i in 0..selection_order.len() {
            let idx = selection_order[i];
            let a = armies[idx].clone();
            let mut max_combined_target:usize = 0;
            let mut max_index = 0;
            for b in 0..armies.len() {
                if armies[b].attacked_by >= 0 {continue};

                let damage = a.target_attack_damage(&armies[b]);
                if damage == 0 {continue};
                let effective_power = armies[b].effective_power();
                if effective_power == 0 {continue};

                let initiative = armies[b].initiative;
                let combined_target = (damage << 32) + (effective_power << 8) + initiative;
                if combined_target > max_combined_target {
                    max_combined_target = combined_target;
                    max_index = b;
                }
            }
            if max_combined_target > 0 { // Did we find someone we can attack?
                armies[max_index].attacked_by = i as i32;
                armies[idx].will_attack = max_index as i32;
            }
        }

//        println!("{:?}", selection_order);
        // Do the attack
        let mut attack_order:Vec<usize> = Vec::from_iter(0..armies.len());
        attack_order.sort_by(|a, b| armies[*b].initiative.cmp(&armies[*a].initiative));
//        println!("{:?}", attack_order);
        let mut sum_damage = 0;
//        print!("Killed ");
        for i in 0..attack_order.len() {
            let a = armies[attack_order[i]].clone();
            let j = a.will_attack;
            if j < 0 {continue};

//            assert!(j as usize != i);
            let b = &mut armies[j as usize];
            let damage = a.target_attack_damage(&b);
            let units_killed = damage/b.hitpoints;
            sum_damage += units_killed;
//            print!(" {units_killed}, ");
            b.units = if units_killed < b.units {b.units-units_killed}else{0};
        }
//        println!("");
        if sum_damage == 0 {return (0,1)} // Nobody can kill anyone!
    }
    let p1 = armies.iter().fold(0, |p1,a| p1 + a.units);
    return (p1 ,armies[0].team);
}

fn process(inp:String) -> (i64,i64)
{
    let mut armies:Vec<Army> = vec![];
    let lines:Vec<String> = inp.split("\n").map(|s| s.to_string()).collect();

    let mut group =0; // Immune system
   //    r"^(\d+) units each with (\d+) hit points (\(immune to slashing; weak to bludgeoning, radiation) with an attack that does 9 slashing damage at initiative 18
    let army_re = Regex::new(
        r"^(\d+) units each with (\d+) hit points (.*with) an attack that does (\d+) (\S+) damage at initiative (\d+)$").unwrap();
    let immune_re = Regex::new(r"immune to ([^;\)]+)[;\)]").unwrap();
    let weak_re = Regex::new(r"weak to ([^;\)]+)[;\)]").unwrap();
    for l in lines {
        if l == String::from("Immune System:") {
            group = 0;
        }
        else if l == String::from("Infection:") {
            group = 1; // Infections
        }
        else if l.len() > 0 {
            for cap in army_re.captures_iter(&l) {
                let (units_str, hitpoints_str, immune_weak, damage_str, damage_type_str, initiative_str) = 
                   (cap[1].to_owned(),cap[2].to_owned(),cap[3].to_owned(),cap[4].to_owned(),cap[5].to_owned(),cap[6].to_owned());
                let mut weak = 0;
                let mut immune = 0;
                if immune_weak.len() > 0 {
                    for im in immune_re.captures_iter(&immune_weak) {
                        let immune_str = im[1].to_owned();
                        for i in immune_str.split(", ") {
                            immune |= attack_to_mask(i);
                        }
                    }
                    for im in weak_re.captures_iter(&immune_weak) {
                        let weak_str = im[1].to_owned();
                        for w in weak_str.split(", ") {
                            weak |= attack_to_mask(w);
                        }
                    }
                }
                let atype = attack_to_mask(&damage_type_str);
                let army = Army{team:group, units:units_str.parse::<usize>().unwrap(),hitpoints:hitpoints_str.parse::<usize>().unwrap(),immunity_bitmap:immune,weakness_bitmap:weak,
                    attack_damage:damage_str.parse::<usize>().unwrap(),attack_mask:atype,initiative:initiative_str.parse::<usize>().unwrap(),will_attack:-1,attacked_by:-1};
//                println!("{:?}", army);
                armies.push(army);
            }
        }
    }
    let (p1,_team) = process_boost(&armies,0);
//    let (_p1,_team) = process_boost(&armies,1570);
    let mut p2;
    let mut team;
    let mut boost = 5000; 
    let mut low = 0;
    loop {
        (p2,team) = process_boost(&armies, boost);
//        println!("low={low}, boost={boost}, team={team}");
        if team == 0 {break;}
        low = boost; boost += boost;
    }
    while boost > low {
        let mid = (boost+low)/2;
        let (units,team) = process_boost(&armies, mid);
//        println!("low={low}, mid={mid}, boost={boost} p2={p2}, units={units}, team={team}");
        if team == 1 {low=mid+1}else{boost=mid; p2 = units}
    }
    return (p1 as i64, p2 as i64)
}

fn main() {

    assert!(process(
"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"
.to_owned()) == (5216,51));

    let fname = "input.txt"; // instead of args[1]
    let mut input = fs::read_to_string(fname).expect("Error readin input file");
    if input.as_bytes()[input.as_bytes().len()-1] == '\n' as u8 {input.pop();}

    let mut devtime = DevTime::new_simple();

    let bench_result = run_benchmark(10, |_| { process(input.clone()); }); bench_result.print_stats();

    devtime.start();
    let (part1, part2) = process(input.clone());
    devtime.stop();

    println!("Part1 = {part1}");
    println!("Part2 = {part2}");
    println!("Total time {} us",devtime.time_in_micros().unwrap());
}