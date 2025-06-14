use eframe::egui;

pub fn launch_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Vencode UI",
        options,
        Box::new(|_cc| Box::new(RecordingApp::default())),
    )
}

#[derive(Default)]
struct RecordingApp {
    is_recording: bool,
}

impl eframe::App for RecordingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎥 Vencode Screen Recorder");

            if !self.is_recording {
                if ui.button("▶ Start Recording").clicked() {
                    match crate::recorder::start_recording() {
                        Ok(_) => self.is_recording = true,
                        Err(e) => {
                            eprintln!("Failed to start recording: {e}");
                        }
                    }
                }
            } else {
                if ui.button("⏹ Stop Recording").clicked() {
                    match crate::recorder::stop_recording() {
                        Ok(_) => self.is_recording = false,
                        Err(e) => {
                            eprintln!("Failed to stop recording: {e}");
                        }
                    }
                }
            }
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}
