mod parser;
use rust_html_engine::run;
use macroquad::prelude::*;

use std::fs::File;
use std::io::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let file_path = "./test.html";
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        eprintln!("Error reading file: {}", err);
        return;
    }

    let root = parser::Parser::parse(contents);

    println!("{}", root);

    // loop {
    //     clear_background(RED);
    //     draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    //     draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    //     draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
    //     draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);
    //
    //     next_frame().await
    // }
}
