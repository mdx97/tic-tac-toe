extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::mouse::MouseButton;
use std::time::Duration;

/// How many pixels wide are the outer borders of the playing area.
const BORDER_THICKNESS: i32 = 10;

/// The height and width of the window, in pixels.
const WINDOW_SIZE: u32 = 512;

/// The coordinate where the playing area starts.
const PLAYING_AREA_OFFSET: u32 = BORDER_THICKNESS as u32 * 2;

/// The height and width of the playing area, in pixels.
const PLAYING_AREA_SIZE: u32 = WINDOW_SIZE - (PLAYING_AREA_OFFSET * 2);

/// The height and width of each square, in pixels.
const SQUARE_SIZE: u32 = PLAYING_AREA_SIZE / 3;

/// How many extra pixels the border must fill in so there is no space between the outer squares and the border.
const FILL_IN: u32 = PLAYING_AREA_SIZE - (SQUARE_SIZE * 3);

/// Fills a rectangle with the given color.
fn fill_rectangle(canvas: &mut WindowCanvas, rectangle: Rect, color: Color) {
    canvas.set_draw_color(color);
    canvas.fill_rect(rectangle).unwrap();
}

/// Returns which square the given coordinates lie within, or None if outside the playing area.
fn get_square_from_coords(x: i32, y: i32) -> Option<u32> {
    let play_x = x - PLAYING_AREA_OFFSET as i32;
    let play_y = y - PLAYING_AREA_OFFSET as i32;
    if play_x < 0 || play_y < 0 || play_x >= (SQUARE_SIZE * 3) as i32 || play_y >= (SQUARE_SIZE * 3) as i32 {
        return None;
    }
    let col = play_x as u32 / SQUARE_SIZE;
    let row = play_y as u32 / SQUARE_SIZE;
    Some(((row * 3) + col) as u32)
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let window = sdl.video().unwrap().window("rust-sdl2 demo", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let screen_rect = Rect::new(0, 0, WINDOW_SIZE, WINDOW_SIZE);
    let border_rect = Rect::new(BORDER_THICKNESS, BORDER_THICKNESS, WINDOW_SIZE - (BORDER_THICKNESS as u32 * 2), WINDOW_SIZE - (BORDER_THICKNESS as u32 * 2));
    let playing_area_rect = Rect::new(
        BORDER_THICKNESS * 2,
        BORDER_THICKNESS * 2,
        WINDOW_SIZE - (BORDER_THICKNESS as u32 * 4) - FILL_IN,
        WINDOW_SIZE - (BORDER_THICKNESS as u32 * 4) - FILL_IN,
    );

    loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return;
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    // TODO: Check if these coords can ever be negative. Why i32?
                    println!("{:?}", get_square_from_coords(x, y));
                }
                _ => {}
            }
        }

        fill_rectangle(&mut canvas, screen_rect, Color::BLACK);
        fill_rectangle(&mut canvas, border_rect, Color::WHITE);
        fill_rectangle(&mut canvas, playing_area_rect, Color::BLACK);

        canvas.set_draw_color(Color::WHITE);
        for i in 0..3 {
            for j in 0..3 {
                canvas.draw_rect(Rect::new((PLAYING_AREA_OFFSET + (SQUARE_SIZE * i)) as i32, (PLAYING_AREA_OFFSET + (SQUARE_SIZE * j)) as i32, SQUARE_SIZE, SQUARE_SIZE)).unwrap();
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
