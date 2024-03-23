mod visualizer;
extern crate midir;

use midir::{Ignore, MidiInput, MidiInputConnection};
use pixels::{Pixels, SurfaceTexture};
use std::error::Error;
use std::sync::mpsc::{channel, Receiver, Sender};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

const WIDTH: u32 = 3080;
const HEIGHT: u32 = 88;

fn create_midi_receiver() -> Result<(Receiver<Vec<u8>>, MidiInputConnection<()>), Box<dyn Error>> {
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (we use the first available input port).
    let ports = midi_in.ports();

    println!(
        "Available ports {:?}",
        ports
            .iter()
            .map(|p| midi_in.port_name(p).unwrap())
            .collect::<Vec<_>>()
    );

    let port = ports.get(1).ok_or("no input port found")?;

    // Read MIDI input.
    println!(
        "Listening for MIDI messages on {}",
        midi_in.port_name(port)?
    );

    let (sender, receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();

    // _conn_in needs to be kept alive to keep receiving messages
    let conn_in = midi_in.connect(
        port,
        "midir-read-input",
        move |_, message, _| {
            // println!("{}: {:?} (len = {})", stamp, message, message.len());
            // Send a copy of the message to the main thread
            if let Err(err) = sender.send(message.to_vec()) {
                eprintln!("Error sending MIDI message: {}", err);
            }
        },
        (),
    )?;

    Ok((receiver, conn_in))
}

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
        .with_title("MIDI colors")
        .build(&event_loop)?;

    let size = window.inner_size();
    let surface_texture: SurfaceTexture<'_, winit::window::Window> =
        SurfaceTexture::new(size.width, size.height, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

    let (receiver, _conn_in) = create_midi_receiver()?;

    let mut visualizer = visualizer::Visualizer::new(WIDTH);

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_) => {
            let visualizer_pixels: Vec<u8> = visualizer.get_pixels().repeat(HEIGHT as usize);
            pixels.frame_mut().copy_from_slice(&visualizer_pixels);
            pixels.render().unwrap();
        }
        Event::MainEventsCleared => {
            if let Ok(msg) = receiver.try_recv() {
                visualizer.receive_message(&msg);
            }
            window.request_redraw();
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            WindowEvent::KeyboardInput { input, .. } => {
                if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                    control_flow.set_exit();
                }
                if input.virtual_keycode == Some(VirtualKeyCode::Space)
                    && input.state == ElementState::Pressed
                {
                    window.request_redraw();
                }
            }
            _ => {}
        },
        _ => {}
    });
}
