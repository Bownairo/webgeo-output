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

    pub fn pop(&mut self) {
        self.x.pop();
        self.y.pop();
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

    pub fn pop(&mut self) {
        self.x.pop();
        self.x.pop();
        self.y.pop();
        self.y.pop();
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

enum Event {
    PushPoint{x: i32, y: i32},
    PopPoint,
    PushSeg{x1: i32, y1: i32, x2: i32, y2: i32},
    PopSeg,
    PushRay{x1: i32, y1: i32, x2: i32, y2: i32},
    PopRay,
}

struct EventQueue(Vec<Event>);

impl EventQueue {
    fn new() -> Self {
        EventQueue (Vec::new())
    }

    fn push_point(&mut self, x: i32, y: i32) {
        self.0.push(Event::PushPoint { x, y });
    }

    fn pop_point(&mut self) {
        self.0.push(Event::PopPoint);
    }

    fn push_seg(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.0.push(Event::PushSeg { x1, y1, x2, y2 });
    }

    fn pop_seg(&mut self) {
        self.0.push(Event::PopSeg);
    }

    fn push_ray(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.0.push(Event::PushRay { x1, y1, x2, y2 });
    }

    fn pop_ray(&mut self) {
        self.0.push(Event::PopRay);
    }
}

pub struct Output {
    points: Points,
    segs: Segs,
    rays: Segs,
    events: EventQueue,
    pointer: usize,
}

impl Output {
    pub fn new() -> Self {
        Output { points: Points::new(), segs: Segs::new(), rays: Segs::new(), events: EventQueue::new(), pointer: 0 }
    }

    pub fn points_add(&mut self, x: i32, y: i32) {
        self.events.push_point(x, y);
    }

    pub fn points_pop(&mut self) {
        self.events.pop_point();
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

    pub fn segs_add(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.events.push_seg(x1, y1, x2, y2);
    }

    pub fn segs_pop(&mut self) {
        self.events.pop_seg();
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
        self.events.push_ray(x1, y1, x2, y2);
    }

    pub fn rays_pop(&mut self) {
        self.events.pop_ray();
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

    pub fn step(&mut self) {
        match self.events.0.get(self.pointer) {
            Some(Event::PushPoint{x, y}) => self.points.add(*x, *y),
            Some(Event::PopPoint) => self.points.pop(),
            Some(Event::PushSeg{x1, y1, x2, y2}) => self.segs.add(*x1, *y1, *x2, *y2),
            Some(Event::PopSeg) => self.segs.pop(),
            Some(Event::PushRay{x1, y1, x2, y2}) => self.rays.add(*x1, *y1, *x2, *y2),
            Some(Event::PopRay) => self.rays.pop(),
            _ => return,
        }

        self.pointer += 1;
    }

    pub fn complete(&mut self) {
        while !self.done() {
            self.step();
        }
    }

    pub fn done(&mut self) -> bool {
        self.pointer == self.events.0.len()
    }
}
