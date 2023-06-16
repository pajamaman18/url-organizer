use eframe::{App, Frame, run_native};
use eframe::egui::Context;

use crate::app::UrlOrganizerApp;
use crate::url_pool::UrlPool;

mod url_pool;
mod file_parsing;
mod url_struct;
mod app;

// impl App for UrlPool {
//     fn update(&mut self, ctx: &Context, frame: &mut Frame) {
//         todo!()
//     }
// }


fn main() -> eframe::Result<()> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "url-organizer",
        native_options,
        Box::new(|cc| Box::new(UrlOrganizerApp::new(cc))),)
    // let all_urls:UrlPool = file_parsing::parse_files_into_data("src/url_files/");
    // file_parsing::save_to_file("test-file", &all_urls).expect("saving to file errored");
    // // println!("{:?}", read_from_parsed_file("test-file"))
    // let parsed_urls = all_urls.get_url_urls("google");
    // println!("{:?}", parsed_urls);
    }
