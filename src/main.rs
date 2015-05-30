#![feature(core)]

extern crate bitmap;
extern crate noise;
extern crate rand;
extern crate num;

use std::io;
use std::fs::File;

use bitmap::Bitmap;
use wrapping2darray::Wrapping2DArray;

mod math;
mod color;
mod wrapping2darray;
mod mapgen;

fn main() {
    const SIZE: i32 = 256*2;
    let test = mapgen::UpperMap::new(SIZE, SIZE);

    println!("Saving elevation map");
    elevation_bitmap(&test, "elevation.bmp").unwrap();

    println!("Saving flow map");
    flow_bitmap(&test, "flow.bmp").unwrap();
}

/// Produce an elevation bitmap
fn elevation_bitmap(map: &mapgen::UpperMap, filename: &str) -> io::Result<()> {
    let land_colors = [color::consts::BEAVER, color::consts::BUFF];
    let sea_colors = [color::consts::AZURE, color::consts::COOL_BLACK];

    let mut bitmap = Bitmap::new(map.elevation.width() as i32, map.elevation.height() as i32);

    for (idx, &h) in map.elevation.iter().enumerate() {
        let x = idx as i32 % bitmap.width();
        let y = idx as i32 / bitmap.width();

        // Map colors above (or equal to) 0.0 to land, and below 0.0 to sea
        if h >= 0.0 {
            bitmap.set_pixel(x, y, color::linear_gradient(&land_colors, h as f64).to_tuple());
        }
        else {
            bitmap.set_pixel(x, y, color::linear_gradient(&sea_colors, -h as f64).to_tuple());
        }
    }
    
    let mut file = try!(File::create(filename));
    bitmap.write(&mut file)
}
/// Produce a flow bitmap
fn flow_bitmap(map: &mapgen::UpperMap, filename: &str) -> io::Result<()> {
    let mut len_map = Wrapping2DArray::from_fn(map.ocean_flow.width(), map.ocean_flow.height(),
            |x, y| map.ocean_flow[(x, y)].length());
    mapgen::normalise(&mut len_map);

    let mut bitmap = Bitmap::new(map.ocean_flow.width() as i32, map.ocean_flow.height() as i32);

    let land_colors = [color::consts::BEAVER, color::consts::BUFF];
    let sea_colors = [color::consts::AZURE, color::consts::BLACK];

    for (idx, (flow, elev)) in len_map.iter().zip(map.elevation.iter()).enumerate() {
        let x = idx as i32 % bitmap.width();
        let y = idx as i32 / bitmap.width();

        if *elev >= 0.0 {
            bitmap.set_pixel(x, y, color::linear_gradient(&land_colors, *elev as f64).to_tuple());
        }
        else {
            bitmap.set_pixel(x, y, color::linear_gradient(&sea_colors, *flow as f64).to_tuple());
        }
    }

    let mut file = try!(File::create(filename));
    bitmap.write(&mut file)
}
