
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
            let mut img = PngImage::load( std::path::Path::new("assets/app_icon.png")).unwrap();
            img.scale(f.w(), f.h(), true, true);
            img.draw(f.x() + f.w()/2 - img.width()/2,
                             f.y() + f.h()/2 - img.height()/2,
                             img.width(), 
                             img.height());
         });
        
        group.end();
        LogoImage { group, logo_frame}
    }
}



// logo_frame.draw(|f| {
//     let mut img = PngImage::load( std::path::Path::new("assets/app_icon.png")).unwrap();
//     img.scale(f.w(), f.h(), true, true);
//     img.draw(f.x() + f.w()/2 - img.width()/2,
//                      f.y() + f.h()/2 - img.height()/2,
//                      img.width(), 
//                      img.height());
//  });
