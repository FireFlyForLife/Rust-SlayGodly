#![allow(dead_code)]

//! A very simple shader example.
use std::env;

use cgmath;
use cgmath::{InnerSpace};
use gfx::{self, *};
use ggez;

use ggez::event;
use ggez::graphics::{self, DrawMode, Vertex};
use ggez::timer;
use ggez::{Context, GameResult};

use noise::{NoiseFn, Perlin,Fbm, Seedable, utils::*};

use std::path;
use std::vec::Vec;

// Define the input struct for our shader.
gfx_defines! {
    constant Dim {
        rate: f32 = "u_Rate",
    }
}

const QUAD_VERTICES: [Vertex; 4] = [
    Vertex{pos: [-0.5, 0.5], uv: [0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0]},
    Vertex{pos: [0.5, 0.5], uv: [1.0, 0.0], color: [1.0, 1.0, 1.0, 1.0]},
    Vertex{pos: [0.5, -0.5], uv: [1.0, 1.0], color: [1.0, 1.0, 1.0, 1.0]},
    Vertex{pos: [-0.5, -0.5], uv: [0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0]}
];
const QUAD_INDICES: [u32; 6] = [ 
    0, 1, 2,
    2, 3, 0
];

fn make_quad(ctx: &mut Context, extends: cgmath::Point2::<f32>) -> GameResult<graphics::Mesh> {
    let mut my_vertices = QUAD_VERTICES.clone();
    for i in 0..4 {
        my_vertices[i].pos[0] *= extends.x;
        my_vertices[i].pos[1] *= extends.y;
    }

    graphics::Mesh::from_raw(ctx, &my_vertices, &QUAD_INDICES, None)
}

type Point2I = cgmath::Vector2::<i32>;


#[derive(Debug, Copy, Clone)]
enum HexagonTile{
    Land,
    Water
}

#[derive(Default)]
struct HexagonGrid{
    hexagons: Vec<HexagonTile>,
    width: usize,
    height: usize
}
impl HexagonGrid{
    fn new() -> Self{
        let w = 100;
        let h = 100;

        let mut hexagons = Vec::with_capacity(w*h);

        let perlin = Perlin::new().set_seed(4);
        
        // PlaneMapBuilder::new(&perlin)
        // .set_x_bounds(0.0, 256.0)
        // .set_y_bounds(0.0, 256.0)
        // .set_size(512, 512)
        // .build()
        // .write_to_file("perlin.png");


        for y in 0..h {
            for x in 0..w {
                let island = is_island(&perlin, cgmath::Vector2::<f32>{x: x as f32 + 0.5, y: y as f32 + 0.5}, w, h);

                hexagons.push( if island { HexagonTile::Land } else { HexagonTile::Water});
            }
        } 

        HexagonGrid{hexagons: hexagons, width: w, height: h}
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{:?}", self.hexagons[y*self.width+x] as u8);
            }
            println!("");
        }
    }
}

fn lerp(val: f64, from: f64, to: f64) -> f64 {
    from + (to - from) * val
}

fn is_island(perlin: &Perlin, mut p: cgmath::Vector2::<f32>, w: usize, h: usize) -> bool {
    // let c = perlin.get([ ((p.x+1.0)*128.0) as f64, ((p.y+1.0)*128.0) as f64]) ;
    let mut c = perlin.get([ p.x as f64 * 128.5, p.y as f64 * 128.5]);
    c = c * 0.5 + 0.5;
    assert!(c >= 0.0 && c <= 1.0);


    p.x /= w as f32;
    p.y /= h as f32;

    let island_center = cgmath::Vector2::<f32>::new(0.5, 0.5);
    // p = p - island_center;

    // let dist = p.x.abs().max(p.y.abs()) as f64;
    // c = lerp(c, 0.5, 0.8);
    // let island_center = cgmath::Vector2::<f32>::new(w as f32 * 0.5 , h as f32 * 0.5 );


    c > (0.6 + 0.9 * (((p - island_center).magnitude() as f64)))
    // c > 0.5
    // c - (1.0 - 0.1) * dist * dist < 0.0
}

// world generation

fn generate_grid(mut grid : Vec<f64>,width : &usize, height : &usize,seed : &u32) -> Vec<f64>{
        let perlin = Perlin::new().set_seed(*seed);
        for  y in 0..*height{
            for x in 0..*width {      
            let nx = (x as f64/(*width as f64) - 0.5).abs();
            let ny = (y as f64/(*height as f64) - 0.5).abs();
            let c = perlin.get([nx, ny]).abs();
            grid.push(c);
        }
    }
    grid
}

fn generate_island(grid : &Vec<f64>,width : &usize, height : &usize) -> Vec<f64>{
    // e = (1 + e - d) / 2 where 0 <= d <= 1
    let args: Vec<String> = env::args().collect();
    let modifer : f64 = args[1].parse().unwrap();
    let compare : f64 = args[2].parse().unwrap();
    let mut islands = Vec::with_capacity(width*height);
    let island_center = cgmath::Vector2::<f64>::new(0.5, 0.5);
    for  y in 0..*height{
        for x in 0..*width {
            let index = y * width +x;
            let dx = (grid[index] as f64 - island_center.x).abs() / *width as f64;
            let dy = (grid[index] as f64 - island_center.y).abs() / *height as f64;
            let d = modifer * (dx + dy);
            let e = (1.0 + grid[index] - d) * 0.5;
            islands.push({
                if  e < compare {
                    1.0
                }else{
                    0.0
                }
            }
            );
        }
        }
        islands
}

fn print(grid : &Vec<f64>,width :&usize,height : &usize) {
        for y in 0..*height {
            for x in 0..*width {
                print!("{:?} ", grid[y*width+x]);
            }
            println!("");
        }
}

fn print_as(grid : &Vec<f64>,width :&usize,height : &usize)
{
         for y in 0..*height {
            for x in 0..*width {
                print!("{:?} ", (grid[y*width+x] as u8));
            }
            println!("");
        }   
}



struct MainState {
    dim: Dim,
    shader: graphics::Shader<Dim>
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let dim = Dim { rate: 0.5 };
        let shader = graphics::Shader::new(
            ctx,
            "/basic_150.glslv",
            "/dimmer_150.glslf",
            dim,
            "Dim",
            None,
        )?;
    
        Ok(MainState { dim, shader})
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dim.rate = 0.5 + (((timer::ticks(ctx) as f32) / 100.0).cos() / 2.0);
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
            let quad = make_quad(ctx, cgmath::Point2::<f32>::new(400.0, 400.0))?;
            
            
            graphics::draw(ctx, &quad, (cgmath::Point2::new(400.0, 300.0),))?;
        }

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
}

pub fn main() /*-> GameResult*/ {
        panic!("USE: cargo run 0.4 0.72 400");
        let args : Vec<String>  = env::args().collect();
        let w = 50;
        let h = 50;
        let seed : u32 = args[3].parse().unwrap();
        let mut grid = Vec::with_capacity(w*h);
        grid = generate_grid(grid,&w,&h,&seed);
        let islands = generate_island(&grid, &w, &h);
        print_as(&islands,&w,&h);
    /*
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("shader", "ggez").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
    */
}