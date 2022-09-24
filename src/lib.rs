// This is some boilerplate code to get you started.
// We recommend taking a look at "game.rs" and building from there.

use std::mem::MaybeUninit;

mod game;
use game::MyGame;

pub trait Game {
    const FPS: i32;
    const FPS_USIZE: usize;
    const JUMP_ARRAY: [i32; 100]; //number here is jump frames, cannot use Self in current version of rust
    fn fall(&mut self);
    fn jump(&mut self);
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
