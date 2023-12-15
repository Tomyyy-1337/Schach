extern crate sdl2;

use schach::Schach;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::rect::Point;

pub mod texture_manager;
pub mod schach;

const SQUARE_SIZE:u32 = 100;

pub fn main() -> Result<(), String> {
    rayon::ThreadPoolBuilder::new().num_threads(16).build_global().unwrap();

    let mut brett = Schach::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Schach", SQUARE_SIZE * 8, SQUARE_SIZE * 8)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext> = canvas.texture_creator();
    let mut tex_man: texture_manager::ResourceManager<'_, String, sdl2::render::Texture<'_>, sdl2::render::TextureCreator<sdl2::video::WindowContext>> = texture_manager::TextureManager::new(&texture_creator);

    tex_man.load("img/black-bishop.png")?;
    tex_man.load("img/black-king.png")?;
    tex_man.load("img/black-queen.png")?;
    tex_man.load("img/black-knight.png")?;
    tex_man.load("img/black-pawn.png")?;
    tex_man.load("img/black-rook.png")?;
    tex_man.load("img/white-bishop.png")?;
    tex_man.load("img/white-king.png")?;
    tex_man.load("img/white-queen.png")?;
    tex_man.load("img/white-knight.png")?;
    tex_man.load("img/white-pawn.png")?;
    tex_man.load("img/white-rook.png")?;

    let sdl_context = sdl2::init().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut selected_squares: Vec<(i32, i32)> = Vec::new();
    let mut active_piece: Option<(i32, i32)> = None;
    let mut arrows: Vec<(i32, i32, i32, i32)> = Vec::new();
    let mut start_pos_right: Option<(i32, i32)> = None; 
    let mut start_pos_left: Option<(i32, i32)> = None; 

    'running: loop {
        canvas.clear();
        
        match brett.get_outcome() {
            schach::Outcome::None => (),
            _ => {
                print_outcome(&brett);
                brett = Schach::new();
                ::std::thread::sleep(Duration::new(3, 0));
            },
        }
        if brett.active_player == schach::Color::White {
            let (a,b,c,d) = brett.best_move(4); 
            brett.move_piece(a, b, c, d);
            //print_outcome(&brett);
            
        } 
        else if brett.active_player == schach::Color::Black {
            let (a,b,c,d) = brett.best_move(4); 
            brett.move_piece(a, b, c, d);   
            //print_outcome(&brett);
        } 

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    brett = schach::Schach::new();
                    selected_squares.clear();
                    arrows.clear();
                },
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            start_pos_left = Some((x / SQUARE_SIZE as i32, y / SQUARE_SIZE as i32));
                        }
                        MouseButton::Right => {
                            start_pos_right = Some((x / SQUARE_SIZE as i32, y / SQUARE_SIZE as i32));
                        }
                        _ => {}
                    }
                },
                Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    match mouse_btn {
                        MouseButton::Right => {
                            if start_pos_right == Some((x / SQUARE_SIZE as i32, y / SQUARE_SIZE as i32)) {
                                let tmp = (x / SQUARE_SIZE as i32, y / SQUARE_SIZE as i32);
                                if selected_squares.contains(&tmp) {
                                    let index: usize = selected_squares.iter().position(|&r| r == tmp).unwrap();
                                    selected_squares.remove(index);
                                } else {
                                    selected_squares.push((x / SQUARE_SIZE as i32, y / SQUARE_SIZE as i32));
                                }
                            } else {
                                if let Some((start_x, start_y)) = start_pos_right {
                                    let tmp = (start_x, start_y, x / SQUARE_SIZE as i32, y / SQUARE_SIZE as i32);
                                    if arrows.contains(&tmp) {
                                        let index = arrows.iter().position(|&r| r == tmp).unwrap();
                                        arrows.remove(index);
                                    } else {
                                        arrows.push(tmp);
                                    }
                                }
                            }
                            start_pos_right = None;
                        }
                        MouseButton::Left => {
                            if start_pos_left == Some((x / SQUARE_SIZE as i32, y / SQUARE_SIZE as i32)) {
                                selected_squares = brett.get_legal_moves((x as u32 / SQUARE_SIZE) as u64, (y as u32 / SQUARE_SIZE) as u64, 1);
                                if !active_piece.is_none() && active_piece != start_pos_left {
                                    let (c,d) = start_pos_left.unwrap();
                                    let a = active_piece.unwrap().0;
                                    let b = active_piece.unwrap().1;
                                    if brett.get_legal_moves(a as u64, b as u64, 1).contains(&(c,d)) {
                                        brett.move_piece(a as u64, b as u64, c as u64, d as u64);
                                        print_outcome(&brett);
                                        selected_squares.clear();
                                        active_piece = None;
                                    }
                                } 
                                if selected_squares.len() > 0 {
                                    active_piece = start_pos_left;
                                }
                                arrows.clear();
                            } else if !start_pos_left.is_none() {
                                let (a,b) = start_pos_left.unwrap();
                                let c = x / SQUARE_SIZE as i32;
                                let d = y / SQUARE_SIZE as i32;
                                if brett.get_legal_moves(a as u64, b as u64, 1).contains(&(c,d)) {
                                    brett.move_piece(a as u64, b as u64, c as u64, d as u64);
                                    print_outcome(&brett);
                                    selected_squares.clear();
                                }
                            }
                        }
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        //Brett
        for i in 0..8 {
            for j in 0..8 {
                let x = i * SQUARE_SIZE;
                let y = j * SQUARE_SIZE;
                
                let color = if (i + j) % 2 == 0 && selected_squares.contains(&(i as i32, j as i32)) {
                    Color::RGB(255, 150, 150)
                } else if (i + j) % 2 == 1 && selected_squares.contains(&(i as i32, j as i32)) {
                    Color::RGB(100, 70, 30) 
                } else if (i + j) % 2 == 0 {
                    Color::RGB(255, 255, 255) 
                } else {
                    Color::RGB(20, 100, 20) 
                };
                canvas.set_draw_color(color);
                canvas.fill_rect(sdl2::rect::Rect::new(x as i32, y as i32, SQUARE_SIZE, SQUARE_SIZE)).unwrap();
            }
        }

        // Figuren
        for (c,p,i,j) in  brett.get_positions() {
            let texture_name = match (p,c) {
                (schach::Piece::King, schach::Color::White) => "img/white-king.png",
                (schach::Piece::King, schach::Color::Black) => "img/black-king.png",
                (schach::Piece::Queen, schach::Color::White) => "img/white-queen.png",
                (schach::Piece::Queen, schach::Color::Black) => "img/black-queen.png",
                (schach::Piece::Rook, schach::Color::White) => "img/white-rook.png",
                (schach::Piece::Rook, schach::Color::Black) => "img/black-rook.png",
                (schach::Piece::Bishop, schach::Color::White) => "img/white-bishop.png",
                (schach::Piece::Bishop, schach::Color::Black) => "img/black-bishop.png",
                (schach::Piece::Knight, schach::Color::White) => "img/white-knight.png",
                (schach::Piece::Knight, schach::Color::Black) => "img/black-knight.png",
                (schach::Piece::Pawn, schach::Color::White) => "img/white-pawn.png",
                (schach::Piece::Pawn, schach::Color::Black) => "img/black-pawn.png",
            };
            let img_size = 128;
            let texture = tex_man.load(&texture_name)?;
            let src = Rect::new(0,0,img_size,img_size);
            let x: i32 = (i as u32 * SQUARE_SIZE) as i32;
            let y: i32 = (j as u32 * SQUARE_SIZE) as i32;
            let dest = Rect::new(x,y,SQUARE_SIZE,SQUARE_SIZE);
            let center = Point::new( 0,0);

            canvas.copy_ex(
                &texture, 
                src,  
                dest,
                0.0,
                center, 
                false, 
                false 
            )?;                  
        }

        
        // Pfeile
        for (start_x, start_y, end_x, end_y) in &arrows {
            let start = (start_x * SQUARE_SIZE as i32 + SQUARE_SIZE as i32 / 2, start_y * SQUARE_SIZE as i32 + SQUARE_SIZE as i32 / 2);
            let end = (end_x * SQUARE_SIZE as i32 + SQUARE_SIZE as i32 / 2, end_y * SQUARE_SIZE as i32 + SQUARE_SIZE as i32 / 2);
            draw_arrow(&mut canvas, start, end);
        }
        
        canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    Ok(())
}

fn print_outcome(brett: &schach::Schach) {
    match brett.get_outcome() {
        schach::Outcome::Checkmate(schach::Color::White) => println!("Weiss gewinnt"),
        schach::Outcome::Checkmate(schach::Color::Black) => println!("Schwarz gewinnt"),
        schach::Outcome::Stalemate => println!("Stalemate"),
        schach::Outcome::None => println!("Evaluation: {}", brett.eval_position()),
    }
}

fn draw_arrow(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, start: (i32, i32), end: (i32, i32)) {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.draw_line(start, end).unwrap();

    let dir_x = end_x - start_x;
    let dir_y = end_y - start_y;

    let len = ((dir_x * dir_x + dir_y * dir_y) as f64).sqrt();
    let dir_x = (dir_x as f64 / len * 10.0) as i32;
    let dir_y = (dir_y as f64 / len * 10.0) as i32;

    let arrow_point1 = (end_x - dir_x - dir_y, end_y - dir_y + dir_x);
    let arrow_point2 = (end_x - dir_x + dir_y, end_y - dir_y - dir_x);

    canvas.draw_line(end, arrow_point1).unwrap();
    canvas.draw_line(end, arrow_point2).unwrap();
}