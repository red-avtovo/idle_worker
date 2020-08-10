use rand::Rng;
use rand::distributions::Uniform;
use autopilot::mouse;
use autopilot::geometry::Point;
use std::cell::Cell;
use std::thread::sleep;
use std::time::Duration;
use std::thread;
use std::io;

struct MouseWave {
    cursor: Cell<Point>,
    scoped_height: i64,
    scoped_width: i64
}

impl MouseWave {
    pub fn new() -> Self {
        MouseWave {
            cursor: Cell::new(mouse::location()),
            scoped_height: 7,
            scoped_width: 20
        }
    }

    pub fn start(self: &Self) {
        loop {
            if self.cursor.get() != mouse::location() {
                self.cursor.set(mouse::location());
                sleep(Duration::from_secs(5));
            } else {
                self.start_wobble();
            }
        }
    }

    fn start_wobble(self: &Self) {
        let x_distribution = Uniform::new(-self.scoped_width,self.scoped_width);
        let mut rng = rand::thread_rng();
        let mut last_x = 0;
        loop {
            if !self.in_cage() { return } 
            let x: i64 = rng.sample_iter(x_distribution).filter(|x| *x != last_x).next().unwrap();
            let y: i64 = rng.gen_range(-self.scoped_height, self.scoped_height);
            last_x = x;
            // println!("X:{}",x);
            let duration = rng.gen_range(1, 3);
            self.move_to(&x,&y);
            sleep(Duration::from_millis(duration*100));
        }
    }
    
    fn in_cage(self: &Self) -> bool {
        let new_cursor = mouse::location();
        if (new_cursor.clone().x - self.cursor.get().x).abs() > self.scoped_width as f64 { return false } 
        if (new_cursor.clone().y - self.cursor.get().y).abs() > self.scoped_height as f64 { return false }
        true
    }
    
    fn move_to(self: &Self, x: &i64, y: &i64) {
        let new_cursor = mouse::location();
        let new_position = Point::new(
            self.cursor.get().x + x.clone() as f64,
            self.cursor.get().y + y.clone() as f64
        );
        let dots: Vec<Point> = self.gen_seq(new_cursor, new_position);
        for p in dots {
            if !self.in_cage() { return }
            match mouse::move_to(p) {
                Err(_) => { println!("Unable to move mouse, but let's not panic"); },
                _ => {}
            }
            sleep(Duration::from_millis(200));
        }
        
    }
    
    fn gen_seq(self: &Self, p1: Point, p2: Point) -> Vec<Point> {
        // * *    *    *  *
        vec![
            p1,
            linear_inter(&p1, &p2, between(p1.x, p2.x, 10) ),
            linear_inter(&p1, &p2, between(p1.x, p2.x, 20) ),
            linear_inter(&p1, &p2, between(p1.x, p2.x, 50) ),
            linear_inter(&p1, &p2, between(p1.x, p2.x, 80) ),
            linear_inter(&p1, &p2, between(p1.x, p2.x, 90) ),
            p2
        ]
    }

}

fn linear_inter(p1: &Point, p2: &Point, x: f64) -> Point {
    let y = p2.y+ (p1.y-p2.y)/ (p1.x-p2.x) * (x - p2.x);
    Point::new(x, y)
}

fn between(x1: f64, x2: f64, percent: i32) -> f64 {
    let dif = (x1-x2).abs() * (percent as f64 / 100.0);
    if x1 > x2 { x1-dif } else { x1+dif }
}

fn main() {
    println!("Press ENTER to exit...");
    thread::spawn(|| {
        MouseWave::new().start();
    });
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        _ => {}
    }
}