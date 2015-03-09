extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use piston::window::WindowSettings;
use piston::event::{
	events,
	RenderArgs,
	RenderEvent,
	UpdateArgs,
	UpdateEvent,
};
use graphics::{
	Context,
	rectangle,
	RelativeTransform
};
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{ Gl, OpenGL};

// GUI Application struct intialize
pub struct App {
	gl: Gl, // OpenGL drawing backend.
	distance: f64,
	speed: f64
}

// implementing render and update function of App
impl App {

	// function to render the window with current value of the object
	fn render(&mut self, _: &mut Window, args: &RenderArgs) {
		const GREEN:  [f32; 4] = [1.0, 1.0, 1.0, 1.0];
		const RED:    [f32; 4] = [0.85, 0.147, 0.215, 1.0];
		
		// Set up a context to draw into.
		let context = &Context::abs(args.width as f64, args.height as f64);

		let center_context = &context.trans((args.width / args.width) as f64, (args.height / args.height) as f64)
		.trans(self.distance + self.speed, self.distance + self.speed);
		let square = rectangle::square(0.0, 0.0, 100.0);

		self .gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
		// Clear the screen.
		graphics::clear(GREEN, gl);
		graphics::rectangle(RED, square, center_context, gl);
		});
	}

	// update the object dimension
	fn update(&mut self, _: &mut Window, args: &UpdateArgs) {
		self.distance += 2.0 * args.dt;
		self.speed += 20.0 * args.dt;
	}
}

fn main() {
	// Create an SDL window.
	let window = Window::new(
		OpenGL::_3_2,
		WindowSettings::default()
	);
	let window = RefCell::new(window);

	// Create a new application and run it.
	let mut app = App {
		gl: Gl::new(OpenGL::_3_2),
		distance: 0.0,
		speed: 10.0
	};

	// capture all window events
	for e in events(&window) {
		// render the window
		if let Some(r) = e.render_args() {
			app.render(&mut window.borrow_mut(), &r);
			println!("{:?}", r);
		}

		// update the window
		if let Some(u) = e.update_args() {
			app.update(&mut window.borrow_mut(), &u);
		}
	}
}
