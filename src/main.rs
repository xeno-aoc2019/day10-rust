extern crate num;

use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

// use num::integer;

mod input;
mod station;
mod grouping;
mod sectors;

type Coordinate = (i32, i32);
type Id = i32;

fn main() {
    let coords = input::read_points("input.txt");
    let (station, stars_in_sight) = station::find_station(&coords);
    println!("Station is at ({},{}) with line of sight to {} other stars", station.0, station.1, stars_in_sight);
    let grouped = grouping::group_surrounding_stars(station, coords);
    let sectors = sectors::next_rotation_sectors(&grouped);
    println!("sector_passes = {}", sectors.len());
    let (sect, n) = sectors::find_sector_containing_nth(&sectors, 200);
    println!("Found a sector containing planet 200, index in sector = {}", n);
    for g in sect.clone() {
        println!("sect member: [{}] ({},{}) [({},{}], {}", g.0, (g.1).0, (g.1).1, (g.2).0, (g.2).1, g.3);
    }
    let xs: HashSet<i32> = HashSet::from_iter(sect.clone().iter().map(|star| ((star.1).0).abs()));
    let _ys: HashSet<i32> = HashSet::from_iter(sect.clone().iter().map(|star| ((star.1).1).abs()));

    let scale_map = make_scale_map(xs);

    let mut scaled = scale(sect, &scale_map);

    scaled.sort();
    // scaled.reverse(); // only valid for positive x'es, it's scaled on x, drops along y.

    for x in scaled.clone() {
        println!("SCALE: ({},{}) -> {}", (x.0).0, (x.0).1, x.1);
    }

    let n1 = scaled.get(n - 1).unwrap();
    let n2 = scaled.get(n).unwrap();
    let n3 = scaled.get(n + 1).unwrap();

    println!("n1(correct)={}, n2={}, n3={}", n1.1, n2.1, n3.1);
}

fn scale(vectors: Vec<(Id, Coordinate, Coordinate, Id)>, scales: &HashMap<i32, i128>) -> Vec<((i128, i128), Id)> {
    let mut res = vec!();
    for vector in vectors {
        let ax = ((vector.1).0).abs();
        let scale = scales.get(&ax).unwrap();
        res.push((((vector.1).0 as i128 * scale.clone(), (vector.1).1 as i128 * scale.clone().clone()), vector.3));
    }
    res
}

fn make_scale_map(factors: HashSet<i32>) -> HashMap<i32, i128> {
    let mut map = HashMap::new();
    for i in factors.clone() {
        let mut scale = 1i128;
        for z in factors.clone() {
            if z != 0 && z != i {
                scale = scale * (z as i128);
            }
            map.insert(i, scale);
        }
    }
    map
}