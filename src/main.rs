use egui::{FontData, FontDefinitions};

fn main() {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert("NanumGodic".to_owned(), FontData::from_static(include_bytes!("")));
}
