use static_assertions::assert_not_impl_all;
use std::marker::PhantomData;

use crate::{bind, Video};

pub struct ScreenSaver<'video> {
    _phantom: PhantomData<&'video Video<'video>>,
}

impl std::fmt::Debug for ScreenSaver<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScreenSaver").finish()
    }
}

assert_not_impl_all!(ScreenSaver: Send, Sync);

impl<'video> ScreenSaver<'video> {
    pub fn new(_: &'video Video) -> Self {
        unsafe { bind::SDL_EnableScreenSaver() }
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<'video> Drop for ScreenSaver<'video> {
    fn drop(&mut self) {
        unsafe { bind::SDL_DisableScreenSaver() }
    }
}
