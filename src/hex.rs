use std::vec::Vec;
use std::ops::Rem;
use ggez::graphics::Color;
use cgmath::InnerSpace;
use num::Float;

use crate::Vec2f;
use crate::Vec3f;
use crate::num_utils::vec2_modulo;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HexagonTile {
    Land,
    Water,
    Clicked,
}
impl Default for HexagonTile {
    fn default() -> Self {
        HexagonTile::Water
    }
}

#[derive(Default)]
pub struct HexagonGrid {
    pub hexagons: Vec<HexagonTile>,
    pub width: usize,
    pub height: usize,
    pub seed: i32,
}
impl HexagonGrid {
    pub fn new(w: usize, h: usize, seed: Option<i32>) -> Self {
        let mut hexagons = Vec::new();
        hexagons.resize(w*h, HexagonTile::Water);

        HexagonGrid {
            hexagons: hexagons,
            width: w,
            height: h,
            seed: seed.unwrap_or(0),
        }
    }

    pub fn get_rgba8(&self) -> Vec<u8> {
        let mut colors = Vec::new();
        colors.resize_with(self.height*self.width*4, Default::default);

        for i in 0..self.width*self.height {
            let index = i*4;

            let color = if self.hexagons[i] == HexagonTile::Land { 
                    Color::new(0.0, 1.0, 0.0, 1.0) 
                } else if self.hexagons[i] == HexagonTile::Water { 
                    Color::new(0.0, 0.0, 1.0, 1.0)
                } else {
                    Color::new(1.0, 0.0, 0.0, 1.0)
                };
            let (r, g, b, a) = color.to_rgba();
            colors[index + 0] = r;
            colors[index + 1] = g;
            colors[index + 2] = b;
            colors[index + 3] = a;
        }

        colors
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{:?}", self.hexagons[y * self.width + x] as u8);
            }
            println!("");
        }
    }
}

// pub fn hex_coord(uv: Vec2f) -> Vec2f {
// 	let r = Vec2f::new(1.0, 1.73);
//     let h = r*0.5;
    
//     let a = vec2_modulo(uv, r) - h;
//     let b = vec2_modulo(uv-h, r)-h;
    
//     let gv = if a.magnitude2() < b.magnitude2() { a } else { b };

//     let id = uv-gv;
    
//     id
// }
/// Round x, y float to nearest hex coordinates
// pub fn nearest(x : f32, y : f32) -> (f32, f32) {
//     let zero: f32 = 0.0;
//     let z: f32 = zero - x - y;

//     let mut rx = x.round();
//     let mut ry = y.round();
//     let rz = z.round();

//     let x_diff = (rx - x).abs();
//     let y_diff = (ry - y).abs();
//     let z_diff = (rz - z).abs();

//     if x_diff > y_diff && x_diff > z_diff {
//         rx = -ry - rz;
//     } else if y_diff > z_diff {
//         ry = -rx - rz;
//     } else {
//         // not needed, kept for a reference
//         // rz = -rx - ry;
//     }

//     (rx, ry)
//     // Coordinate {
//     //     x: I::from(rx).unwrap(),
//     //     y: I::from(ry).unwrap(),
//     // }
// }

// pub fn pixel_to_pointy_hex(point: Vec2f, _zoom: f32) -> (f32, f32) {
//     let size = 10.0;
//     let q = ((3.0f32).sqrt()/3.0 * point.x  -  1.0/3.0 * point.y) / size;
//     let r = (                        2.0/3.0 * point.y) / size;

//     nearest(q, -r-q)
// }



pub fn cube_to_axial(cube: Vec3f ) -> Vec2f{
    let q = cube.x;
    let r = cube.z;
    Vec2f::new(q, r)
}

pub fn axial_to_cube(hex: Vec2f ) -> Vec3f {
    let x = hex.x;
    let z = hex.y;
    let y = -x-z;
    Vec3f::new(x, y, z)
}

fn round(f: f32) -> f32 {
    f.round()
}
fn abs(f: f32) -> f32 {
    f.abs()
}
fn sqrt(f: f32) -> f32 {
    f.sqrt()
}

pub fn cube_round(cube: Vec3f) -> Vec3f {
    let mut rx = round(cube.x);
    let mut ry = round(cube.y);
    let mut rz = round(cube.z);

    let x_diff = abs(rx - cube.x);
    let y_diff = abs(ry - cube.y);
    let z_diff = abs(rz - cube.z);

    if x_diff > y_diff && x_diff > z_diff {
        rx = -ry-rz;
    }else if y_diff > z_diff {
        ry = -rx-rz;
    }else{
        rz = -rx-ry;
    }

    Vec3f::new(rx, ry, rz)
}

pub fn hex_round(hex: Vec2f) -> Vec2f {
    return cube_to_axial(cube_round(axial_to_cube(hex)));
}

const size: f32 = 4.0;

pub fn pixel_to_pointy_hex(point: Vec2f ) -> Vec2f {
    let q = (sqrt(3.0)/3.0 * point.x  -  1.0/3.0 * point.y) / size;
    let r = (                        2.0/3.0 * point.y) / size;
    hex_round(Vec2f::new(q, r))
}
