pub struct Cam {
    x: f64,
    y: f64,

    scale: f64,

}


impl Cam {
    pub fn new(x: f64, y: f64, scale: f64) -> Self {
        Cam { x: x, y: y, scale: scale }
    }

    #[inline]
    pub fn get_x(&self) -> f64 {
        self.x
    }

    #[inline]
    pub fn get_y(&self) -> f64 {
        self.y
    }

    #[inline]
    pub fn get_scale(&self) -> f64 {
        self.scale
    }

    #[inline]
    pub fn translate_x(&self, x: f64) -> f64 {
        x + self.x
    }

    #[inline]
    pub fn translate_y(&self, y: f64) -> f64 {
        y + self.y
    }

    #[inline]
    pub fn translate(&self, x: f64, y: f64) -> (f64, f64) {
        (self.translate_x(x), self.translate_y(y))
    }

    #[inline]
    pub fn translate_inv(&self, x: f64, y: f64) -> (f64, f64) {
        (x - self.x, y - self.y)
    }

    #[inline]
    pub fn scale(&self, width: f64, height: f64) -> (f64, f64) {
        (self.scale * width, self.scale * height)
    }

    #[inline]
    pub fn scale_inv(&self, width: f64, height: f64) -> (f64, f64) {
        ((1.0 / self.scale) * width, (1.0 / self.scale) * height)
    }

    #[inline]
    pub fn zoom_out(&mut self, k: f64) {
        self.scale -= k;
    }

    #[inline]
    pub fn zoom_in(&mut self, k: f64) {
        self.scale += k;
    }

    #[inline]
    pub fn move_right(&mut self, offset: f64) {
        self.x -= offset;
    }

    #[inline]
    pub fn move_left(&mut self, offset: f64) {
        self.x += offset;
    }

    #[inline]
    pub fn move_up(&mut self, offset: f64) {
        self.y += offset;
    }

    #[inline]
    pub fn move_down(&mut self, offset: f64) {
        self.y -= offset;
    }
}
