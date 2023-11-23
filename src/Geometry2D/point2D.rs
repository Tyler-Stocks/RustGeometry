use std::ops::{Add, Sub, Mul, Div, AddAssign};

#[derive(Debug, Default)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T
}

pub trait Mirror {
    fn mirror_x(&mut self) -> ();
    fn mirror_y(&mut self) -> ();
}

impl Mirror for Point2D<i128> {
    fn mirror_x(&mut self) -> () { self.x *= -1; }
    fn mirror_y(&mut self) -> () { self.y *= -1; }
}

impl Mirror for Point2D<f64> {
    fn mirror_x(&mut self) -> () { self.x *= -1.0; }
    fn mirror_y(&mut self) -> () { self.y *= -1.0; }
}

pub trait Translate<T> {
    fn translate_x(&mut self, distance: T) -> ();
    fn translate_y(&mut self, distance: T) -> ();
    fn tarnslate(&mut self, x_distance: T, y_distance: T) -> ();
}

impl Translate<i128> for Point2D<i128> {
    fn translate_x(&mut self, distance: i128) -> () { self.x += distance; }
    fn translate_y(&mut self, distance: i128) -> () { self.y += distance; }
    fn tarnslate(&mut self, x_distance: i128, y_distance: i128) -> () {
        self.x += x_distance;
        self.y += y_distance;
    }
}

impl Translate<f64> for Point2D<f64> {
    fn translate_x(&mut self, distance: f64) -> () { self.x += distance; }
    fn translate_y(&mut self, distance: f64) -> () { self.y += distance; }
    fn tarnslate(&mut self, x_distance: f64, y_distance: f64) -> () {
        self.x += x_distance;
        self.y += y_distance;
    }
}

impl Add for Point2D<f64> {
    type Output = Point2D<f64>;

    fn add(self, rhs: Self) -> Self::Output {
        Point2D {x: (self.x + rhs.x), y: (self.y + rhs.y) }
    }
}

impl Add for Point2D<i128> {
    type Output = Point2D<i128>;

    fn add(self, rhs: Self) -> Self::Output {
        Point2D {x: (self.x + rhs.x), y: (self.y + rhs.y)}
    }
}

impl Sub for Point2D<f64> {
    type Output = Point2D<f64>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2D {x: (self.x + rhs.x), y: (self.y + rhs.y)}
    }
}

impl Sub for Point2D<i128> {
    type Output = Point2D<i128>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2D {x: (self.x + rhs.x), y: (self.y + rhs.y)}
    }
}

impl Mul for Point2D<f64> {
    type Output = Point2D<f64>;

    fn mul(self, rhs: Self) -> Self::Output {
        Point2D {x: (self.x * rhs.x), y: (self.y + rhs.y)}
    }
}

impl Mul for Point2D<i128> {
    type Output = Point2D<i128>;

    fn mul(self, rhs: Self) -> Self::Output {
        Point2D {x: (self.x * rhs.x), y: (self.y * rhs.y)}
    }
}

impl Div for Point2D<f64> {
    type Output = Point2D<f64>;

    fn div(self, rhs: Self) -> Self::Output {
        Point2D {x: (self.x / rhs.x), y: (self.y / rhs.y)}
    }
}

impl Div for Point2D<i128> {
    type Output = Point2D<i128>;

    fn div(self, rhs: Self) -> Self::Output {
        Point2D {x: (self.x / rhs.x), y: (self.y / rhs.y)}
    }
}


pub trait Reflect {
    fn reflect_x(&mut self) -> ();
    fn reflect_Y((&mut self))
}