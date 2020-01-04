extern crate num;

use num::integer;
use std::collections::HashSet;
use std::iter::FromIterator;


pub fn find_station(coords: &Vec<(i32,i32)>) -> ((i32,i32),u32){
    let mut max_stars = 0;
    let mut station = (0,0);
    for c in coords {
        let directions = dirs_to_others(&c, &coords);
        let unique_dirs = unique(&directions);
        let star_count=  unique_dirs.len();
        if star_count > max_stars {
            max_stars = star_count;
            station = c.clone();
        }
    }
    (station, max_stars as u32)
}

fn dirs_to_others(c: &(i32, i32), others: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut vects = vec!();
    for coord in others {
        let dx = coord.0 - c.0;
        let dy = coord.1 - c.1;
        let div = integer::gcd(dx, dy);
        if div != 0 {
            let ux = dx / div;
            let uy = dy / div;
            vects.push((ux, uy));
        }
    }
    vects
}

fn unique(dirs: &Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
    HashSet::from_iter(dirs.iter().cloned())
}

