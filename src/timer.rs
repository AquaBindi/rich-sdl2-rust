use std::ops;

use crate::{bind, Sdl};

mod ticks;

pub use ticks::*;

pub fn delay(ms: u32) {
    unsafe { bind::SDL_Delay(ms) }
}

pub mod performance {
    use crate::bind;

    pub fn counter() -> u64 {
        unsafe { bind::SDL_GetPerformanceCounter() }
    }

    pub fn frequency() -> u64 {
        unsafe { bind::SDL_GetPerformanceFrequency() }
    }
}
