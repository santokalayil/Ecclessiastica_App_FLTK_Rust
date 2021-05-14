
use fltk::{image::*,
    frame::Frame,
    group::Group,
    enums::Color,
    // enums::Align,
};
use fltk::prelude::*;
use std::fs::read;


pub struct LogoImage {
    group: fltk::group::Group,
    // button: fltk::button::Button,
    logo_frame: Frame,
}

impl LogoImage {
    pub fn new(x:i32, y:i32, w:i32, h:i32, label:&'static str) -> LogoImage {
        let mut group = Group::new(x, y, w, h, label);
        group.set_align(fltk::enums::Align::Center);
        group.set_frame(fltk::enums::FrameType::FlatBox);
        group.set_color(fltk::enums::Color::Red);

        let mut logo_frame = Frame::default()
        .with_size(group.width(), group.height())
        .center_of(&group)
        .with_label("H");
        logo_frame.set_frame(fltk::enums::FrameType::FlatBox);
        logo_frame.set_color(Color::Yellow);

        logo_frame.draw(|f| {
            // let image_data = read("assets\\app_icon.png").unwrap();
            let mut img = PngImage::load("assets\\app_icon.png").unwrap();
            img.scale(f.w(), f.h(), true, true);
            img.draw(f.x(), f.y(), f.w(), f.h());
        });
        
        group.end();
        LogoImage { group, logo_frame}
    }
}



// pub fn generate_logo(parent:fltk::group::Pack) -> fltk::group::Pack {
//     let pack = fltk::group::Pack::new(parent.x(), parent.y(), parent.width(), parent.height(), "");

//     let image_data = read("assets\\app_icon.png").unwrap(); // .replace("red", "green")
//     let mut img = PngImage::from_data(&image_data).unwrap();
//     img.scale(img.w()/10, img.h()/10, false,false);
//     println!("{}{}", img.w(), img.h());
//     // let mut svg = SvgImage::from_data(&image_data).unwrap();
//     // let mut svg = SvgImage::load("assets\\Ecclesiastica.svg")
    

//     let mut photo_frame = Frame::default()
//         .center_of(&pack)
//         .with_size(img.w(), img.h());
//     photo_frame.set_frame(fltk::enums::FrameType::FlatBox);
//     photo_frame.set_color(fltk::enums::Color::Red);

//     photo_frame.draw({
//         move |f| {
//             // img.scale(f.width(), f.height(), true, false);
//             img.draw(
//                 parent.x(),
//                 // (parent.x() as f32 + parent.width() as f32 / 2.8) as i32,
//                  parent.y() + 0, 
//                  f.width(), f.height()
//             );
//         }
//     });
//     pack.end();
//     pack
// }
