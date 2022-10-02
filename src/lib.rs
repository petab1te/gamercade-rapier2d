// This is some boilerplate code to get you started.
// We recommend taking a look at "game.rs" and building from there.

use std::mem::MaybeUninit;
const JUMP_FRAMES: usize = 120;
mod game;
use game::MyGame;

pub trait Game {
    const FPS: i32;
    const FPS_USIZE: usize;
    const JUMP_ARRAY: [i32; JUMP_FRAMES]; //number here is jump frames, cannot use Self in current version of rust
    const GRAVITY: f32;
    fn init() -> Self;
    fn update(&mut self);
    fn draw(&self);
}

static mut GAME: MaybeUninit<MyGame> = MaybeUninit::uninit();

#[no_mangle]
pub extern "C" fn init() {
    unsafe {
        GAME.write(MyGame::init());
    }
}

#[no_mangle]
pub extern "C" fn update() {
    unsafe {
        GAME.assume_init_mut().update();
    }
}

#[no_mangle]
pub extern "C" fn draw() {
    unsafe {
        GAME.assume_init_ref().draw();
    }
}
