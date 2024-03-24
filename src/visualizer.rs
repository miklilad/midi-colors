pub struct Visualizer {
    // RGBA pixels
    pixels: Vec<(u8, u8, u8, u8)>,
    keys: [f32; 88],
    pedal: f32,
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
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
    (r + m, g + m, b + m)
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
                let hue = 360.0 / (i % 12) as f32;
                let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);
                let r = (r * 255.0) as u8;
                let g = (g * 255.0) as u8;
                let b = (b * 255.0) as u8;
                (r, g, b, 0xFF)
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
            let velocity = message[2];
            match status {
                0x90 => {
                    self.keys[(note - 21) as usize] = velocity as f32 / 127.0;
                }
                0x80 => {
                    self.keys[(note - 21) as usize] = 0.0;
                }
                0xB0 => {
                    if note == 64 {
                        self.pedal = velocity as f32 / 127.0;
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
