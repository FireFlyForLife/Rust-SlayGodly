use ggez::graphics::Color;

use crate::hex;
use crate::hex::{HexagonGrid, HexagonTile};
use crate::Vec2f;


pub fn cpu_render_map(grid: &HexagonGrid, cam_pos: Vec2f, scr_size: [usize; 2]) -> Vec<u8> {
    let mut img = Vec::with_capacity(grid.width * grid.height);

    for y in 0..scr_size[1] {
        for x in 0..scr_size[0] {
            let hex_coord = hex::pixel_to_pointy_hex(Vec2f::new(x as f32, y as f32) + cam_pos);
            let tile_opt = grid.get(hex_coord.x, hex_coord.y);
            
            let color = if let Some(tile) = tile_opt {
                    if tile == HexagonTile::Land { 
                        Color::new(0.0, 1.0, 0.0, 1.0) 
                    } else if tile == HexagonTile::Water { 
                        Color::new(0.0, 0.0, 1.0, 1.0)
                    } else {
                        Color::new(1.0, 0.0, 0.0, 1.0)
                    }
                } else {
                    Color::new(0.0, 0.0, 0.0, 1.0)
                };
            let (r, g, b, a) = color.to_rgba();

            img.push(r);
            img.push(g);
            img.push(b);
            img.push(a);
        }
    }

    img
}