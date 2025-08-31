use std::num::NonZeroU32;
use std::rc::Rc;

use cgmath::{EuclideanSpace, InnerSpace, Point3, Vector3};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowId};

use crate::color::Color;

mod color;
mod ray;

fn hit_sphere(center: &Point3<f32>, radius: f32, ray: &ray::Ray) -> f32 {
    let oc = center - ray.orig;
    let a = ray.dir.dot(ray.dir);
    let b = -2.0 * ray.dir.dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: ray::Ray) -> Color {
    let hit = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r);
    if hit > 0.0 {
        let unit = (r.at(hit).to_vec() - Vector3::new(0.0, 0.0, -1.0)).normalize();
        // let N = unit / unit.magnitude();
        return (Vector3::new(unit.x + 1.0, unit.y + 1.0, unit.z + 1.0) * 0.5).into();
    }
    let unit_direction = r.dir / r.dir.magnitude(); // unit_vector(r.direction());
    let a: f32 = 0.5 * (unit_direction.y + 1.0);
    ((1.0 - a) * Vector3::<f32>::new(1.0, 1.0, 1.0) + a * Vector3::<f32>::new(0.5, 0.7, 1.0)).into()
}

struct App {
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<OwnedDisplayHandle, Rc<Window>>>,
}

impl Default for App {
    fn default() -> Self {
        App {
            window: None,
            surface: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Rc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let context = softbuffer::Context::new(event_loop.owned_display_handle()).unwrap();
        self.surface = Some(softbuffer::Surface::new(&context, window.clone()).unwrap());
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(surface)) = (self.window.as_ref(), &mut self.surface) {
                    let (width, height) = {
                        let size = window.inner_size();
                        (size.width, size.height)
                    };
                    // let aspect_ratio = NonZeroU32::new((width as f32 / height as f32) as u32);

                    //define camera
                    let focal_length = 1.0;
                    let viewport_height = 2.0;
                    let viewport_width = viewport_height * (width as f32 / height as f32);
                    let camera_center = Point3::new(0.0, 0.0, 0.0);

                    // Calculate the vectors across the horizontal and down the vertical viewport edges.
                    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
                    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

                    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
                    let pixel_delta_u = viewport_u / width as f32;
                    let pixel_delta_v = viewport_v / height as f32;

                    // Calculate the location of the upper left pixel.
                    let viewport_upper_left = camera_center
                        - Vector3::new(0.0, 0.0, focal_length)
                        - viewport_u / 2.0
                        - viewport_v / 2.0;
                    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

                    surface
                        .resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap(),
                        )
                        .unwrap();

                    let mut buffer = surface.buffer_mut().unwrap();

                    for j in 0..width {
                        for i in 0..height {
                            let pixel_center = pixel00_loc
                                + (j as f32 * pixel_delta_u)
                                + (i as f32 * pixel_delta_v);
                            let ray_direction = pixel_center - camera_center;
                            let r = ray::Ray::new(camera_center, ray_direction);

                            let pixel_color = ray_color(r);
                            buffer[(j + i * width) as usize] = pixel_color.into();
                        }
                    }

                    buffer.present().unwrap();
                }
                // self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

pub fn run() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

// pub fn run() -> anyhow::Result<()> {
//     info!("run");
//     #[cfg(target_arch = "wasm32")]
//     {
//         std::panic::set_hook(Box::new(console_error_panic_hook::hook));
//         console_log::init_with_level(log::Level::Debug).unwrap_throw();
//     }
//
//     let event_loop = EventLoop::with_user_event().build()?;
//     let mut app = App::new(
//         #[cfg(target_arch = "wasm32")]
//         &event_loop,
//     );
//     event_loop.run_app(&mut app)?;
//
//     Ok(())
// }
