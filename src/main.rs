#![allow(dead_code, unused_variables)]

use std::cmp;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Costs {
    offset: i32,
    cost: i32,
}

fn max_interval(a: &[i32]) -> i32 {
    let mut maxint: i32 = 0;
    let mut i: usize = 0;
    let mut j: usize = 1;
    while j < a.len() {
        maxint = cmp::max(maxint, a[j] - a[i]);
        i += 1;
        j += 1;
    }
    return maxint;
}

fn total_diff(a: &[i32], offset: i32, interval: i32) -> i32 {
    return offset^2;
}

fn min_cost_displace<'a>(a: &[i32], costs_vec: &'a mut Vec<Costs>, maxint: i32, interval: i32) -> Option<&'a Costs> {
    for offset in -maxint..=maxint {
        let cost = total_diff(&a, offset, interval);
        let c = Costs {
            offset: offset,
            cost: cost,
        };
        costs_vec.push(c);
    }
    return costs_vec.iter().max_by_key(|x| x.cost);
}

fn main() {
    let a = &[0, 2, 3, 4, 8, 12, 20];
    let maxint = max_interval(a);
    println!("maxint: {:#?}", maxint);

    let mut costs_vec: Vec<Costs> = Vec::new();
    match min_cost_displace(a, &mut costs_vec, maxint, 2) {
        Some(c) => println!("c: {:#?}", c),
        None => println!("not found"),
    }
}
