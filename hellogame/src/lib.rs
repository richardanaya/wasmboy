use wasmboy::*;

#[no_mangle]
pub fn key_down(_key_code:u8) -> () {
}

#[no_mangle]
pub fn key_up(_key_code:u8) -> () {
}


#[no_mangle]
pub fn run() -> () {
    let mut screen = vec![0;SCREEN_WIDTH*SCREEN_HEIGHT*4];
    for i in 0..screen.len() {
        screen[i] = (rand()*255.0) as u8;
    }
    blit(screen);
}