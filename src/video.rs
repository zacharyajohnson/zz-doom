use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{RendererContext, Texture};
use sdl2::surface::Surface;
use sdl2::sys::SDL_Rect;
use sdl2::Sdl;
use std::thread;
use std::time::Duration;

struct Patch {
    pub width: i16,
    pub height: i16,
    pub left_offset: i16,
    pub top_offset: i16,
    pub column_ofs: [i32; SCREEN_WIDTH],
}

struct Column {
    pub top_delta: u8,
    pub length: u8,
}

pub const SCREEN_WIDTH: usize = 320;
pub const SCREEN_HEIGHT: usize = 200;

pub fn patch_to_pixel(mut data: Vec<u8>) {
    let mut screen: [u8; SCREEN_WIDTH * SCREEN_HEIGHT] = [0; SCREEN_WIDTH * SCREEN_HEIGHT];
    let patch: Patch = Patch {
        width: i16::from_le_bytes(data[0..=1].try_into().unwrap()),
        height: i16::from_le_bytes(data[2..=3].try_into().unwrap()),
        left_offset: i16::from_le_bytes(data[4..=5].try_into().unwrap()),
        top_offset: i16::from_le_bytes(data[6..=7].try_into().unwrap()),
        column_ofs: {
            let mut array: [i32; SCREEN_WIDTH] = [0; SCREEN_WIDTH];

            let start_byte: &[u8] = &data[8..];

            for i in 0..SCREEN_WIDTH {
                let start = i * 4;
                let end = start + 4;
                let chunk = &start_byte[start..end];

                array[i] = i32::from_le_bytes(chunk.try_into().unwrap());
            }
            array
        },
    };

    for i in 0..patch.column_ofs.len() {
        let column_ofs: usize = patch.column_ofs[i] as usize;
        let mut column: Column = Column {
            top_delta: data[column_ofs],
            length: data[column_ofs + 1],
        };
        println!(
            "Column {}: Top delta{}, length{}",
            i, column.top_delta, column.length
        );

        while column.top_delta != 0xff {
            let source: &[u8] = &data[column_ofs + 3..];
            let count: usize = column.length as usize;
            let mut source_index: usize = 0;
            let mut dest_index: usize = (column.top_delta as usize) * SCREEN_WIDTH;

            for j in (0..count).rev() {
                screen[dest_index] = source[source_index];
                source_index += 1;
                dest_index += SCREEN_WIDTH;
                println!("{}", j);
            }

            column = Column {
                top_delta: data[column_ofs + (column.length as usize) + 4],
                length: data[column_ofs + (column.length as usize) + 4 + 1],
            };
            println!("test");
        }
    }
    println!("test");
}
pub unsafe fn sdl_test(mut data: Vec<u8>) {
    let sdl_context: Sdl = sdl2::init().unwrap();
    let sdl_video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();
    let mut sdl_window = sdl_video_subsystem
        .window("test", 320, 300)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = sdl_window
        .into_canvas()
        .target_texture()
        .software()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let rect: Rect = Rect::from(SDL_Rect {
        x: 0,
        y: 0,
        w: 320,
        h: 300,
    });

    let texture_creator = canvas.texture_creator();
    let mut texture: Texture =
        Surface::from_data(&mut data, 320, 300, 320 * 4, PixelFormatEnum::ARGB32)
            .unwrap()
            .as_texture(&texture_creator)
            .unwrap();

    'running: loop {
        canvas.copy(&texture, None, rect).unwrap();
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
