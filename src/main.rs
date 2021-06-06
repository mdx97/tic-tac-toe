extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::mouse::MouseButton;
use std::time::{Duration, Instant};

/// How many pixels wide are the outer borders of the playing area.
const BORDER_THICKNESS: i32 = 20;

/// The height and width of the window, in pixels.
const WINDOW_SIZE: u32 = 680;

/// The coordinate where the playing area starts.
const PLAYING_AREA_OFFSET: u32 = BORDER_THICKNESS as u32 * 2;

/// The height and width of the playing area, in pixels.
const PLAYING_AREA_SIZE: u32 = WINDOW_SIZE - (PLAYING_AREA_OFFSET * 2);

/// The number of squares in the horizontal and vertical direction.
/// Total number of squares in the playing area = SQUARES^2
const SQUARES: u32 = 4;

/// The height and width of each square, in pixels.
const SQUARE_SIZE: u32 = PLAYING_AREA_SIZE / SQUARES;

/// How many extra pixels the border must fill in so there is no space between the outer squares and the border.
const FILL_IN: u32 = PLAYING_AREA_SIZE - (SQUARE_SIZE * SQUARES);

/// The number of seconds to wait after someone has won a game before clearing the board.
const WIN_TIMEOUT: u64 = 2;

struct GameState {
    freeze_until: Option<Instant>,
    squares: Vec<Square>,
    turn: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            freeze_until: None,
            squares: vec![Square::Empty; (SQUARES * SQUARES) as usize],
            turn: true,
        }
    }
}

#[derive(Clone, PartialEq)]
enum Square {
    X,
    O,
    Empty,
}

/// Fills a rectangle with the given color.
fn fill_rectangle(canvas: &mut WindowCanvas, rectangle: Rect, color: Color) {
    canvas.set_draw_color(color);
    canvas.fill_rect(rectangle).unwrap();
}

/// Returns which square the given coordinates lie within, or None if outside the playing area.
fn get_square_from_coords(x: i32, y: i32) -> Option<usize> {
    let x = x - PLAYING_AREA_OFFSET as i32;
    let y = y - PLAYING_AREA_OFFSET as i32;
    if x <= 0 || y <= 0 || x >= (SQUARE_SIZE * SQUARES) as i32 || y >= (SQUARE_SIZE * SQUARES) as i32 {
        return None;
    }
    let col = x as u32 / SQUARE_SIZE;
    let row = y as u32 / SQUARE_SIZE;
    Some(((row * SQUARES) + col) as usize)
}

/// Returns a new rect that covers the inner portion of the given rectangle.
fn get_inner_rect(rect: Rect) -> Rect {
    let mut new = rect.clone();
    new.set_x(rect.x() + 1);
    new.set_y(rect.y() + 1);
    new.set_width(rect.width() - 2);
    new.set_height(rect.height() - 2);
    new
}

/// Returns whether or not the given board state has a winner.
fn has_winner(squares: &Vec<Square>) -> bool {
    for i in 0..SQUARES as usize {
        if line_has_winner(&squares, |constant, i| (constant, i), i) || line_has_winner(&squares, |constant, i| (i, constant), i) {
            return true;
        }
    }
    line_has_winner(&squares, |_, i| (i, i), 0) || line_has_winner(&squares, |_, i| (SQUARES as usize - i - 1, i), 0)
}

/// Returns whether or not the given line has a winner.
/// This function operates in kind of a wonky way. Essentially it traverses the size of the board, and for each iteration,
/// executes the provided function get_square() with the arguments constant, i (the iteration).
fn line_has_winner(squares: &Vec<Square>, get_square: fn(usize, usize) -> (usize, usize), constant: usize) -> bool {
    let start = get_square(constant, 0);
    let line_square = get_square_flatten_index(squares, start.0, start.1);
    if *line_square == Square::Empty {
        return false;
    }
    for i in 1..SQUARES as usize {
        let square = get_square(constant, i);
        if *get_square_flatten_index(squares, square.0, square.1) != *line_square {
            return false;
        }
    }
    true
}

/// Returns a square from the square vector by treating it as a table.
fn get_square_flatten_index(squares: &Vec<Square>, row: usize, col: usize) -> &Square {
    &squares[(row * SQUARES as usize) + col]
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let window = sdl.video().unwrap().window("Tic-Tac-Toe!", WINDOW_SIZE, WINDOW_SIZE)
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

    let mut state = GameState::default();

    loop {
        if state.freeze_until.is_some() {
            if Instant::now() > state.freeze_until.unwrap() {
                state.freeze_until = None;
                state = GameState::default();
            } else {
                // We need to drain the event pump so that events from the
                // frozen period are not picked up once input is re-enabled.
                for _ in event_pump.poll_iter() { }
            }
        } else {
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        return;
                    },
                    Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                        if let Some(square) = get_square_from_coords(x, y) {
                            if state.squares[square] == Square::Empty {
                                state.squares[square] = if state.turn { Square::X } else { Square::O };
                                state.turn = !state.turn;
                            }
                        }
                    }
                    _ => {}
                }
            }

            if has_winner(&state.squares) || !state.squares.iter().any(|s| *s == Square::Empty) {
                // TODO: Display who the winner is. Preferably via text in the game window, and not console output.
                println!("There is a winner!");
                state.freeze_until = Some(Instant::now() + Duration::from_secs(WIN_TIMEOUT));
            }

            fill_rectangle(&mut canvas, screen_rect, Color::BLACK);
            fill_rectangle(&mut canvas, border_rect, Color::WHITE);
            fill_rectangle(&mut canvas, playing_area_rect, Color::BLACK);

            for i in 0..SQUARES as usize {
                for j in 0..SQUARES as usize {
                    let rect = Rect::new((PLAYING_AREA_OFFSET + (SQUARE_SIZE * i as u32)) as i32, (PLAYING_AREA_OFFSET + (SQUARE_SIZE * j as u32)) as i32, SQUARE_SIZE, SQUARE_SIZE);
                    canvas.set_draw_color(Color::WHITE);
                    canvas.draw_rect(rect).unwrap();

                    match get_square_flatten_index(&state.squares, j, i) {
                        Square::X => {
                            canvas.set_draw_color(Color::RED);
                            canvas.fill_rect(get_inner_rect(rect)).unwrap();
                        },
                        Square::O => {
                            canvas.set_draw_color(Color::BLUE);
                            canvas.fill_rect(get_inner_rect(rect)).unwrap();
                        },
                        Square::Empty => (),
                    };
                }
            }
            canvas.present();
        }
    }
}
