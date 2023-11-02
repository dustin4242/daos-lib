use crate::screen::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn cat_graphic() -> &'static [u8] {
    include_bytes!("./cat.gf")
}

pub struct Graphics {
    pub width: u16,
    pub height: u16,
    pub data: [[u8; 16]; SCREEN_WIDTH / 8 * SCREEN_HEIGHT / 16],
    pub color_pallete: Option<[(u8, u8, u8); 8]>,
    pub color_data: Option<&'static [u8]>,
}

pub fn load_gf(graphic: &'static [u8]) -> Graphics {
    let width: u16 = graphic[0] as u16;
    let height: u16 = graphic[1] as u16;
    let graphical_data = graphic.get(2..(width * height * 16) as usize + 2).unwrap();
    let mut data: [[u8; 16]; SCREEN_WIDTH / 8 * SCREEN_HEIGHT / 16] =
        [[0; 16]; SCREEN_WIDTH / 8 * SCREEN_HEIGHT / 16];
    for y in 0..height {
        for x in 0..width {
            let mut character: [u8; 16] = [0; 16];
            for h in 0..16 {
                character[h] = *graphical_data
                    .get((y * width * 16) as usize + (h * width as usize))
                    .unwrap();
            }
            data[(y * width + x) as usize] = character;
        }
    }
    let mut color_index = (width * height * 16) as usize + 2;
    let mut color_pallete = None;
    if graphic.get(color_index..color_index + 8 * 3).is_some() {
        let mut temp: [(u8, u8, u8); 8] = [(0, 0, 0); 8];
        for x in 0..8 {
            let mut rgb = (0, 0, 0);
            rgb.0 = *graphic.get(color_index + x * 3).unwrap();
            rgb.1 = *graphic.get(color_index + x * 3 + 1).unwrap();
            rgb.2 = *graphic.get(color_index + x * 3 + 2).unwrap();
            temp[x] = rgb;
        }
        color_pallete = Some(temp);
    }
    color_index += 8 * 3;
    let color_data = graphic.get(color_index..);
    Graphics {
        width,
        height,
        data,
        color_pallete,
        color_data,
    }
}
