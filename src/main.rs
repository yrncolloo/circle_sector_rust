use raylib::{color::Color, drawing::RaylibDraw, ffi::Vector2, math::{rrect, Rectangle}, rgui::RaylibDrawGui, rstr};

const SCREEN_WIDHT: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDHT, SCREEN_HEIGHT)
        .title("Draw circle sector")
        .build();

    let center = Vector2{
        x: (SCREEN_WIDHT as f32 - 300.0)/2.0,
        y: SCREEN_HEIGHT as f32/ 2.0 

    };
    
    let mut outer_radius = 180.0;
    let mut start_angle = 0.0;
    let mut end_angle = 180.0;
    let mut segment = 100.0;
    let mut mini_segment = 4.0;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_line(500, 0, 500, SCREEN_HEIGHT, Color::LIGHTGRAY);
        d.draw_rectangle(500, 0, SCREEN_WIDHT - 500, SCREEN_HEIGHT, Color::LIGHTGRAY);

        d.draw_circle_sector(center, outer_radius, start_angle, end_angle, segment as i32, Color::MAROON);
        d.draw_circle_sector_lines(center, outer_radius, start_angle, end_angle, segment as i32, Color::GREEN);
        
        // GUI Controls
        //

        // Start angle
        let start_angle_string = rstr!("{:.2}", start_angle);
        let str_start_angle = Some(start_angle_string.as_c_str());
        d.gui_slider_bar(Rectangle{
            x: 600.0,
            y: 40.0,
            width: 120.0,
            height: 20.0
        }, 
       Some(rstr!("Start angle")), 
       str_start_angle,
        &mut start_angle,
        0.0,
        720.0);

        // end angle
        let end_angle_string = rstr!("{:.2}", end_angle);
        let str_end_angle = Some(end_angle_string.as_c_str());
        d.gui_slider_bar(Rectangle{
            x: 600.0,
            y: 70.0,
            width: 120.0,
            height: 20.0
        }, 
       Some(rstr!("End angle")), 
       str_end_angle,
        &mut end_angle,
        0.0,
        720.0);

        // radius
        let radius_string = rstr!("{:.2}", outer_radius);
        let str_radius = Some(radius_string.as_c_str());
        d.gui_slider_bar(rrect(600, 140, 120, 20), 
       Some(rstr!("Radius")), 
       str_radius,
        &mut outer_radius,
        0.0,
        200.0);
        //
        // segment
        let segment_string = rstr!("{:.2}", segment);
        let str_segment = Some(segment_string.as_c_str());
        d.gui_slider_bar(rrect(600, 170, 120, 20), 
       Some(rstr!("Segments")), 
       str_segment,
        &mut segment,
        0.0,
        100.0);


        // mini_segment 
        //

        mini_segment =( end_angle - start_angle) / 90.0;
        let text_mode = if segment >= mini_segment{
            "MANUAL"
        }else{
            "AUTO"
        };

        let color_mode= if segment >= mini_segment {
            Color::MAROON
        }else{
            Color::DARKGRAY
        };
        d.draw_text(&format!("MODE: {}",text_mode), 600, 200, 10, color_mode);
    }

}
