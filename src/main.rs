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

        let (hit_x, hit_y);
        (self.x, hit_x) = {
            if self.x < 0.0 {
                (self.x * -1.0, true)
            } else if self.x > 400.0 {
                (800.0 - self.x, true)
            } else {
                (self.x, false)
            }
        };
        (self.y, hit_y) = {
            if self.y < 0.0 {
                (self.y * -1.0, true)
            } else if self.y > 400.0 {
                (800.0 - self.y, true)
            } else {
                (self.y, false)
            }
        };
        if hit_x || hit_y {
            self.vel_y *= 0.9 * if hit_y { -1.0 } else { 1.0 };
            self.vel_x *= 0.95 * if hit_x { -1.0 } else { 1.0 };
        }

        self.vel_y -= 0.1;
    }
}

fn main() {
    ITEM_ARRAY.set(Mutex::new(Vec::new()));
    {
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
    } // block to scope out the mutex, which needs to be unlocked for .run()
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let mut items = ITEM_ARRAY.get().lock().unwrap();

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
