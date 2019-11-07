#![allow(dead_code)]

//! A very simple shader example.
use std::env;

use std::path;
use std::vec::Vec;

use cgmath;
use cgmath::InnerSpace;
use gfx::{self, *};
use ggez;
use num;

use ggez::event;
use ggez::graphics::{self, DrawMode, Vertex, Image, Drawable, DrawParam, FilterMode, WrapMode, Mesh};
use ggez::timer;
use ggez::{Context, GameResult};
use ggez::conf::WindowMode;
use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard::is_key_pressed;

use simdnoise::NoiseBuilder;

mod hex;
mod num_utils;
mod terrain_generator;

// Define the input struct for our shader.
gfx_defines! {
    constant Dim {
        rate: f32 = "u_Rate",
        texSizeX: f32 = "u_TexDimensionsX",
        texSizeY: f32 = "u_TexDimensionsY",
        zoom: f32 = "u_Zoom",
        camera_pos: [f32; 2] = "u_CamPos",
    }
}

const QUAD_VERTICES: [Vertex; 4] = [
    Vertex {
        pos: [-0.5, 0.5],
        uv: [0.0, 0.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex {
        pos: [0.5, 0.5],
        uv: [1.0, 0.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex {
        pos: [0.5, -0.5],
        uv: [1.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex {
        pos: [-0.5, -0.5],
        uv: [0.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
];
const QUAD_INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];

fn make_quad(ctx: &mut Context, extends: cgmath::Point2<f32>, texture: Option<Image>) -> GameResult<graphics::Mesh> {
    let mut my_vertices = QUAD_VERTICES.clone();
    let aspect = extends.x / extends.y;

    for i in 0..4 {
        my_vertices[i].pos[0] *= extends.x;
        my_vertices[i].pos[1] *= extends.y;
        // if extends.x > extends.y{
            my_vertices[i].uv[0] *= aspect;
            my_vertices[i].uv[1] *= 1.0;
        // } else {
        //     my_vertices[i].uv[0] *= 1.0;
        //     my_vertices[i].uv[1] *= extends.y / extends.x;
        // }
    }

    graphics::Mesh::from_raw(ctx, &my_vertices, &QUAD_INDICES, texture)
}

pub type Point2I = cgmath::Vector2<i32>;
pub type Vec2f = cgmath::Vector2<f32>;

// fn print_perlin(grid: &Vec<f32>, width: usize, height: usize) {
//     for y in 0..height {
//         for x in 0..width {
//             print!("{:.2} ", grid[y * width + x]);
//         }
//         println!("");
//     }
// }

// fn print(grid: &Vec<HexagonTile>, width: &usize, height: &usize) {
//     for y in 0..*height {
//         for x in 0..*width {
//             print!("{:?} ", grid[y * width + x]);
//         }
//         println!("");
//     }
// }

// fn print_as(grid: &Vec<HexagonTile>, width: &usize, height: &usize) {
//     for y in 0..*height {
//         for x in 0..*width {
//             print!("{:?} ", (grid[y * width + x] as u8));
//         }
//         println!("");
//     }
// }

struct MainState {
    dim: Dim,
    shader: graphics::Shader<Dim>,
    grid_texture: graphics::Image,
    grid: hex::HexagonGrid,
    quad: Mesh,
    camera_pos: Vec2f,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        graphics::set_mode(ctx, WindowMode::default().dimensions(800.0, 800.0).resizable(true))?;

        let args: Vec<String> = env::args().collect();
        let w = 100;
        let h = 100;
        let seed: i32 = args.get(3).unwrap_or(&"123".to_owned()).parse().unwrap();

        let mut grid = hex::HexagonGrid::new(w, h, Some(seed));
        terrain_generator::generate_island(&mut grid, args.get(1).unwrap_or(&"not a number".to_owned()).parse().ok(), args.get(2).unwrap_or(&"not a number".to_owned()).parse().ok());

        let color_array = grid.get_rgba8();
        let mut grid_texture = Image::from_rgba8(ctx, grid.width as u16, grid.height as u16, color_array.as_slice())?;
        grid_texture.set_filter(FilterMode::Nearest);
        grid_texture.set_wrap(WrapMode::Border, WrapMode::Border);

        let dim = Dim { rate: 0.5, texSizeX: grid.width as f32, texSizeY: grid.height as f32, zoom: 100.0, camera_pos: [0.0, 0.0] };
        let shader = graphics::Shader::new(
            ctx,
            "/basic_150.glslv",
            "/dimmer_150.glslf",
            dim,
            "Dim",
            None,
        )?;

        let quad = make_quad(ctx, cgmath::Point2::<f32>::new(800.0*1.0, 600.0*1.0), Some(grid_texture.clone()))?;

        let camera_pos = Vec2f::new(0.0, 0.0);

        Ok(MainState { dim, shader, grid_texture, grid, quad, camera_pos })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dim.rate = 0.5 + (((timer::ticks(ctx) as f32) / 100.0).cos() / 2.0);
        self.dim.camera_pos = [self.camera_pos.x, self.camera_pos.y];

        let movement_speed: f32 = 20.0 * ggez::timer::delta(ctx).as_secs_f32();
        if is_key_pressed(ctx, KeyCode::Left) {
            self.camera_pos.x -= movement_speed;
        } else if is_key_pressed(ctx, KeyCode::Right) {
            self.camera_pos.x += movement_speed;
        } else if is_key_pressed(ctx, KeyCode::Up) {
            self.camera_pos.y -= movement_speed;
        } else if is_key_pressed(ctx, KeyCode::Down) {
            self.camera_pos.y += movement_speed;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            cgmath::Point2::new(100.0, 300.0),
            100.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (cgmath::Point2::new(0.0, 0.0),))?;

        {
            let _lock = graphics::use_shader(ctx, &self.shader);
            self.shader.send(ctx, self.dim)?;
            // let circle = graphics::Mesh::new_circle(
            //     ctx,
            //     DrawMode::fill(),
            //     cgmath::Point2::new(400.0, 300.0),
            //     100.0,
            //     2.0,
            //     graphics::WHITE,
            // )?;
            let (scr_w, _scr_h) = graphics::size(ctx);
            let draw_params = DrawParam::new()
                .dest(cgmath::Point2::new(scr_w / 2.0, 0.0));
            
            graphics::draw(ctx, &self.quad, draw_params)?;
        }

        //self.grid_texture.draw(ctx, DrawParam::new().scale(Vec2f::new(0.25, 0.25)))?;

        let circle = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            cgmath::Point2::new(700.0, 300.0),
            100.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (cgmath::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }

    /// Called when the user resizes the window, or when it is resized
    /// via [`graphics::set_mode()`](../graphics/fn.set_mode.html).
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let _ = graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height)).unwrap();

        self.quad = make_quad(ctx, cgmath::Point2::<f32>::new(width * 1.0, height * 2.0), Some(self.grid_texture.clone())).unwrap();

        println!("Resizing!!! {}, {}", width, height);
    }

    /// The mousewheel was scrolled, vertically (y, positive away from and negative toward the user)
    /// or horizontally (x, positive to the right and negative to the left).
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        self.dim.zoom -= y;
        self.dim.zoom = num::clamp(self.dim.zoom, 10.0, 150.0);
    }

    /// A keyboard button was pressed.
    ///
    /// The default implementation of this will call `ggez::event::quit()`
    /// when the escape key is pressed.  If you override this with
    /// your own event handler you have to re-implment that
    /// functionality yourself.
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            ggez::event::quit(ctx);
        }
    }
}

pub fn main() -> GameResult {
    //We now use: cargo run 0.45 0.6 429

    //panic!("USE: cargo run 0.4 0.72 400");

    //let mut grid = Vec::with_capacity(w * h);
    //grid = generate_grid(grid, w, h, seed);
    //let islands = generate_island(&grid, w, h);
    //print_as(&islands, &w, &h);
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("Slay Godly", "Maiko & Simon").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
