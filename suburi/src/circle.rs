extern crate sdl2;
use sdl2::gfx::primitives::DrawRenderer;
use std;

pub enum Direction {
    east = 0,
    south = 90,
    west = 180,
    north = 270
}

pub struct CirclePosition {
    pub x: i16,
    pub y: i16,
    pub radius: i16,
    pub direction: i16,
    pub color: sdl2::pixels::Color,
    pub isOpeningMouth: bool
}

impl CirclePosition {
    pub fn movePosition(&self, renderer: &sdl2::render::Canvas<sdl2::video::Window>) -> () {
        renderer.filled_pie(self.x, self.y, 20,
                            self.radius + self.direction, self.direction, self.color);
    }

    pub fn isFullOpenMouth(&mut self) -> bool {
        return 80 <= self.radius;
    }

    pub fn isClosedMouth(&mut self) -> bool {
        return self.radius <= 10;
    }

    pub fn moveMouth(&mut self) -> () {
        if self.isOpeningMouth {
            self.radius += 10;
        } else {
            self.radius -= 10;
        }

        if self.isFullOpenMouth() || self.isClosedMouth() {
            self.isOpeningMouth = !self.isOpeningMouth;
        }
    }
}