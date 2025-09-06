use std::num::NonZeroU32;
use std::rc::Rc;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowId};

use log::info;
use mint::Point2;
use rand::Rng;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const POINTS_COUNT: usize = 40;
const POINTS_RADIUS: usize = 4;
const POINTS_COLOR: u32 = 0xFFFFFF;

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
    points: Vec<Point2<i32>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            surface: None,
            points: Vec::with_capacity(POINTS_COUNT),
        }
    }
    fn reset(&mut self) {
        //create the points
        if let Some(window) = &self.window {
            let (width, height) = (window.inner_size().width, window.inner_size().height);
            info!("window: {width}x{height}");
            let mut rng = rand::rng();
            self.points = (0..POINTS_COUNT)
                .map(|_| Point2 {
                    x: rng.random_range(0..width as i32),
                    y: rng.random_range(0..height as i32),
                })
                .collect();
        }
    }
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
            self.reset();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(_size) => {
                self.reset();
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
                    let radius_sq = POINTS_RADIUS * POINTS_RADIUS;
                    let max_dist = width * width + height * height;
                    for x in 0..width as i32 {
                        for y in 0..height as i32 {
                            let mut is_point = false;
                            let (mut dist, mut id) = (max_dist, 0);
                            for (i, point) in self.points.iter().enumerate() {
                                let dx = (x - point.x).abs();
                                let dy = (y - point.y).abs();
                                let d = dx * dx + dy * dy;
                                if d < radius_sq as i32 {
                                    is_point = true;
                                    break;
                                } else if d < dist as i32 {
                                    dist = d as u32;
                                    id = i % PASTEL_PALETTE.len();
                                }
                            }
                            if is_point {
                                buffer[(x + y * width as i32) as usize] = POINTS_COLOR;
                            } else {
                                buffer[(x + y * width as i32) as usize] = PASTEL_PALETTE[id];
                            }
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
