extern crate num;

use std::collections::HashSet;
use std::collections::HashMap;
// use std::iter::FromIterator;

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
    let mut grouped = grouping::group_surrounding_stars(station, coords);
    grouped.sort();
    let sectored = sectors::next_rotation_sectors(&grouped);
    let mut sorted_sectored = vec!();
    for scan_sector in sectored {
        if !scan_sector.is_empty() {
            let factors = find_x_factors(scan_sector.clone());
            let scale_map = make_scale_map(factors);
            let mut scaled_sector = scale(scan_sector.clone(), &scale_map);
            let sorted = sort_sector(&scaled_sector);
            sorted_sectored.push(sorted.clone());
            println!("SECTOR: ");
            for star in sorted {
                println!("  S: [{}] ({},{}) : {}", star.0, (star.1).0, (star.1).1, star.2);
            }
        }
    }
    println!("DONE, all sorted: ");
    let all = all_stars_from_sectors(&sorted_sectored);
    for star in all.clone() {
        println!("  #: [{}] ({},{}) : {}", star.0, (star.1).0, (star.1).1, star.2);
    }
    println!("result: {}, {}", all.get(199).unwrap().2, all.get(200).unwrap().2);
}

fn all_stars_from_sectors(sectors: &Vec<Vec<(Id, (i128, i128), Id)>>) -> Vec<(Id, (i128, i128), Id)> {
    let mut res = vec!();
    for sector in sectors {
        for star in sector {
            res.push(star.clone());
        }
    }
    res
}

fn sort_sector(sector: &Vec<(Id, (i128, i128), Id)>) -> Vec<(Id, (i128, i128), Id)> {
    if sector.is_empty() {
        return sector.clone();
    }
    let sector_id = sector.get(0).unwrap().0;
    println!("sector_id: {}", sector_id);
    let reverse = match sector_id {
        1 => false,
        2 => false,
        3 => false,
        4 => false,
        5 => true,
        6 => true,
        7 => true,
        8 => true,
        9 => true,
        10 => true,
        11 => false,
        12 => false,
        id => {
            println!("Unknown sector: {}", id);
            panic!("Unknown sector id")
        }
    };
    let mut sect = sector.clone();
    sect.sort();
    if reverse {
        sect.reverse();
    }
    sect
}

fn scale(vectors: Vec<(Id, Coordinate, Coordinate, Id)>, scales: &HashMap<i32, i128>) -> Vec<(Id, (i128, i128), Id)> {
    let mut res = vec!();
    for vector in vectors {
        let ax = ((vector.1).0).abs();
        if let Some(scale) = scales.get(&ax) {
            let ux = (vector.1).0 as i128 * scale.clone();
            let uy = (vector.1).1 as i128 * scale.clone();
            res.push((vector.0, (ux, uy), vector.3));
        } else {
            println!("Failed to find {} in scale map", &ax);
        }
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

fn mul(factors: HashSet<i32>) -> i128 {
    let mut val = 1;
    for z in factors {
        if z != 0 {
            val = val * z as i128;
        }
    }
    val
}

fn find_x_factors(vectors: Vec<(Id, Coordinate, Coordinate, Id)>) -> HashSet<i32> {
    let mut res = HashSet::new();
    for v in vectors {
        res.insert((v.1).0.abs());
    }
    res
}