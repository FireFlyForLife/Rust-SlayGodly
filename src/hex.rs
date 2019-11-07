use std::vec::Vec;
use ggez::graphics::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HexagonTile {
    Land,
    Water,
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

            let color = if self.hexagons[i] == HexagonTile::Land { Color::new(0.0, 1.0, 0.0, 1.0) } else { Color::new(0.0, 0.0, 1.0, 1.0)};
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