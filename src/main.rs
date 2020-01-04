extern crate num;

// use num::integer;

mod input;
mod station;
mod grouping;

type Coordinate = (i32,i32);
type Id = i32;

fn main() {
    let coords = input::read_points("input.txt");
    let (station, stars_in_sight) = station::find_station(&coords);
    println!("Station is at ({},{}) with line of sight to {} other stars", station.0, station.1, stars_in_sight);
    let grouped = grouping::group_surrounding_stars(station, coords);
    for g in grouped.clone() {
        // println!("normalized: [{}] ({},{}) [({},{}], {}", g.0, (g.1).0, (g.1).1, (g.2).0, (g.2).1, g.3);
    }
    let sectors = next_rotation_sectors(&grouped);
    println!("sector_passes = {}", sectors.len());
    for sect in sectors {
        println!("*** NEXT PASS: ");
        for g in sect {
            println!("sect member: [{}] ({},{}) [({},{}], {}", g.0, (g.1).0, (g.1).1, (g.2).0, (g.2).1, g.3);
        }
    }
}

fn next_rotation_sectors(sorted_star_info: &Vec<(Id,Coordinate,Coordinate,Id)>) -> Vec<Vec<(Id,Coordinate,Coordinate,Id)>> {
    let mut next_rotations = vec!();
    let mut sectors = vec!();
    let mut last_norm_vect = (0, 0);
    let mut current_sector = 1;
    let mut sector_stars = vec!();
    for star_info in sorted_star_info {
        if star_info.1 == last_norm_vect {
            // println!("    pushing to next rotation: [{}] ({},{})", star_info.0, (star_info.1).0, (star_info.1).1);
            next_rotations.push(star_info.clone());
        } else {
            if current_sector != star_info.0 {
               // println!("  new sector for current sector: [{}] ({},{})", star_info.0, (star_info.1).0, (star_info.1).1);
                sectors.push(sector_stars);
                sector_stars=vec!();
                current_sector = star_info.0;
            }
            // println!("adding to current sector: [{}] ({},{})", star_info.0, (star_info.1).0, (star_info.1).1);
            sector_stars.push(star_info.clone());
            last_norm_vect = star_info.1
        }
    }
    sectors.push(sector_stars);
    if next_rotations.len() > 0 {
        for sector in next_rotation_sectors(&next_rotations) {
            sectors.push(sector);
        }
    }
    sectors
}
