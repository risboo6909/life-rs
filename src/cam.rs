pub struct Cam {

    pub x: f64,
    pub y: f64,

    pub scale: f64,

}


impl Cam {

    pub fn new(x: f64, y: f64, scale: f64) -> Self {
        Cam { x: x, y: y, scale: scale }
    }

    pub fn translate_x(&self, x: f64) -> f64 {
        x + self.x
    }

    pub fn translate_y(&self, y: f64) -> f64 {
        y + self.y
    }

    pub fn translate(&self, x: f64, y: f64) -> (f64, f64) {
        (self.translate_x(x), self.translate_y(y))
    }

    pub fn translate_inv(&self, x: f64, y: f64) -> (f64, f64) {
        (x - self.x, y - self.y)
    }

    pub fn scale(&self, width: f64, height: f64) -> (f64, f64) {
        (self.scale * width, self.scale * height)
    }

    pub fn scale_inv(&self, width: f64, height: f64) -> (f64, f64) {
        ((1.0/self.scale) * width, (1.0/self.scale) * height)
    }

    pub fn zoom_out(&mut self, k: f64) {
        self.scale -= k;
    }

    pub fn zoom_in(&mut self, k: f64) {
        self.scale += k;
    }

    pub fn move_right(&mut self, offset: f64) {
        self.x -= offset;
    }

    pub fn move_left(&mut self, offset: f64) {
        self.x += offset;
    }

    pub fn move_up(&mut self, offset: f64) {
        self.y += offset;
    }

    pub fn move_down(&mut self, offset: f64) {
        self.y -= offset;
    }

}
