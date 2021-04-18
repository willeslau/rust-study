use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    radius: f64,
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

pub struct Square {
    length: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.length * self.length
    }
}

impl Square {
    pub fn new(length: f64) -> Square {
        Square{ length }
    }
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle{ width, height }
    }
}
impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle{ radius }
    }
}