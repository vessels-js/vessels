pub trait Renderer {
    fn new() -> Self;
    fn run(&self);
}

pub trait Frame {
    fn resize(&mut self, size: Size);
    fn clip(&mut self, start: Option<Point>, end: Option<Point>);
    fn position(&mut self, position: Point);
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Size {
    pub w: i32,
    pub h: i32,
}
