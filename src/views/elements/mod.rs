use fltk::{
    button::*,
    enums::{Color, Font, FrameType},
    frame::Frame,
    prelude::*,
};
// use crate::views::logic; // working

// // getting asset folder contents
// #[derive(RustEmbed)]
// #[folder = "assets/"]
// struct Assets;

pub fn gen_login_button() -> Button {
    let mut but = Button::default().with_size(70, 30).with_label("Login");
    but.set_color(Color::from_u32(0xd6027b));
    but.set_label_color(Color::from_u32(0xffffff));
    but.set_frame(FrameType::RFlatBox);
    but.set_selection_color(Color::from_u32(0xff0393));
    but
}

pub fn gen_output_label_frame(x: i32, y: i32, width: i32, height: i32) -> Frame {
    let mut output_label_frame = Frame::new(x, y, width, height, "ECCLESSIASTICA");
    {
        output_label_frame.set_frame(FrameType::FlatBox);
        output_label_frame.set_color(Color::from_u32(0x1a0713));
        // output_label_frame.set_color(Color::from_rgb(0, 120, 254));
        output_label_frame.set_label_color(Color::from_u32(0xf5e0d0));
        output_label_frame.set_label_size(20);
        output_label_frame.set_label_font(Font::Helvetica);
    }

    output_label_frame
}
