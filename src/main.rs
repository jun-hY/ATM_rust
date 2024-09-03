use eframe::egui;
use egui::{vec2, FontData, FontDefinitions, FontFamily};

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "JJerrrryTM",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
}

#[derive(Default)]
struct MyEguiApp {
    btn_clicked: bool,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "MaruBuri".to_owned(),
            FontData::from_static(include_bytes!("../font/MaruBuri-Regular.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MaruBuri".to_owned());

        cc.egui_ctx.set_fonts(fonts);
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if !self.btn_clicked {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("안녕");
                if ui
                    .add_sized(vec2(120., 80.), egui::Button::new("다음"))
                    .clicked()
                {
                    self.btn_clicked = true;
                }
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("버튼 눌림");
                if ui
                    .add_sized(vec2(120., 80.), egui::Button::new("돌아가기"))
                    .clicked()
                {
                    self.btn_clicked = false;
                }
            });
        }
    }
}
