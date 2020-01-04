extern crate num;

use num::integer;

mod input;
mod station;

fn main() {
    let coords = input::read_points("input.txt");
    let (station, stars_in_sight) = station::find_station(&coords);
    println!("Station is at ({},{}) with line of sight to {} other stars", station.0, station.1, stars_in_sight);
    group(station, coords);
}

fn group(station: (i32, i32), coords: Vec<(i32, i32)>) -> () {
    let mut grouped = vec!();
    for star in coords {
        let dist = distance(station, star);
        let norm_dist = normalized_dist(station, star);
        let id = star.0 * 100 + star.1;
        // println!("normalized: ({},{})", norm_dist.0, norm_dist.1);
        grouped.push((norm_dist, dist, id));
    }
    grouped.sort();
    for g in grouped {
        println!("normalized: ({},{}) [({},{}], {}", (g.0).0, (g.0).1, (g.1).0, (g.1).1, g.2);
    }
}

fn distance(station: (i32, i32), star: (i32, i32)) -> (i32,i32) {
    (star.0 - station.0, star.1 - station.1)
}

fn normalized_dist(station: (i32, i32), star: (i32, i32)) -> (i32, i32) {
    let dist = distance(station, star);
    let gcd = integer::gcd(dist.0, dist.1);
    if gcd == 0 {
        (0,0)
    } else {
        (dist.0 / gcd, dist.1 / gcd)
    }
}