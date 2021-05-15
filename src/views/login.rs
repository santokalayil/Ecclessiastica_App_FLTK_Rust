use fltk::{
    app,
    button::*,
    enums::{Align, Color, FrameType},
    frame::*,
    image::*,
    prelude::*,
    window::*,
}; //

use crate::views::logic;
use crate::views::elements;

// getting asset folder contents
#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

// use crate::views::logo::generate_logo;
use crate::views::logo::LogoImage;


pub fn login_window() {
    let window_x: i32 = 100;
    let window_y: i32 = 100;
    let window_width: i32 = 800;
    let window_height: i32 = 550;
    let window_title: &str = "Ecclesiastica v.01 - Login";
    let mut win = Window::new(window_x, window_y, window_width, window_height, None)
        .center_screen()
        .with_label(&window_title);

    let img = Assets::get("app_icon.png").unwrap(); // these 2 lines to embed if not use line after next line only
    let image = PngImage::from_data(&img).unwrap();

    win.set_icon(Some(image));

    let mut clone_win = win.clone();
    window_contents(&mut clone_win);

    win.make_resizable(true);
    win.end();
    win.show_with_args(&["-scheme", "+gtk", "-nokbd"]);
}

fn window_contents(win: &mut Window) {
    app::background(92, 26, 71);
    let mut frame = Frame::default().size_of(win);
    frame.set_frame(FrameType::FlatBox);

    // win.add(&frame);
    let top_banner_frame = elements::gen_output_label_frame(0, 0, frame.width(), 50);
    let close_button_width: i32 = 40;
    let mut close_button = Button::default()
        .with_pos(top_banner_frame.width()-close_button_width-5, 5)
        .with_size(close_button_width, 16)  
        .with_align(Align::Inside | Align::Center)
        .with_label("Logout");
    close_button.set_color(Color::from_u32(0xbb3333));
    close_button.set_frame(FrameType::RFlatBox);
    close_button.set_selection_color(Color::from_u32(0xff0000));
    close_button.set_label_font(fltk::enums::Font::Times);
    close_button.set_label_size(10);
    close_button.hide();



    // c============================ continue section starts here =============================
    // button to continue to main app section
    let mut continue_section = fltk::group::Pack::default()
    // .center_of(&frame)
    .with_pos(
        frame.x() + frame.width() / 4,
        frame.y() + top_banner_frame.height() + frame.height() / 6,
    )
    .with_size(frame.width() / 2, (frame.height()-top_banner_frame.height())/2);

    continue_section.set_spacing(30);
    continue_section.set_type(fltk::group::PackType::Vertical);
    let clone_continue_section = continue_section.clone();
    // let _logo = generate_logo(clone_continue_section);
    let logo = LogoImage::new(0, 0, 200, 200, "");

    // login_succesfull message frame that show
    let mut login_successful_frame = fltk::frame::Frame::default()
    .with_label("Welcome");
    login_successful_frame.set_label_color(Color::from_u32(0xffffff));
    login_successful_frame.set_label_size(22);
    login_successful_frame.set_frame(FrameType::FlatBox);

    let continue_button_width: i32 = 40;
    let mut continue_button = Button::default()
        // .with_pos(top_banner_frame.width()-continue_button_width-5, 5)
        .with_size(continue_button_width, 30)  
        .with_align(Align::Inside | Align::Center)
        .with_label("Continue");
    continue_button.set_frame(FrameType::RFlatBox);
    continue_button.set_color(Color::from_u32(0x00aa00));
    continue_button.set_selection_color(Color::from_u32(0x00ff00));
    continue_section.end();
    continue_section.hide();
    // c============================= continue section over =============================

    // login box frame section that contains username input, password input and login button
    let pack = fltk::group::Pack::default()
        // .center_of(&frame)
        .with_pos(
            frame.x() + frame.width() / 4,
            frame.y() + top_banner_frame.height() + frame.height() / 6,
        )
        .with_size(frame.width() / 2, 400);

    let mut username_label = Frame::new(0, 0, pack.width(), 30, "Username");
    username_label.set_frame(FrameType::FlatBox);
    username_label.set_label_color(Color::from_u32(0xffffff));

    let mut username = fltk::input::Input::new(0, 0, pack.width(), 30, "");
    // username.set_value(&format!("{}", ""));
    username.set_value(&format!("{}", "santokalayil"));
    username.set_color(Color::from_u32(0xedbedc));
    username.set_tooltip("Type your username here");
    username.set_frame(FrameType::ThinUpBox); // ShadowBox

    let mut password_label = Frame::new(0, 0, pack.width(), 30, "Password");
    password_label.set_frame(FrameType::FlatBox);
    password_label.set_label_color(Color::from_u32(0xffffff));

    let mut password = fltk::input::SecretInput::new(0, 0, pack.width(), 30, "");
    // password.set_value(&format!("{}", ""));
    password.set_value(&format!("{}", "hi"));
    password.set_color(Color::from_u32(0xedbedc));
    password.set_tooltip("Type your password here");
    password.set_frame(FrameType::ThinUpBox);
    password.set_trigger(fltk::enums::CallbackTrigger::EnterKeyAlways);
    

    let mut space_after_password = Frame::new(0, 0, pack.width(), 10, "");
    space_after_password.set_frame(FrameType::FlatBox);
    let mut login_button = elements::gen_login_button();
    let mut space_after_loginbtn = Frame::new(0, 0, pack.width(), 30, "");
    space_after_loginbtn.set_frame(FrameType::FlatBox);
    space_after_loginbtn.set_label_size(12);
    pack.end();

    let mut clone_close_button = close_button.clone();
    let mut clone_pack = pack.clone();
    let mut clone_continue_section = continue_section.clone();
    let mut clone_space_after_loginbtn = space_after_loginbtn.clone();
    let clone_username = username.clone();
    let clone_password = password.clone();

    // this will resized to match the windows size on successful login
    login_button.set_callback(move |_| {
        logic::check_login(
            &mut clone_space_after_loginbtn, 
            &mut clone_close_button, 
            &mut clone_pack, 
            &mut clone_continue_section, 
            &clone_username, 
            &clone_password
        );
    });

    let mut clone_space_after_loginbtn = space_after_loginbtn.clone();
    let mut clone_close_button = close_button.clone();
    let mut clone_pack = pack.clone();
    let mut clone_continue_section = continue_section.clone();
    // let mut clone_login_successful_frame = login_successful_frame.clone();
    let clone_username = username.clone();
    let clone_password = password.clone();
    password.set_callback(move |_| {
        logic::check_login(
            &mut clone_space_after_loginbtn, 
            &mut clone_close_button, 
            &mut clone_pack, 
            &mut clone_continue_section, 
            &clone_username, 
            &clone_password
        );
    });

    let mut clone_close_button = close_button.clone();
    let mut clone_win = win.clone();
    let mut clone_login_successful_frame = login_successful_frame.clone();
    close_button.set_callback(move |_| {
        clone_close_button.hide();
        clone_login_successful_frame.hide();
        space_after_loginbtn.set_label("");
        password.set_value("");
        username.set_value("");
        // pack.show();
        clone_win.clear();
        // clone_win.hide();
        // login_window();
        window_contents(&mut clone_win);
        // win.close();
    });










    win.add(&frame);
    win.add(&top_banner_frame);
    win.add(&pack);
    win.add(&close_button);
    // win.add(&login_successful_frame);
    win.add(&continue_section);
}
