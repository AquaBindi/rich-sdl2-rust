use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::geo::Point;
use crate::surface::Surface;
use crate::{bind, Sdl, SdlError};

use super::Window;

pub enum SystemCursorKind {
    Arrow,
    IBeam,
    Wait,
    Crosshair,
    WaitArrow,
    SizeNwse,
    SizeNesw,
    SizeWe,
    SizeNs,
    SizeAll,
    No,
    Hand,
}

impl SystemCursorKind {
    pub(crate) fn as_raw(&self) -> bind::SDL_SystemCursor {
        use SystemCursorKind::*;
        match self {
            Arrow => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_ARROW,
            IBeam => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_IBEAM,
            Wait => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_WAIT,
            Crosshair => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_CROSSHAIR,
            WaitArrow => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_WAITARROW,
            SizeNwse => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENWSE,
            SizeNesw => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENESW,
            SizeWe => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZEWE,
            SizeNs => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENS,
            SizeAll => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZEALL,
            No => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_NO,
            Hand => bind::SDL_SystemCursor_SDL_SYSTEM_CURSOR_HAND,
        }
    }
}

pub struct Cursor<'window> {
    cursor: NonNull<bind::SDL_Cursor>,
    window: PhantomData<&'window ()>,
}

impl<'window> Cursor<'window> {
    pub fn system(_: &'window Window, kind: SystemCursorKind) -> Result<Self, SdlError> {
        let cursor = unsafe { bind::SDL_CreateSystemCursor(kind.as_raw()) };
        let cursor = NonNull::new(cursor).ok_or_else(|| SdlError::UnsupportedFeature)?;
        Ok(Self {
            cursor,
            window: PhantomData,
        })
    }

    pub fn colored(
        _: &'window Window,
        surface: &impl Surface,
        hot_spot: Point,
    ) -> Result<Self, SdlError> {
        let cursor = unsafe {
            bind::SDL_CreateColorCursor(surface.as_ptr().as_ptr(), hot_spot.x, hot_spot.y)
        };
        let cursor = NonNull::new(cursor).ok_or_else(|| SdlError::Others { msg: Sdl::error() })?;
        Ok(Self {
            cursor,
            window: PhantomData,
        })
    }
}

impl Drop for Cursor<'_> {
    fn drop(&mut self) {
        unsafe { bind::SDL_FreeCursor(self.cursor.as_ptr()) }
    }
}
