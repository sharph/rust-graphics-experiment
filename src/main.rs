use nannou::prelude::*;
use state::InitCell;
use std::sync::Mutex;

static ITEM_ARRAY: InitCell<Mutex<Vec<Item>>> = InitCell::new();

const NUM_ITEMS: u8 = 100;

struct Item {
    x: f64,
    y: f64,
    vel_x: f64,
    vel_y: f64,
}

impl Item {
    fn calc(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;

        if self.x < 0.0 || self.x > 400.0 {
            if self.x < 0.0 {
                self.x *= -1.0;
            } else {
                self.x = 800.0 - self.x;
            }
            self.vel_x *= -0.9;
            self.vel_y *= 0.95;
        }
        if self.y < 0.0 || self.y > 400.0 {
            if self.y < 0.0 {
                self.y *= -1.0;
            } else {
                self.y = 800.0 - self.y;
            }
            self.vel_y *= -0.9;
            self.vel_x *= 0.95;
        }

        self.vel_y -= 0.1;
    }
}

fn init() {
    ITEM_ARRAY.set(Mutex::new(Vec::new()));
    let mut items = ITEM_ARRAY.get().lock().unwrap();
    for _ in 0..NUM_ITEMS {
        let item = Item {
            x: rand::random::<f64>() * 400.0,
            y: rand::random::<f64>() * 400.0,
            vel_x: (rand::random::<f64>() - 0.5) * 50.0,
            vel_y: (rand::random::<f64>() - 0.5) * 50.0,
        };
        items.push(item);
    }
}

fn main() {
    init();
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let mut items = ITEM_ARRAY.get().lock().unwrap();

    // set background to blue
    draw.background().color(BLACK);

    for item in items.iter_mut() {
        item.calc();
        draw.ellipse()
            .color(WHITE)
            .w_h(4.0, 4.0)
            .x_y(item.x as f32 - 200.0, item.y as f32 - 200.0);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
