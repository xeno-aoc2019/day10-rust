extern crate num;
use num::integer;

pub fn group_surrounding_stars(station: (i32, i32), coords: Vec<(i32, i32)>) -> Vec<(i32, (i32, i32), (i32, i32), i32)> {
    let mut grouped = vec!();
    for star in coords {
        let dist = distance(station, star);
        if !(dist == (0, 0)) {
            let norm_dist = normalized_dist(station, star);
            let id = star.0 * 100 + star.1;
            // println!("normalized: ({},{})", norm_dist.0, norm_dist.1);
            grouped.push((sector(norm_dist), norm_dist, dist, id));
        }
    }
    grouped.sort();
    grouped
}

fn sector(star: (i32, i32)) -> i32 {
    if star.0 == 0 && star.1 == 0 {
        panic!("trying to find the sector of origo");
    }

    let x = star.0;
    let y = star.1;
    if x == 0 && y > 0 {
        return 1; // straight up - first "sector"
    }
    if y == 0 && x > 0 {
        return 4; // straight east - fourth "sector"
    }
    if x == 0 && y < 0 {
        return 7; // straight south - seventh "sector"
    }
    if y == 0 && x < 0 {
        return 10; // straight west - 10th "sector"
    }

    let ys = y / y.abs();
    let xs = x / x.abs();
    let xgty = (x.abs() - y.abs()) > 0;
    let sector = match (ys, xs, xgty) {
        (1, 1, false) => 2, // nne (northnortheast)
        (1, 1, true) => 3, // nee
        (-1, 1, true) => 5, // see
        (-1, 1, false) => 6, // sse
        (-1, -1, false) => 8, // ssw
        (-1, -1, true) => 9, // sww
        (1, -1, true) => 11,  // nww
        (1, -1, false) => 12, // nnw
        zot => {
            println!("({},{},{})", zot.0, zot.1, zot.2);
            panic!("Should not happen");
            // 5000
        }
    };
    return sector;
}

fn distance(station: (i32, i32), star: (i32, i32)) -> (i32, i32) {
    (star.0 - station.0, star.1 - station.1)
}

fn normalized_dist(station: (i32, i32), star: (i32, i32)) -> (i32, i32) {
    let dist = distance(station, star);
    let gcd = integer::gcd(dist.0, dist.1);
    if gcd == 0 {
        (0, 0)
    } else {
        (dist.0 / gcd, dist.1 / gcd)
    }
}