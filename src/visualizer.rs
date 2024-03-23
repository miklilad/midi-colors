pub struct Visualizer {
    // RGBA pixels
    pixels: Vec<(u8, u8, u8, u8)>,
    keys: [f32; 88],
    pedal: f32,
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
                (0xFF, 0xFF, 0xFF, 0xFF)
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
