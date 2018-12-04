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
    pub points: Points,
    pub segs: Segs,
    pub rays: Segs,
    stepping: bool,
    halt: u8,
}

impl Output {
    pub fn new() -> Self {
        Output { points: Points::new(), segs: Segs::new(), rays: Segs::new(), stepping: false, halt: 0 }
    }

    pub fn stepping(&self) -> bool {
        self.stepping
    }

    pub fn halt(&self) -> *const u8 {
        &self.halt
    }
}
