pub struct Cam {
    x: f64,
    y: f64,

    scale: f64,

    zoom_step: f64,
    default_move_step: f64,
    move_step: f64,
    move_acc: f64
}


impl Cam {
    pub fn new(x: f64, y: f64) -> Self {
        Cam {
            x: x, y: y, scale: 1.0,
            zoom_step: 0.1,
            default_move_step: 1.0,
            move_step: 1.0,
            move_acc: 1.4
        }
    }

    pub fn zoom_step(mut self, zoom_step: f64) -> Self {
        self.zoom_step = zoom_step;
        self
    }

    pub fn move_step(mut self, move_step: f64) -> Self {
        self.move_step = move_step;
        self.default_move_step = move_step;
        self
    }

    pub fn move_acc(mut self, move_acc: f64) -> Self {
        self.move_acc = move_acc;
        self
    }

    pub fn reset_move_step(&mut self) {
        self.move_step = self.default_move_step
    }

    pub fn get_move_step(&self) -> f64 {
        self.move_step
    }

    pub fn get_zoom_step(&self) -> f64 {
        self.zoom_step
    }

    pub fn get_move_acc(&self) -> f64 {
        self.move_acc
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_scale(&self) -> f64 {
        self.scale
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
        ((1.0 / self.scale) * width, (1.0 / self.scale) * height)
    }

    pub fn zoom_out(&mut self) {
        self.scale -= self.get_zoom_step();
    }

    pub fn zoom_in(&mut self) {
        self.scale += self.get_zoom_step();
    }

    pub fn move_right(&mut self) {
        self.x -= self.get_move_step();
        self.move_step *= self.move_acc;
    }

    pub fn move_left(&mut self) {
        self.x += self.get_move_step();
        self.move_step *= self.move_acc;
    }

    pub fn move_up(&mut self) {
        self.y += self.get_move_step();
        self.move_step *= self.move_acc;
    }

    pub fn move_down(&mut self) {
        self.y -= self.get_move_step();
        self.move_step *= self.move_acc;
    }
}
