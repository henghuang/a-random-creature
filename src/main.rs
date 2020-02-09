//a random life
//author: Heng Huang
//project page: github.com/henghuang

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::path;
use std::time::{Duration, Instant};
// import commonly used items from the prelude:
use rand::prelude::*;

const SCREEN_SIZE: (f32, f32) = (800 as f32, 600 as f32);
const UPDATES_PER_SECOND: f32 = 64.0;
// And we get the milliseconds of delay that this update rate corresponds to.
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

struct Life {
    shape: Vec<ggez::mint::Point2<f32>>,
    position: ggez::mint::Point2<f32>,
    last_position: ggez::mint::Point2<f32>,
    head_dir: ggez::mint::Point2<f32>,
    rotation: f32,
    last_update: Instant,
}

impl Life {
    /// Load images and create meshes.
    fn new(_ctx: &mut Context) -> GameResult<Life> {
        let init_points = vec![
            ggez::mint::Point2 {
                x: -0.5 * 20.0,
                y: -(0.75 as f32).sqrt() / 2.0 * 20.0,
            },
            ggez::mint::Point2 {
                x: 0.5 * 20.0,
                y: -(0.75 as f32).sqrt() / 2.0 * 20.0,
            },
            ggez::mint::Point2 {
                x: 0.0 * 20.0,
                y: (0.75 as f32).sqrt() / 2.0 * 20.0,
            },
        ];
        let position = ggez::mint::Point2 {
            x: SCREEN_SIZE.0 / 2.0,
            y: SCREEN_SIZE.1 / 2.0,
        };
        let life = Life {
            shape: init_points,
            position: position,
            last_position: position,
            rotation: 0.0,
            head_dir: ggez::mint::Point2 { x: 0.0, y: 0.0 },
            last_update: Instant::now(),
        };

        Ok(life)
    }
    fn update_rotation(&mut self, rotation: f32) {
        for item in self.shape.iter_mut() {
            let x = item.x;
            let y = item.y;
            item.x = rotation.cos() * x - rotation.sin() * y;
            item.y = rotation.sin() * x + rotation.cos() * y;
        }
        self.head_dir = self.shape[0];
        let norm = (self.head_dir.x.powi(2) + self.head_dir.y.powi(2)).sqrt();
        self.head_dir.x /= norm;
        self.head_dir.y /= norm;
    }
    fn update_forward(&mut self) {
        let step = 1.0;
        self.last_position = self.position;
        self.position.x += step * self.head_dir.x;
        self.position.y += step * self.head_dir.y;
    }
    fn add_position(&mut self) -> GameResult<Vec<ggez::mint::Point2<f32>>> {
        let mut final_shape = self.shape.clone();
        let mut outofbox = false;
        for item in &mut final_shape {
            item.x += self.position.x;
            item.y += self.position.y;
            if item.x < 0.0 || item.x > SCREEN_SIZE.0 || item.y < 0.0 || item.y > SCREEN_SIZE.1 {
                outofbox = true;
                break;
            }
        }
        //a simple box
        if outofbox {
            final_shape = self.shape.clone();
            self.position = self.last_position;
            for item in &mut final_shape {
                item.x += self.position.x;
                item.y += self.position.y;
            }
        }
        Ok(final_shape)
    }
}

impl event::EventHandler for Life {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            let mut rng = thread_rng();
            self.rotation = rng.gen_range(-0.3, 0.3);
            self.update_rotation(self.rotation);
            self.update_forward();
            self.last_update = Instant::now();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let body_shape = self.add_position()?;
        let body = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &body_shape,
            [1.0, 0.3, 0.0, 1.0].into(),
        )?;
        graphics::draw(
            ctx,
            &body,
            graphics::DrawParam::new(), //.dest(self.position),
        )?;

        let head = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            body_shape[0],
            3.0,
            0.1,
            [0.631, 0.211, 0.662, 1.0].into(),
        )?;
        graphics::draw(ctx, &head, graphics::DrawParam::new())?;
        graphics::present(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("life", "henghuang");

    let (ctx, events_loop) = &mut cb
        .window_setup(ggez::conf::WindowSetup::default().title("Random Life"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    println!("{}", graphics::renderer_info(ctx)?);
    let state = &mut Life::new(ctx).unwrap();
    event::run(ctx, events_loop, state)
}
