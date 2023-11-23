use std::ops::{Add, Sub, Mul, Div, Rem, AddAssign, MulAssign, SubAssign, DivAssign, RemAssign};

#[derive(Debug, Default)]
pub struct Point2D {
    pub x: f64,
    pub y: f64
}

// ----------
// STD Traits
// ----------

impl Add for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Self) -> Self::Output {
        Self {x: (self.x + rhs.x), y: (self.y + rhs.y) }
    }
}

impl Sub for Point2D {
    type Output = Point2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {x: (self.x + rhs.x), y: (self.y + rhs.y)}
    }
}

impl Mul for Point2D {
    type Output = Point2D;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {x: (self.x * rhs.x), y: (self.y + rhs.y)}
    }
}

impl Div for Point2D {
    type Output = Point2D;

    fn div(self, rhs: Self) -> Self::Output {
        Self {x: (self.x / rhs.x), y: (self.y / rhs.y)}
    }
}

impl Rem for Point2D {
    type Output = Point2D;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {x: (self.x % rhs.x), y: (self.x % rhs.x)}
    }
}


impl AddAssign for Point2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}


impl SubAssign for Point2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl MulAssign for Point2D {
    fn mul_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

impl DivAssign for Point2D {
    fn div_assign(&mut self, rhs: Self) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl RemAssign for Point2D {
    fn rem_assign(&mut self, rhs: Self) {
        self.x = self.x % rhs.x;
        self.y = self.y % rhs.y;
    }
}

// -------------
// CUSTOM TRAITS
// -------------

pub trait Reflect {
    fn reflect_x(&mut self) -> ();
    fn reflect_y(&mut self) -> ();
}

impl Reflect for Point2D {
    fn reflect_x(&mut self) -> () { self.x *= -1.0; }
    fn reflect_y(&mut self) -> () { self.y *= -1.0; }
}

pub trait Translate {
    fn translate_x(&mut self, distance: f64) -> ();
    fn translate_y(&mut self, distance: f64) -> ();
    fn tarnslate(&mut self, x_distance: f64, y_distance: f64) -> ();
}

impl Translate for Point2D {
    fn translate_x(&mut self, distance: f64) -> () { self.x += distance; }
    fn translate_y(&mut self, distance: f64) -> () { self.y += distance; }
    fn tarnslate(&mut self, x_distance: f64, y_distance: f64) -> () {
        self.x += x_distance;
        self.y += y_distance;
    }
}