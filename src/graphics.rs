use macroquad::prelude::*;

pub async fn draw_monitor(x: f32, y: f32, w: f32, h: f32, buf: &[u8]) {
    let pixel_width = w / 160.0;
    let pixel_height = h / 120.0;

    for row in 0..120 {
        for col in 0..20 {
            let word = buf[(row * 20 + col) as usize];

            for pixel in 0..8 {
                let is_pixel_white = (word >> pixel) & 0x01 == 1;
                let color = if is_pixel_white { WHITE } else { BLACK };
                draw_rectangle(
                    x + ((col as f32 * 8.0 + pixel as f32) * pixel_width),
                    y + (row as f32 * pixel_height),
                    pixel_width,
                    pixel_height,
                    color,
                );
            }
        }
    }
}

pub async fn draw_leds(x: f32, y: f32, led_values: u16) {
    let led_spacing = 30.0;
    let led_size = 10.0;

    for led in 0..16 {
        let is_led_on = (led_values >> (15 - led)) & 0x01 == 1;
        let color = if is_led_on { GREEN } else { BLACK };
        let x_pos = x + led as f32 * led_spacing + (led / 4) as f32 * led_spacing;
        draw_rectangle(x_pos, y, led_size, led_size, color);
    }
}

pub async fn draw_switches(x: f32, y: f32, switch_states: &mut u16) {
    let led_spacing = 30.0;

    for switch in 0..16 {
        let is_sw_on = (*switch_states >> (15 - switch)) & 0x01 == 1;
        let x_pos = x + switch as f32 * led_spacing + (switch / 4) as f32 * led_spacing;
        draw_rectangle(x_pos, y, 10.0, 30.0, WHITE);
        let offset = if is_sw_on { 0f32 } else { 25f32 };
        draw_rectangle(x_pos, y + offset, 10.0, 5.0, BLACK);
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        for switch in 0..16 {
            let x_pos = x + switch as f32 * led_spacing + (switch / 4) as f32 * led_spacing;
            let dx = mouse_x - x_pos;
            let dy = mouse_y - y;

            if 0f32 < dx && dx < 10f32 && 0f32 < dy && dy < 30f32 {
                *switch_states ^= 0x01 << (15 - switch);
            }
        }
    }
}

pub async fn get_curr_button_states() -> u8 {
    let mut states = 0;

    if is_key_down(KeyCode::W) {
        states |= 16;
    }
    if is_key_down(KeyCode::X) {
        states |= 8;
    }
    if is_key_down(KeyCode::A) {
        states |= 4;
    }
    if is_key_down(KeyCode::D) {
        states |= 2;
    }
    if is_key_down(KeyCode::S) {
        states |= 1;
    }

    states
}
