//! Provides functionalities for constructing billiard simulations

pub struct Billiard<T: Contour> {
    ball: Ball,
    table: Table<T>
}

pub struct Ball {
    location: Point,
    speed: f64,
    direction: Point
}

impl Ball {
    pub fn new(location: (f64, f64), speed: f64, direction: (f64, f64)) -> Ball {
        Ball {location: Point::new(location), speed, direction: Point::new(direction)}
    }
    pub fn movement(&self) -> impl Fn(f64) -> (f64, f64) {
        |t| (self.location.x + self.speed * self.direction.x * t, self.location.y + self.speed * self.direction.y * t)
    }
}

pub struct Table<T: Contour> {
    contours: [T]
}

pub trait Contour {
    fn contour(&self) -> impl Fn(f64) -> (f64, f64);
}

pub struct Line {
    start: Point,
    end: Point
}

impl Line {
    pub fn new(point_1: (f64, f64), point_2: (f64, f64)) -> Line {
        Line {start: Point::new(point_1), end: Point::new(point_2)}
    }
}

impl Contour for Line {
    fn contour(&self) -> impl Fn(f64) -> (f64, f64) {
        |t| (self.start.x + (self.end.x - self.start.x) * t, self.start.y + (self.end.y - self.start.y) * t)
    }
}

struct Point {
    x: f64,
    y: f64
}

impl Point {
    pub fn new((x,y): (f64, f64)) -> Point {
        Point {x, y}
    }
}