use macroquad::{miniquad::conf::Icon, prelude::*, ui::root_ui};

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_mode = MODE::Menu;

    // Completed as of now, works fine and looks fine don't mess with this
    let label_text = "Choose your color";
    let button_text_white = "Play as White";
    let button_text_black = "Play as Black";

    loop {
        clear_background(GOLD);
        match current_mode {
            MODE::Menu => {
                // Completed as of now, works fine and looks fine don't mess with this
                let x = (screen_width() - measure_text(label_text, None, 20, 1.0).width) / 2.;
                let y = 0.;
                root_ui().label(vec2(x, y), label_text);
                root_ui().separator();
                let text_dimensions_white = measure_text(button_text_white, None, 20, 1.0);
                let text_dimensions_black = measure_text(button_text_black, None, 20, 1.0);
                let ui_width = screen_width();
                let x = 2. * ui_width / 6. - text_dimensions_white.width / 2.;
                let y = screen_height() / 6.;
                if root_ui().button(vec2(x, y), "Play as White") {
                    current_mode = MODE::PlayWhite;
                }
                let x = 4. * ui_width / 6. - text_dimensions_black.width / 2.;
                if root_ui().button(vec2(x, y), "Play as Black") {
                    current_mode = MODE::PlayBlack;
                }
            }
            MODE::PlayBlack => {
                // current_mode = MODE::Menu; // Last line of this block, uncomment after the block is completed
            }
            MODE::PlayWhite => {
                // current_mode = MODE::Menu; // Last line of this block, uncomment after the block is completed
            }
        }
        next_frame().await
    }
}

#[derive(PartialEq)]
enum MODE {
    Menu,
    PlayWhite,
    PlayBlack,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess".to_owned(),
        icon: Some(Icon {
            small: *include_bytes!("logo16.rgba"),
            medium: *include_bytes!("logo32.rgba"),
            big: *include_bytes!("logo64.rgba"),
        }),
        window_resizable: true,
        ..Default::default()
    }
}
