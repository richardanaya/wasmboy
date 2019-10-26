extern "C" {
    fn random_number()->f32;
    fn frame_buffer_write(start:u32);
}

pub fn blit(pixels:Vec<u8>){
    unsafe {
        frame_buffer_write(pixels.as_ptr() as u32);
    }
}

pub fn rand() -> f32 {
    unsafe { random_number() }
}

pub const SCREEN_WIDTH:usize = 160;
pub const SCREEN_HEIGHT:usize = 144;