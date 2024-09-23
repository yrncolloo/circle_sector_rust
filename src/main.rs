use raylib::{color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, ffi::Vector2, math::Rectangle, rgui::RaylibDrawGui, rstr};

const SCREEEN_WIDHT: i32 = 800;
const SCREEEN_HEIGHT: i32 = 450;
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEEN_WIDHT, SCREEEN_HEIGHT)
        .title("Rust Draw Circle Sector")
        .build();

    let mut outer_radius = 180.0;
    let mut start_angle = 0.0;
    let mut end_angle = 180.0;
    let mut segments = 10.0;
    let mut mini_segment = 4.0;

    let center = Vector2{
        x: (SCREEEN_WIDHT as f32 -300.0) /2.0,
        y: SCREEEN_HEIGHT as f32 / 2.0

    };

    while !rl.window_should_close(){
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_line(500, 0, 500, SCREEEN_HEIGHT, Color::LIGHTGRAY);
        d.draw_rectangle(500, 0, SCREEEN_WIDHT - 500, SCREEEN_HEIGHT, Color::LIGHTGRAY);

        d.draw_circle_sector(center, outer_radius, start_angle, end_angle, segments as i32, Color::MAROON);
        d.draw_circle_sector_lines(center, outer_radius, start_angle, end_angle, segments as i32, Color::GREEN);

        // Drawing gui Controls
        let rec1 = Rectangle{
            x: 600.0,
            y: 40.0,
            width: 120.0,
            height: 20.0

        };
        let rec2 = Rectangle{
            x: 600.0,
            y: 70.0,
            width: 120.0,
            height: 20.0

        };
        let rec3 = Rectangle{
            x: 600.0,
            y: 140.0,
            width: 120.0,
            height: 20.0

        };
        let rec4 = Rectangle{
            x: 600.0,
            y: 170.0,
            width: 120.0,
            height: 20.0

        };
        
        create_gui_slide_bar(&mut d, rec1, "Start Angle".to_owned(), start_angle, &mut start_angle, 0.0, 720.0);
        create_gui_slide_bar(&mut d, rec2, "End Angle".to_owned(), end_angle, &mut end_angle, 0.0, 720.0);
        create_gui_slide_bar(&mut d, rec3, "Radius".to_owned(), outer_radius, &mut outer_radius, 0.0, 200.0);
        create_gui_slide_bar(&mut d, rec4, "segments".to_owned(), segments, &mut segments, 0.0, 100.0);

        let mode = if segments >= mini_segment {
            "Manual"
        }else{
            "Auto"
        };
        let seg_color = if segments >= mini_segment {
            Color::MAROON
        }else{
            Color::DARKGRAY
        };
        mini_segment = (end_angle - start_angle) / 90.0;
        d.draw_text(&format!("MODE: {}",mode), 600, 200, 10, seg_color);
    }

}

fn create_gui_slide_bar(
    handle: &mut RaylibDrawHandle, 
    rect: Rectangle, 
    lable_left: String, 
    lable_right: f32,
    value: &mut f32,
    min_value: f32,
    max_value: f32

    ) {

    let  text_left_string = rstr!("{}", lable_left);
    let text_left = Some(text_left_string.as_c_str());

    let  text_right_string = rstr!("{:.2}", lable_right);
    let text_right = Some(text_right_string.as_c_str());
    
    handle.gui_slider_bar(rect, text_left, text_right, value, min_value, max_value);
}
