use instant::Instant;
use std::time::Duration;
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    simple_logger::init().unwrap();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    let timer_length = Duration::new(1, 0);

    let mut redraw = false;

    event_loop.run(move |event, _, control_flow| {
        println!("{:?}", event);
        match event {
            Event::NewEvents(start_cause) => {
                redraw = true;
                match start_cause {
                    StartCause::Init => {
                        *control_flow = ControlFlow::WaitUntil(Instant::now() + timer_length)
                    }
                    StartCause::ResumeTimeReached { .. } => {
                        *control_flow = ControlFlow::WaitUntil(Instant::now() + timer_length);
                        println!("\nTimer\n");
                    }
                    _ => {
                        redraw = false;
                    }
                }
            }
            Event::MainEventsCleared => {
                if redraw {
                    window.request_redraw();
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
