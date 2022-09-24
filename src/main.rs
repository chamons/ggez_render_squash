use std::{env, path};

use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Transform};
use ggez::input::keyboard::*;
use ggez::mint;
use ggez::{conf, Context, ContextBuilder, GameResult};

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(conf::WindowMode {
            width: 2560.0,
            height: 1920.0,
            ..Default::default()
        })
        .add_resource_path(resource_dir)
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    map: ggez::graphics::Image,
    active: bool,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let map = ggez::graphics::Image::from_path(ctx, "/map1.png").unwrap();
        MyGame { map, active: false }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        let mut scale = 2.0;

        if self.active {
            canvas.set_screen_coordinates(graphics::Rect::new(0.0, 0.0, 1280.0, 960.0));
            scale = 1.0
        }

        let draw_params = DrawParam {
            transform: Transform::Values {
                dest: mint::Point2 { x: 31.0, y: 31.0 },
                rotation: 0.0,
                scale: mint::Vector2 { x: scale, y: scale },
                offset: mint::Point2 { x: 0.0, y: 0.0 },
            },
            ..Default::default()
        };

        canvas.draw(&self.map, draw_params);

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult<()> {
        if input.keycode == Some(KeyCode::F1) {
            self.active = !self.active;
        }
        Ok(())
    }
}
