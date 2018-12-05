use std::cell::RefCell;

pub struct Points {
    x: Vec<i32>,
    y: Vec<i32>,
}

impl Points {
    pub fn new() -> Self {
        Points { x: Vec::new(), y: Vec::new() }
    }

    pub fn add(&mut self, x: i32, y: i32) {
        self.x.push(x);
        self.y.push(y);
    }

    pub fn len(&self) -> u32 {
        self.x.len() as u32
    }

    pub fn x(&self) -> *const i32 {
        self.x.as_ptr()
    }

    pub fn y(&self) -> *const i32 {
        self.y.as_ptr()
    }

    pub fn condense(&self) -> Vec<(i32, i32)> {
        self.x.iter().cloned().zip(self.y.iter().cloned()).collect()
    }
}

pub struct Segs {
    x: Vec<i32>,
    y: Vec<i32>,
}

impl Segs {
    pub fn new() -> Self {
        Segs { x: Vec::new(), y: Vec::new() }
    }

    pub fn add(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.x.push(x1);
        self.x.push(x2);
        self.y.push(y1);
        self.y.push(y2);
    }

    pub fn pop(&mut self) -> ((i32, i32), (i32, i32)) {
        ((self.x.pop().unwrap(), self.x.pop().unwrap()), (self.y.pop().unwrap(), self.y.pop().unwrap()))
    }

    pub fn len(&self) -> u32 {
        (self.x.len() / 2) as u32
    }

    pub fn x(&self) -> *const i32 {
        self.x.as_ptr()
    }

    pub fn y(&self) -> *const i32 {
        self.y.as_ptr()
    }
}

pub struct Output {
    points: Points,
    segs: Segs,
    rays: Segs,
    stepping: bool,
    halt: RefCell<u8>,
}

impl Output {
    pub fn new() -> Self {
        Output { points: Points::new(), segs: Segs::new(), rays: Segs::new(), stepping: true, halt: RefCell::new(0) }
    }

    pub fn halt(&self) -> *const u8 {
        &*self.halt.borrow()
    }

    pub fn points_add(&mut self, x: i32, y: i32) {
        self.points.add(x, y);
        self.spin();
    }

    pub fn points_len(&self) -> u32 {
        self.points.len()
    }

    pub fn points_x(&self) -> *const i32 {
        self.points.x()
    }

    pub fn points_y(&self) -> *const i32 {
        self.points.y()
    }

    pub fn points_condense(&self) -> Vec<(i32, i32)> {
        self.points.condense()
    }

    pub fn segs_add(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let ret = self.segs.add(x1, y1, x2, y2);
        self.spin();
        ret
    }

    pub fn segs_pop(&mut self) -> ((i32, i32), (i32, i32)) {
        let ret = self.segs.pop();
        self.spin();
        ret
    }

    pub fn segs_len(&self) -> u32 {
        self.segs.len()
    }

    pub fn segs_x(&self) -> *const i32 {
        self.segs.x()
    }

    pub fn segs_y(&self) -> *const i32 {
        self.segs.y()
    }

    pub fn rays_add(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let ret = self.rays.add(x1, y1, x2, y2);
        self.spin();
        ret
    }

    pub fn rays_pop(&mut self) -> ((i32, i32), (i32, i32)) {
        let ret = self.rays.pop();
        self.spin();
        ret
    }

    pub fn rays_len(&self) -> u32 {
        self.rays.len()
    }

    pub fn rays_x(&self) -> *const i32 {
        self.rays.x()
    }

    pub fn rays_y(&self) -> *const i32 {
        self.rays.y()
    }

    fn spin(&self) {
        if self.stepping {
            while *self.halt.borrow() == 0 {
            }
        }
    }
}
