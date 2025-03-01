pub mod widget;
use sdl2::event::Event;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let screen_width = 800u32;
    let screen_height = 600u32;
    let sdl_context = sdl2::init()?;
    let sdl_video_subsystem = sdl_context.video()?;
    let window = sdl_video_subsystem
        .window("Rust!", screen_width, screen_height)
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    let mut event_poll = sdl_context.event_pump()?;

    'running: loop {
        for event in event_poll.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }

                _ => {}
            }
        }

        canvas.present();
    }

    Ok(())
}
