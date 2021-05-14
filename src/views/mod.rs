use fltk::{
    app,
    // // app::sleep,
    // button::*,
    // enums::{Align, Color, FrameType},
    // frame::*,
    // image::*,
    // prelude::*,
    // window::*,
}; //


mod elements; // accessing elements folder
mod logic;
mod logo;
pub mod login;

// getting asset folder contents
#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

pub fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default(); //.with_scheme(AppScheme::Gtk)
    app::set_visible_focus(false);
    // app::background(92, 26, 71); // this is the app module imported and not he app

    login::login_window();

    // theming
    

    app.run().unwrap();
    Ok(())
}
