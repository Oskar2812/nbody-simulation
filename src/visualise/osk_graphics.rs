#![allow(dead_code)]

use std::{ffi::CString, os::raw::{c_char, c_int}};

#[repr(C)]
struct OskWindow {
    _private: [u8; 0],
}

#[repr(C)]
struct OskColour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

unsafe extern "C" {
    fn OpenWindow(width: u32, height: u32, title: *const c_char, renderer_type: c_int) -> *mut OskWindow;
    fn PollEvents() -> c_int;
    fn BeginFrame(window: *mut OskWindow) -> c_int;
    fn EndFrame(window: *mut OskWindow) -> c_int;
    fn SetBackground(window: *mut OskWindow, colour: OskColour) -> c_int;
    fn DrawTriangle(window: *mut OskWindow, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, colour: OskColour) -> c_int;
    fn DrawQuad(window: *mut OskWindow, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: OskColour) -> c_int;
    fn DrawCircle(window: *mut OskWindow, cx: f32, cy: f32, radius: f32, segments: c_int, colour: OskColour) -> c_int;
    fn DrawLine(window: *mut OskWindow, x0: f32, y0: f32, x1: f32, y1: f32, thickness: f32, colour: OskColour) -> c_int;
}

pub struct Window {
    handle: *mut OskWindow,
    pub width: u32,
    pub height: u32,
    pub is_open: bool
}

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Window {
    pub fn open(width: u32, height: u32, title: &str) -> Option<Window> {
        let c_title: CString = CString::new(title).ok()?;

        let ptr: *mut OskWindow = unsafe { OpenWindow(width, height, c_title.as_ptr(), 0)};
        if ptr.is_null() {
            None
        } else {
            Some(Window { 
                handle: ptr,
                width,
                height,
                is_open: true
            })
        }
    }

    pub fn poll_events(&mut self) -> bool {
        let success = unsafe { PollEvents() != 0};

        self.is_open = success;
        success
    }

    pub fn begin_frame(&self) -> bool {
        unsafe { BeginFrame(self.handle) != -1 }
    }

    pub fn end_frame(&self) -> bool {
        unsafe { EndFrame(self.handle) != -1  }
    }

    pub fn set_background(&self, colour: Colour) -> bool {
        let osk_colour: OskColour = OskColour {
            r: colour.r,
            g: colour.g,
            b: colour.b,
            a: colour.a,
        };
        unsafe { SetBackground(self.handle, osk_colour) != -1 }
    }

    pub fn draw_triangle(&self, p0: Point, p1: Point, p2: Point, colour: Colour) -> bool {
        let osk_colour = OskColour {
            r: colour.r,
            g: colour.g,
            b: colour.b,
            a: colour.a,
        };

        unsafe { DrawTriangle(self.handle, p0.x, p0.y, p1.x, p1.y, p2.x, p2.y, osk_colour) != -1 }
    }

    pub fn draw_quad(&self, p0: Point, p1: Point, p2: Point, p3: Point, colour: Colour) -> bool {
        let osk_colour = OskColour {
            r: colour.r,
            g: colour.g,
            b: colour.b,
            a: colour.a,
        };

        unsafe {
            DrawQuad(
                self.handle,
                p0.x, p0.y,
                p1.x, p1.y,
                p2.x, p2.y,
                p3.x, p3.y,
                osk_colour,
            ) != -1
        }
    }

    pub fn draw_circle(&self, center: Point, radius: f32, segments: i32, colour: Colour) -> bool {
        let osk_colour = OskColour {
            r: colour.r,
            g: colour.g,
            b: colour.b,
            a: colour.a,
        };

        unsafe {
            DrawCircle(self.handle, center.x, center.y, radius, segments, osk_colour) != -1
        }
    }

    pub fn draw_line(&self, start: Point, end: Point, thickness: f32, colour: Colour) -> bool {
        let osk_colour = OskColour {
            r: colour.r,
            g: colour.g,
            b: colour.b,
            a: colour.a,
        };

        unsafe {
            DrawLine(self.handle, start.x, start.y, end.x, end.y, thickness, osk_colour) != -1
        }
    }
}

impl Colour {
    pub const RED: Colour = Colour { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Colour = Colour { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Colour = Colour { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const BLACK: Colour = Colour { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const WHITE: Colour = Colour { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Colour = Colour { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
}