extern crate num;

mod input;
mod station;

fn main() {
    let coords = input::read_points("input.txt");
    let (station, stars_in_sight) = station::find_station(&coords);
    println!("Station is at ({},{}) with line of sight to {} other stars", station.0, station.1, stars_in_sight);
}


