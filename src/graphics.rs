use crate::screen::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn cat_graphic() -> &'static [u8] {
    [
        0x01, 0x01, 0x00, 0x07, 0x18, 0x22, 0x44, 0x22, 0x18, 0x06, 0x18, 0x22, 0x44, 0x22, 0x18,
        0x07, 0x00,
    ]
    .as_slice()
}

pub struct Graphics {
    pub width: u16,
    pub height: u16,
    pub data: [[u8; 16]; SCREEN_WIDTH / 8 * SCREEN_HEIGHT / 16],
}

pub fn load_gf(graphic: &'static [u8]) -> Graphics {
    let width: u16 = graphic[0] as u16;
    let height: u16 = graphic[1] as u16;
    let graphical_data = graphic.get(2..).unwrap();
    let mut data: [[u8; 16]; SCREEN_WIDTH / 8 * SCREEN_HEIGHT / 16] =
        [[0; 16]; SCREEN_WIDTH / 8 * SCREEN_HEIGHT / 16];
    for y in 0..height {
        for x in 0..width {
            let mut character: [u8; 16] = [0; 16];
            for h in 0..16 {
                character[h] = *graphical_data
                    .get((y * width * 16) as usize + (h * width as usize) + x as usize)
                    .unwrap_or(&0x00);
            }
            data[(y * width + x) as usize] = character;
        }
    }
    Graphics {
        width,
        height,
        data,
    }
}
