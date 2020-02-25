use std::collections::HashMap;
//use std::time;
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

//const WAIT_TIME: time::Duration = time::Duration::from_millis(100);

fn main() {
    simple_logger::init().unwrap();
    let event_loop = EventLoop::new();

    let mut windows = HashMap::new();
    for _ in 0..3 {
        let window = Window::new(&event_loop).unwrap();
        windows.insert(window.id(), window);
    }

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;
        println!("{:?}", event);
        match event {
            Event::WindowEvent { event, window_id } => {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("Window {:?} has received the signal to close", window_id);

                        // This drops the window, causing it to close.
                        windows.remove(&window_id);

                        if windows.is_empty() {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        let window = Window::new(&event_loop).unwrap();
                        windows.insert(window.id(), window);
                    }
                    _ => (),
                }
            }
            Event::MainEventsCleared => {
                //for window in windows.values() {
                //window.request_redraw();
                //};
            }
            Event::RedrawEventsCleared => {
                //*control_flow = ControlFlow::WaitUntil(time::Instant::now() + WAIT_TIME);
            }
            _ => (),
        }
    })
}
