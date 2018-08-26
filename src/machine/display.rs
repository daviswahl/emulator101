//! Basic hello world example.
use crossbeam_channel::Receiver;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::path;

pub const FB_SIZE: usize = 0x4000 - 0x2400;
type DisplayBuf = [u8; 4 * 224 * 256];
// First we make a structure to contain the game's state
struct Display {
    frames: usize,
    buf: DisplayBuf,
    receiver: Receiver<[u8; FB_SIZE]>,
}

impl Display {
    fn new(_ctx: &mut Context, receiver: Receiver<[u8; FB_SIZE]>) -> GameResult<Display> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let buf = [0; 4 * 224 * 256];
        let s = Display {
            frames: 0,
            buf,
            receiver,
        };
        Ok(s)
    }

    fn update_buf(&mut self, fb: [u8; FB_SIZE]) {
        update_buf(&mut self.buf, fb)
    }
}

fn update_buf(buf: &mut DisplayBuf, fb: [u8; FB_SIZE]) {
    for i in 0..224 {
        let mut j = 0;
        while j < 256 {
            let pixel_offset = (i * (256 / 8)) + j / 8;
            let pixel = fb[pixel_offset];

            let mut offset = (255 - j) * (224 * 4) + (i * 4);
            for p in 0..8 {
                let p1 = if 0 != (pixel & (1 << p)) { 0xff } else { 0x00 };

                buf[offset] = p1;
                buf[offset + 1] = p1;
                buf[offset + 2] = p1;
                buf[offset + 3] = 0xff;

                offset = offset.wrapping_sub(224 * 4)
            }
            j += 8;
        }
    }
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl event::EventHandler for Display {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        if let Some(buf) = self.receiver.try_recv() {
            self.update_buf(buf);
        }
        let image = graphics::Image::from_rgba8(ctx, 224, 256, &self.buf)?;

        let dest_point = graphics::Point2::new(0.0, 0.0);
        graphics::draw(ctx, &image, dest_point, 0.0)?;

        // Drawables are drawn from their top-left corner.
        graphics::present(ctx);

        self.frames += 1;
        if (self.frames % 100) == 0 {
            //println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }
}

// Now our main function, which does three things:
//
// * First, create a new `ggez::conf::Conf`
// object which contains configuration info on things such
// as screen resolution and window title.
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game.
// * Then, just call `game.run()` which runs the `Game` mainloop.
use crate::machine::display;
pub fn run(recv: Receiver<[u8; display::FB_SIZE]>) -> GameResult<()> {
    let mut c = conf::Conf::new();
    c.window_mode.width = 224;
    c.window_mode.height = 256;

    let ctx = &mut Context::load_from_conf("helloworld", "ggez", c)?;
    // We add the CARGO_MANIFEST_DIR/resources to the filesystem's path
    // so that ggez will look in our cargo project directory for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut Display::new(ctx, recv)?;
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
    Ok(())
}
