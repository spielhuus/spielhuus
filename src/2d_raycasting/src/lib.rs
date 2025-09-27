use std::num::NonZeroU32;
use std::rc::Rc;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowId};

use log::info;
use mint::{Point2, Vector2};
use rand::Rng;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const POINTS_RADIUS: usize = 4;
const POINTS_COLOR: u32 = 0xFFFFFF;


const MAP_WIDTH: u32 = 24;
const MAP_HEIGHT: u32 = 24;
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

const WORLD_MAP: [[u32; MAP_HEIGHT as usize]; MAP_WIDTH as usize] =
[
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
];

pub static PASTEL_PALETTE: &[u32] = &[
    // Reds & Terracottas
    0xD0A0A5, // 1. Dusty Rose
    0xC87E6F, // 2. Terracotta
    0xB06B5C, // 3. Cinnamon Stick
    0x9B5B5F, // 4. Faded Garnet
    0xC58C79, // 5. Muted Clay
    // Oranges & Yellows
    0xDDAA55, // 6. Golden Ochre
    0xC7A250, // 7. Spiced Mustard
    0xD18460, // 8. Burnt Sienna
    0xEAA243, // 9. Marigold
    0xC88A3B, // 10. Amber Glow
    // Greens
    0x7A8D7B, // 11. Deep Sage
    0x6A7049, // 12. Olive Drab
    0x838A73, // 13. Mossy Stone
    0x5A6349, // 14. Forest Floor
    0x4A7C82, // 15. Muted Teal
    // Blues
    0x6A829A, // 16. Slate Blue
    0x6C85A1, // 17. Faded Denim
    0x5E7485, // 18. Stormy Sky
    0x7E88B0, // 19. Deep Periwinkle
    0x8D9BA6, // 20. Coastal Fog
    // Purples & Mauves
    0x9A8B9E, // 21. Dusty Heather
    0x7D5A7D, // 22. Faded Plum
    0xA3869C, // 23. Smoky Mauve
    0x786684, // 24. Withered Iris
    0x826D8C, // 25. Vintage Violet
    // Neutrals & Browns
    0xA8988A, // 26. Warm Taupe
    0x8C8C8C, // 27. Stonewash
    0x6E5A4F, // 28. Rich Umber
    0x5A5A5A, // 29. Charcoal Dust
    0xB4AFAF, // 30. Pebble Path
];

struct App {
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<OwnedDisplayHandle, Rc<Window>>>,
    pos: Point2<f32>,
    dir: Vector2<f32>,
    plane: Vector2<f32>,
    time: f64,
    old_time: f64,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            surface: None,
            pos: Point2 { x: 22.0, y: 22.0 },
            dir: Vector2 { x: -1.0, y: 0.0 },
            plane: Vector2 { x: 0.0, y: 0.66 },
            time: 0.0,
            old_time: 0.0,
        }
    }
    // fn reset(&mut self) {
    //     //create the points
    //     if let Some(window) = &self.window {
    //         let (width, height) = (window.inner_size().width, window.inner_size().height);
    //         info!("window: {width}x{height}");
    //         let mut rng = rand::rng();
    //         self.points = (0..POINTS_COUNT)
    //             .map(|_| Point2 {
    //                 x: rng.random_range(0..width as i32),
    //                 y: rng.random_range(0..height as i32),
    //             })
    //             .collect();
    //     }
    // }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let canvas = document.get_element_by_id("shader").unwrap();
            let canvas: web_sys::HtmlCanvasElement =
                canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

            let attributes = Window::default_attributes().with_canvas(Some(canvas));

            let window = Rc::new(event_loop.create_window(attributes).unwrap());

            let context = softbuffer::Context::new(event_loop.owned_display_handle()).unwrap();
            self.surface = Some(softbuffer::Surface::new(&context, window.clone()).unwrap());
            self.window = Some(window);
            // self.reset();
        }

        let window = Rc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        #[cfg(not(target_arch = "wasm32"))]
        {
            let context = softbuffer::Context::new(event_loop.owned_display_handle()).unwrap();
            self.surface = Some(softbuffer::Surface::new(&context, window.clone()).unwrap());
            self.window = Some(window);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(_size) => {
            }
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(surface)) = (self.window.as_ref(), &mut self.surface) {
                    let (width, height) = {
                        let size = window.inner_size();
                        (size.width, size.height)
                    };

                    surface
                        .resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap(),
                        )
                        .unwrap();


                    let mut buffer = surface.buffer_mut().unwrap();
                    
                    //the position on the map
                    let map_x = self.pos.x;
                    let map_y = self.pos.y;

                    //start raycasting
                    for x in 0..SCREEN_WIDTH {
                        let camera_x = 2.0 * x as f32 / SCREEN_WIDTH as f32 - 1.0;
                        let ray_dir_x = self.dir.x + self.plane.x * camera_x;
                        let ray_dir_y = self.dir.y + self.plane.y * camera_x;

                        let (step_x, side_dist_x) = if ray_dir_x < 0.0 {
                            (-1, self.pos.x) 
                        } else {
                            (1.0, (map_x = 1.0 - self.pos.x) * delta_dist_x)
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

#[cfg(not(target_arch = "wasm32"))]
pub fn run() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    info!("run");
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).unwrap_throw();

    let event_loop = EventLoop::with_user_event().build().unwrap_throw();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap_throw();

    Ok(())
}
