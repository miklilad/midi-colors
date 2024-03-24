#[derive(Debug)]
enum Key {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

impl Key {
    fn from_note_number(note_number: u8) -> Self {
        match note_number % 12 {
            0 => Key::A,
            1 => Key::Bb,
            2 => Key::B,
            3 => Key::C,
            4 => Key::Db,
            5 => Key::D,
            6 => Key::Eb,
            7 => Key::E,
            8 => Key::F,
            9 => Key::Gb,
            10 => Key::G,
            11 => Key::Ab,
            _ => unreachable!(),
        }
    }

    fn get_colour(self) -> (u8, u8, u8) {
        match self {
            Key::C => hsv_to_rgb(0.0, 1.0, 1.0),
            Key::G => hsv_to_rgb(30.0, 1.0, 1.0),
            Key::D => hsv_to_rgb(60.0, 1.0, 1.0),
            Key::A => hsv_to_rgb(90.0, 1.0, 1.0),
            Key::E => hsv_to_rgb(120.0, 1.0, 1.0),
            Key::B => hsv_to_rgb(150.0, 1.0, 1.0),
            Key::Gb => hsv_to_rgb(180.0, 1.0, 1.0),
            Key::Db => hsv_to_rgb(210.0, 1.0, 1.0),
            Key::Ab => hsv_to_rgb(240.0, 1.0, 1.0),
            Key::Eb => hsv_to_rgb(270.0, 1.0, 1.0),
            Key::Bb => hsv_to_rgb(330.0, 1.0, 1.0),
            Key::F => hsv_to_rgb(330.0, 1.0, 1.0),
        }
    }
}

pub struct Visualizer {
    // RGBA pixels
    pixels: Vec<(u8, u8, u8, u8)>,
    keys: [f32; 88],
    pedal: f32,
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    let r = ((r + m) * 255.0) as u8;
    let g = ((g + m) * 255.0) as u8;
    let b = ((b + m) * 255.0) as u8;
    (r, g, b)
}

impl Visualizer {
    pub fn new(width: u32) -> Self {
        let pixels = vec![(0, 0, 0, 0xFF); (width) as usize];
        let keys = [0.0; 88];
        let pedal = 0.0;
        Visualizer {
            pixels,
            keys,
            pedal,
        }
    }

    fn draw_pixels(&mut self) {
        for i in 0..self.pixels.len() {
            let key = i as f32 * 88.0 / self.pixels.len() as f32;
            let key = key as usize;
            let color = if self.keys[key] > 0.0 {
                let (r, g, b) = Key::from_note_number(key as u8).get_colour();
                (r, g, b, 0xFF)
                // let hue = 360.0 / (key % 12) as f32;
                // let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);
                // (r, g, b, 0xFF)
            } else {
                (0, 0, 0, 0xFF)
            };
            self.pixels[i] = color;
        }
    }

    pub fn receive_message(&mut self, message: &[u8]) {
        if message.len() == 3 {
            let status = message[0] & 0xF0;
            let note = message[1];
            let velocity = message[2] as f32 / 127.0;
            match status {
                0x90 => {
                    self.keys[(note - 21) as usize] = velocity;
                    let key = Key::from_note_number(note - 21);
                    println!("{:?} {:?}", key, velocity);
                }
                0x80 => {
                    self.keys[(note - 21) as usize] = 0.0;
                }
                0xB0 => {
                    if note == 64 {
                        self.pedal = velocity;
                    }
                }
                _ => {}
            }
        }
        self.draw_pixels();
    }

    pub fn step(&mut self) {}

    pub fn get_pixels(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .flat_map(|(r, g, b, a)| vec![*r, *g, *b, *a])
            .collect::<Vec<_>>()
    }
}
