use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewMode {
    FilePicker,
    Editor(PathBuf), // Owned, heap-allocated path
    Error(String),   // Owned, heap-allocated string
}

#[derive(Default)]
pub struct MagnificentApp {
    // dropped_files: Vec<egui::DroppedFile>,
    // picked_path: Option<String>,
    mode: ViewMode,
}

impl Default for ViewMode {
    fn default() -> Self {
        ViewMode::FilePicker
    }
}
impl MagnificentApp {
    fn draw_file_picker(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label("Drag-and-drop files onto the window!");

        if ui.button("Open file…").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.mode = ViewMode::Editor(path);
                log::info!("Mode: {:?}", self.mode);
            }
        }

        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                if !i.raw.dropped_files.is_empty() {
                    // Example: switch to editor mode with the first dropped file
                    if let Some(file) = i.raw.dropped_files.first() {
                        if let Some(path) = &file.path {
                            self.mode = ViewMode::Editor(path.clone());
                        } else {
                            self.mode = ViewMode::Error("Dropped file has no path!".to_string());
                        }
                    }
                }
            }
        });
    }

    fn draw_editor(&mut self, ui: &mut egui::Ui, path: &PathBuf) {
        // Check if the path has a valid extension
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|ext| ext.to_lowercase());

        if extension.is_none() {
            self.mode = ViewMode::Error(format!(
                "Invalid file path {:?}. The file must have a '.mp4' extension.",
                path
            ));
            return;
        }

        if extension.as_deref() != Some("mp4") {
            self.mode = ViewMode::Error(format!(
                "Invalid file extension for {:?}. The file must have a '.mp4' extension.",
                path
            ));
            return;
        }

        // Valid case, proceed with the editor UI
        if ui.button("Back to file picker").clicked() {
            self.mode = ViewMode::FilePicker;
        }
        ui.label(format!("Editing file: {}", path.display()));
    }

    fn draw_error(&mut self, ui: &mut egui::Ui, message: &str) {
        ui.colored_label(egui::Color32::RED, format!("Error: {}", message));
        if ui.button("Back to file picker").clicked() {
            self.mode = ViewMode::FilePicker;
        }
    }
}

impl eframe::App for MagnificentApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mode = self.mode.clone();
            match mode {
                ViewMode::FilePicker => {
                    self.draw_file_picker(ctx, ui);
                }
                ViewMode::Editor(path) => {
                    self.draw_editor(ui, &path);
                }
                ViewMode::Error(message) => {
                    self.draw_error(ui, &message);
                }
            }
        });
    }
}
