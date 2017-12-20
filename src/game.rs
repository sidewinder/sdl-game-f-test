use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::init;
use sdl2::video::FullscreenType;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

pub struct Game {
    is_running : bool,
    sdl_context: Sdl,    
    canvas: Canvas<Window>,       
}

impl Game  {
    pub fn new(title: &str, width: u32, height: u32) -> Game {

        let sdl_context : Sdl = init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        let window = video_subsystem.window(title, width, height)        
        .position_centered()        
        .opengl()
        .build()
        .unwrap();
        
        let canvas = window.into_canvas().present_vsync().build().unwrap();

        Game {
            is_running :  true,
            sdl_context : sdl_context,             
            canvas : canvas           
            }
    }

    pub fn init(&mut self, fullscreen: bool) {
        println!("Init method");

        let mut is_fullscreem = FullscreenType::Off;
        if fullscreen == true {
            is_fullscreem = FullscreenType::True;
        }
        self.canvas.window_mut().set_fullscreen(is_fullscreem);
    }

    pub fn handle_events(&mut self) {

        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
                match event {
                        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        self.is_running = false;
                    },
                    _ => {}
                }
            }
    }

    pub fn update(&mut self) {
        
    }

    pub fn render(&mut self) {

        self.canvas.set_draw_color(Color::RGB(0, 0, 255));
        self.canvas.clear();
        
        let (size_width, size_hight) = self.canvas.output_size().unwrap();
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        self.canvas.fill_rect(Rect::new((size_width / 2 - 50 / 2) as i32, (size_hight / 2 - 50 / 2) as i32, 50, 50)).unwrap();

        self.canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    pub fn clean(&mut self) {
        println!("Clean method");
    }

    pub fn is_running_instance(&self) -> bool {
        self.is_running
    }
}