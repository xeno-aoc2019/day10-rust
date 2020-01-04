type Coordinate = (i32, i32);
type Id = i32;

pub fn find_sector_containing_nth(sectors: &Vec<Vec<(Id, Coordinate, Coordinate, Id)>>, nth: usize) -> (Vec<(Id, Coordinate, Coordinate, Id)>, usize) {
    let mut n = nth;
    for sect in sectors {
        if sect.len() < n {
            n = n - sect.len();
        } else {
            return (sect.clone(), n);
        }
    }
    panic!("Did not found the sector");
}

pub fn next_rotation_sectors(sorted_star_info: &Vec<(Id, Coordinate, Coordinate, Id)>) -> Vec<Vec<(Id, Coordinate, Coordinate, Id)>> {
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
                sector_stars = vec!();
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
