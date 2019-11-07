// world generation
use cgmath::InnerSpace;

use crate::hex::{HexagonGrid, HexagonTile};
use crate::Vec2f;
use simdnoise::NoiseBuilder;

pub fn generate_island(
    grid: &mut HexagonGrid,
    modifier_arg: Option<f32>,
    compare_arg: Option<f32>,
) {
    let noise = NoiseBuilder::fbm_2d(grid.width, grid.height)
        .with_seed(grid.seed as i32)
        .generate_scaled(0.0, 1.0);
    // e = (1 + e - d) / 2 where 0 <= d <= 1;
    let modifer: f32 = modifier_arg.unwrap_or(0.45);
    let compare: f32 = compare_arg.unwrap_or(0.6);
    let island_center = Vec2f::new(grid.width as f32 * 0.5, grid.height as f32 * 0.5);
    for y in 0..grid.height {
        for x in 0..grid.width {
            let index = y * grid.width + x;
            let p = Vec2f::new(x as f32, y as f32);

            let dist = (p - island_center).magnitude()
                / (Vec2f::new(0.0, 0.0) - island_center).magnitude();

            let e = (1.0 + noise[index] - dist * modifer) * 0.5;

            grid.hexagons[index] = if compare < e {
                HexagonTile::Land
            } else {
                HexagonTile::Water
            };
        }
    }
}
