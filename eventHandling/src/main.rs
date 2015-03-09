extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{ Gl, OpenGL};
use piston::event::PressEvent;
use piston::event::{
	events,
	RenderArgs,
	RenderEvent,
};
use graphics::{
	Context,
	rectangle,
	RelativeTransform
};
use piston::input:: {
	Button,
	mouse,
};
pub struct AppButton {
	label: String,
	width: f64,
	pos_x: f64,
	pos_y: f64,
	bg: [f32; 4],
}

impl AppButton {
	fn new(name: String, w: f64) -> AppButton{
		AppButton {
			label: name,
			width: w,
			pos_x: 0.0,
			pos_y: 0.0,
			bg: [0.85, 0.147, 0.215, 1.0],
		}
	}

	fn set_position(&mut self, x: f64, y: f64) -> &mut AppButton{
		self.pos_x = x;
		self.pos_y = y;
		self
	}
}

pub struct App {
	name: String
}

impl App {
	pub fn draw_button(&mut self, gl: &mut Gl, but: &mut AppButton, args: &RenderArgs) {
		let context = &Context::abs(args.width as f64, args.height as f64);
		let button_dim = &context.trans(0.0 as f64, 0.0 as f64).trans(but.pos_x, but.pos_y);
		let button = rectangle::square(0.0, 0.0, but.width + but.width);

		gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
			graphics::rectangle(but.bg, button, button_dim, gl);
		});
	}
}

fn main() {
	let window = Window::new(
			OpenGL::_3_2,
			WindowSettings{
			title: "App Launcher".to_string(),
			size: [1180, 580],
			fullscreen: false,
			exit_on_esc: true,
			samples: 4,
			},
		);


	let mut ren = RenderArgs {
		ext_dt: 0.0,
		width: 0,
		height: 0,
	};

	let window = RefCell::new(window);
	let mut gl = Gl::new(OpenGL::_3_2);

	let mut app = App {
		name: "App Launcher".to_string()
	};
	let mut button1 = AppButton::new("Button 1".to_string(), 100.0);
	button1.set_position(0.0, 0.0);

	let mut button2 = AppButton::new("Button 2".to_string(), 100.0);
	button2.set_position(0.0, 200.0);
	button2.bg = [1.0, 1.0, 1.0, 1.0];
	for e in events(&window) {
		if let Some(r) = e.render_args() {
			ren = RenderArgs {
				ext_dt: r.ext_dt         ,
				width: r.width,
				height: r.height,
			};
			app.draw_button(&mut gl, &mut button1, &r);
		}

		if let Some(p) = e.press_args() {
			match p {
				Button::Mouse(b) => if b == mouse::MouseButton::Left { app.draw_button(&mut gl, &mut button2, &ren); } else { println!("Press Left Mouse Button."); },
				Button::Keyboard(k) => println!("Keyboard Button {:?}", k),
			}
		}
	}
}
