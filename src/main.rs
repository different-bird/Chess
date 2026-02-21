use macroquad::prelude::*;

#[macroquad::main("Chess")]
async fn main()
{
    loop {
        clear_background(RED);



        next_frame().await
    }
}