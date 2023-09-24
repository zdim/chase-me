use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod controllers;
use controllers::player;
mod models;
mod view;
use view::level;
use view::world;

use crate::models::position::Position;
use crate::models::vector::Vector;
use crate::view::camera;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    const FRAME_CAP: i8 = 60;
    const TARGET_TICKS: f32 = (1.0 / (FRAME_CAP as f32)) * 1000.0;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Game", WINDOW_WIDTH, WINDOW_HEIGHT).build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // should this just be the camera?
    let world: world::Renderer = world::Renderer {
        screen_area: Rect::new(0, 0, 800, 600),
        clear_color: Color::RGB(64, 192, 255),
    };

    let mut event_queue = sdl_context.event_pump()?;
    canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));

    // set up our entities 
    let mut player = player::Player {
        location: Position { x: 0.0, y: 0.0 },
        destination: None,
        speed: 250.0,
        direction: Vector { x: 0.0, y: 0.0 },
    };

    let level = level::Renderer {
        area: Rect::new(-100, -100, 400, 400)
    };

    let mut camera = camera::Camera {
        size: Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT),
        location: Position { x: 0.0, y: 0.0 }
    };

    let timer = sdl_context.timer().unwrap();
    let mut current_ticks = 0;
    let mut total_ticks: u32 = 0;

    'running: loop {
        let elapsed_ticks = timer.ticks() - total_ticks;
        total_ticks = timer.ticks();
        current_ticks += elapsed_ticks;
        let delta = (current_ticks as f64) / 1000.0;

        // cap the frame rate :(
        if (current_ticks as f32) < TARGET_TICKS {
            continue;
        }

        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        let is_left_mouse_down = event_queue
            .mouse_state()
            .is_mouse_button_pressed(MouseButton::Left);
        if is_left_mouse_down {
            let x = event_queue.mouse_state().x() as f32;
            let y = event_queue.mouse_state().y() as f32;
            player.set_destination(Position { x, y });
        }

        player.update(delta);

        // the camera's location should be offset by half the width & height
        camera.location = Position {
            x: player.location.x - ((WINDOW_WIDTH / 2) as f32),
            y: player.location.y - ((WINDOW_HEIGHT / 2) as f32),
        };

        // clear
        world.clear(&mut canvas);

        // render the level
        level.render(&mut canvas);

        // render the player
        player.render(&mut canvas, camera.location);

        canvas.present();

        current_ticks = 0;
    }

    Ok(())
}
