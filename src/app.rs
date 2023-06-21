use std::fs::OpenOptions;
use std::net::ToSocketAddrs;
use std::ops::Deref;
use std::os::unix::raw::uid_t;

use eframe::egui;
use eframe::egui::*;

use crate::file_parsing;
use crate::url_pool::UrlPool;
use crate::url_struct::UrlData;

#[derive(Default)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct UrlOrganizerApp {
    urls: UrlPool,
    loaded_file: Option<String>,
    saving_dialog: bool,
    maximised: bool,
    minimized: bool,
    name_to_save: String,
}


impl UrlOrganizerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for UrlOrganizerApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let Self { urls, loaded_file, requesting_input, maximised , minimized} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        self.saving_dialog = true;
                        self.name_to_save = "".to_string();
                        ui.close_menu()
                    }
                    if ui.button("load").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let string_path = path.display().to_string();
                            self.loaded_file = Some(string_path.clone());
                            match file_parsing::parse_random_file(&string_path) {
                                Some(u) => self.urls = u,
                                None => self.loaded_file = None
                            }
                            // println!("{}", self.urls.get_all_urls().len());
                            ui.close_menu()
                        }
                    }
                });
                // add normal window movement
                ui.with_layout(egui::Layout::right_to_left(Default::default()), |ui| {
                    if ui.button("x").clicked() {
                        _frame.close();
                    }
                    if ui.button("□").clicked() {
                        if !self.maximised {
                            _frame.set_maximized(true);
                            self.maximised = true
                        } else {
                            _frame.set_maximized(false);
                            self.maximised = false
                        }
                    }
                    if ui.button("_").clicked() {
                        if !self.minimized {
                            _frame.set_minimized(true);
                            self.minimized = true
                        } else {
                            _frame.set_minimized(false);
                            self.minimized = false
                        }
                    }
                })
            });
            if self.saving_dialog {
                egui::Window::new("input filename")
                    .interactable(false)
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2{ x: 0.0, y: 0.0 })
                    .show(ctx, |ui| {
                        ui.label("give name of file to be saved to ");
                        ui.add(egui::TextEdit::singleline(&mut self.name_to_save).desired_width(120.0));
                        // ui.text_edit_singleline(&mut filename);

                    });
                if ctx.input(|i| i.key_pressed(Key::Escape)){
                    self.saving_dialog = false;
                }
                if ctx.input(|i| i.key_pressed(Key::Enter)){
                    if !self.name_to_save.is_empty() {
                        file_parsing::save_to_file(self.name_to_save.as_str(), &self.urls).expect("file save failed");
                        ui.label("file saved to: {filename}");
                        self.saving_dialog = false;
                    } else {
                        ui.label("empty file names are not allowed");
                        // self.requesting_input = false;
                    }
                }
            }
        });

        // egui::SidePanel::left("side_panel").show(ctx, |ui| {
        //     ui.heading("Side Panel");
        //
        //     ui.horizontal(|ui| {
        //         ui.label("Write something: ");
        //         ui.text_edit_singleline(label);
        //     });
        //
        //     ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
        //     if ui.button("Increment").clicked() {
        //         *value += 1.0;
        //     }
        //
        //     ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //         ui.horizontal(|ui| {
        //             ui.spacing_mut().item_spacing.x = 0.0;
        //             ui.label("powered by ");
        //             ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        //             ui.label(" and ");
        //             ui.hyperlink_to(
        //                 "eframe",
        //                 "https://github.com/emilk/egui/tree/master/crates/eframe",
        //             );
        //             ui.label(".");
        //         });
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("url organizer");
            let mut scroll_area = egui::ScrollArea::vertical()
                .max_height(f32::INFINITY)
                .auto_shrink([false; 2]);
            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let url_list = self.urls.get_all_urls();
            let num_rows = url_list.len();
            let pad_size: usize = 30;
            if num_rows > 0 {
                ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
                    ui,
                    row_height,
                    num_rows,
                    |ui, row_range| {
                        for url_blob in url_list {

                            ui.horizontal(|ui| {
                                let name_clicked = ui.selectable_label(false, format_string(&url_blob.name, pad_size));
                                ui.separator();
                                let url_clicked = ui.selectable_label(false ,format_string(&url_blob.url, pad_size));
                                ui.separator();
                                let tag_clicked = ui.selectable_label(false ,
                                    format_string(&url_blob.tags.iter()
                                        .map(|s| &**s)
                                        .collect::<Vec<&str>>()
                                        .join(", "), 2*pad_size)
                                );
                                if name_clicked.clicked(){
                                    ui.output(()).copied_text = String::from(&url_blob.name);
                                }
                                if url_clicked.clicked(){
                                    ui.output(()).copied_text = String::from(&url_blob.url);
                                }
                                if tag_clicked.clicked(){
                                    ui.output(()).copied_text = String::from(&url_blob.tags);
                                }
                            });

                        }
                    },
                );
            }


            // let (current_scroll, max_scroll) = scroll_area
            //     .show(ui, |ui| {
            //
            //         // ui.vertical(|ui| {
            //         //     for item in 1..=50 {
            //         //         if item == self.track_item {
            //         //             let response =
            //         //                 ui.colored_label(egui::Color32::YELLOW, format!("This is item {}", item));
            //         //             response.scroll_to_me(self.tack_item_align);
            //         //         } else {
            //         //             ui.label(format!("This is item {}", item));
            //         //         }
            //         //     }
            //         // });
            //
            //         let margin = ui.visuals().clip_rect_margin;
            //
            //         let current_scroll = ui.clip_rect().top() - ui.min_rect().top() + margin;
            //         let max_scroll = ui.min_rect().height() - ui.clip_rect().height() + 2.0 * margin;
            //         (current_scroll, max_scroll)
            //     })
            //     .inner;
            ui.separator();

            // ui.label(format!(
            //     "Scroll offset: {:.0}/{:.0} px",
            //     current_scroll, max_scroll
            // ));


            // match &self.loaded_file {
            //     Some(s) => {
            //         ui.label(s.rsplit_once('/').unwrap().1);
            //     }
            //     None => {}
            //     _ => {}
            // };
            // ui.hyperlink("https://github.com/emilk/eframe_template");
            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/master/",
            //     "Source code."
            // ));
            egui::warn_if_debug_build(ui);
        });

        // if false {
        //     egui::Window::new("Window").show(ctx, |ui| {
        //         ui.label("Windows can be moved by dragging them.");
        //         ui.label("They are automatically sized based on contents.");
        //         ui.label("You can turn on resizing and scrolling if you like.");
        //         ui.label("You would normally choose either panels OR windows.");
        //     });
        // }
    }

    // /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }
}

fn format_string(s: &String, length: usize) -> String {
    let fs: String;
    if s.len() <= length{
        fs = format!("{:length$}", s.clone())
    }else{
        fs = s[..length+1].to_string()
    }
    fs
}