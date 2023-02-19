use std::f32::consts::PI;

use speedy2d::color::Color;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::window::{MouseButton, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use vector::Vector;

fn main() {
    let window = Window::new_centered("Pendulum", (800, 480)).unwrap();

    let font = Font::new(include_bytes!("./assets/bebas.ttf")).unwrap();

    let win = MyWindowHandler {
        p: Pendulum::new(400.0, 0.0, 200.0),
        font,
        grabbed: false,
        mouse_x: 0.0,
        mouse_y: 0.0,
    };

    window.run_loop(win)
}

struct MyWindowHandler {
    p: Pendulum,
    font: Font,
    grabbed: bool,
    mouse_x: f32,
    mouse_y: f32,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        self.p.update();
        if self.grabbed {
            let diff = Vector::new(
                self.p.origin.x - self.mouse_x,
                self.p.origin.y - self.mouse_y,
            );

            self.p.position.set(self.mouse_x, self.mouse_y);
            self.p.r = ((self.p.position.x - self.p.origin.x).powi(2)
                + (self.p.position.y - self.p.origin.y).powi(2))
            .sqrt();
            self.p.angular_acceleration = 0.0;
            self.p.angular_velocity = 0.0;
            self.p.angle = (-diff.y).atan2(diff.x) - PI / 2.0;
        }
        self.p.draw(graphics, &self.font);

        helper.request_redraw();
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode,
    ) {
        match scancode {
            57416 => self.p.g += 0.1, // UP Arrow - Increase Gravity
            57424 => self.p.g -= 0.1, // DOWN Arrow - Decrease Gravity
            57419 => self.p.m -= 1.0, // LEFT Arrow - Decrease Mass
            57421 => self.p.m += 1.0, // RIGHT Arrow - Increase Mass
            19 => {
                // R - Reset pendulum position
                self.p.r = 200.0;
                self.p.angle = 1.0;
            }
            _ => return,
        }
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: speedy2d::dimen::Vec2) {
        self.mouse_x = position.x;
        self.mouse_y = position.y;
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        if button == MouseButton::Left {
            if self.p.distance(&Vector {
                x: self.mouse_x,
                y: self.mouse_y,
            }) < 28.0
            {
                self.grabbed = false;
                self.p.angular_velocity = 0.0;
            }
        }
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        if button == MouseButton::Left {
            if self.p.distance(&Vector {
                x: self.mouse_x,
                y: self.mouse_y,
            }) < 28.0
            {
                self.grabbed = true;
            }
        }
    }
}

struct Pendulum {
    origin: Vector,

    position: Vector,

    angle: f32,

    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32,
    m: f32,
    g: f32,
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum {
            origin: Vector::new(x, y),
            position: Vector::new(0.0, 0.0),
            angle: 1.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            r,
            m: 1.0,
            g: 0.5,
        }
    }

    fn update(&mut self) {
        let dumping = 0.995 - 0.0003 * self.m / 3.0;

        self.angular_acceleration = -self.g * self.angle.sin() / self.r;

        self.angular_velocity += self.angular_acceleration;

        self.angular_velocity *= dumping;

        self.angle += self.angular_velocity;

        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());

        self.position.add(&self.origin);
    }

    fn draw(&mut self, graphics: &mut Graphics2D, font: &Font) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::GRAY,
        );

        graphics.draw_text(
            (0.0, 0.0),
            Color::BLACK,
            &font.layout_text(
                format!("Gravity: {:.2}", self.g).as_str(),
                30.0,
                TextOptions::new(),
            ),
        );

        graphics.draw_text(
            (0.0, 30.0),
            Color::BLACK,
            &font.layout_text(
                format!("Angle: {:.2}", self.angle).as_str(),
                30.0,
                TextOptions::new(),
            ),
        );
        graphics.draw_text(
            (0.0, 60.0),
            Color::BLACK,
            &font.layout_text(
                format!("Acceleration: {:.2}", self.angular_acceleration * 10.0).as_str(),
                30.0,
                TextOptions::new(),
            ),
        );
        graphics.draw_text(
            (0.0, 90.0),
            Color::BLACK,
            &font.layout_text(
                format!("Velocity: {:.2}", self.angular_velocity).as_str(),
                30.0,
                TextOptions::new(),
            ),
        );
        graphics.draw_text(
            (0.0, 120.0),
            Color::BLACK,
            &font.layout_text(
                format!("Mass: {:.2}", self.m).as_str(),
                30.0,
                TextOptions::new(),
            ),
        );

        graphics.draw_circle((self.position.x, self.position.y), 28.0, Color::DARK_GRAY);
        graphics.draw_circle((self.position.x, self.position.y), 25.0, Color::LIGHT_GRAY);
    }

    fn distance(&mut self, other: &Vector) -> f32 {
        ((self.position.x - other.x).powi(2) + (self.position.y - other.y).powi(2)).sqrt()
    }
}

mod vector {
    #[derive(Copy, Clone)]
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y }
        }

        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;

            self
        }

        pub fn set(&mut self, x: f32, y: f32) {
            self.x = x;
            self.y = y;
        }

        pub fn sub(&mut self, other: &Vector) -> &Vector {
            self.x -= other.x;
            self.y -= other.y;

            self
        }
    }
}
