#![allow(dead_code)]
use math::vectors::Vec2;

/// Circle structure, with center and radius
pub struct Circle {
    pub center: Vec2<f32>,
    pub radius: f32,
}

/// Rectangle structure
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

impl Rect {
    /// Gets the top left coordinate of the rectangle
    pub fn top_left(&self) -> Vec2<f32> {
        Vec2::new(self.top(), self.left())
    }

    /// Gets the top right coordinate of the rectangle
    pub fn top_right(&self) -> Vec2<f32> {
        Vec2::  new(self.x + self.width, self.y)
    }

    /// Gets the bottom left coordinate of the rectangle
    pub fn bottom_left(&self) -> Vec2<f32> {
        Vec2::new(self.x, self.y + self.height)
    }

    /// Gets the bottom right coordinate of the rectangle
    pub fn bottom_right(&self) -> Vec2<f32> {
        Vec2::new(self.x + self.width, self.y + self.height)
    }

    /// Gets the center of the rectangle
    pub fn center(&self) -> Vec2<f32> {
        Vec2::new(self.x + self.width/2.0, self.y + self.height/2.0)
    }

    /// Gets the x value of the left of the rectangle
    pub fn left(&self) -> f32 {
        self.x
    }

    /// Gets the x value of the right of the rectangle
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    /// Gets the y value of the top of the rectangle
    pub fn top(&self) -> f32 {
        self.y
    }

    /// Gets the y value of the bottom of the rectangle
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    /// Moves the rectangle using a vector
    /// # Arguments
    /// `vec` - the vector to move by
    pub fn move_vec(&mut self, vec: Vec2<f32>) {
        self.x += vec.x;
        self.y += vec.y;
    }

    /// Calculate the intersection area of two rectangles
    /// # Arguments
    /// `other` - the rectangle to calculate the intersection with
    pub fn intersect_area(&self, other: &Rect) -> f32 {
        // !!! FIXME: Change to use fmin/fmax once they are working in Rust again
        let x_intersect =
            if self.right() < other.right() { self.right() } else { other.right() } -
            if self.left() > other.left() { self.left() } else { other.left() };

        let y_intersect =
            if self.bottom() < other.bottom() { self.bottom() } else { other.bottom() } -
            if self.top() > other.top() { self.top() } else { other.top() };

        if x_intersect < 0.0 || y_intersect < 0.0 {
            0.0
        }
        else {
            x_intersect * y_intersect
        }
    }
}