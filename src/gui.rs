use eframe::egui;

pub fn launch_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([350.0, 250.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Vencode UI",
        options,
        Box::new(|_cc| Box::new(RecordingApp::default())),
    )
}

#[derive(Default)]
struct RecordingApp {
    is_recording: bool,
    last_error: Option<String>,
    status_message: Option<String>,
}

impl eframe::App for RecordingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Add some padding and vertical centering
            ui.add_space(20.0);
            
            // Center the content horizontally
            ui.vertical_centered(|ui| {
                // Title with larger font
                ui.heading("🎥 Vencode Screen Recorder");
                ui.add_space(30.0);
                
                // Status indicator
                let status_text = if self.is_recording {
                    "🔴 Recording in progress..."
                } else {
                    "⚫ Ready to record"
                };
                
                ui.label(egui::RichText::new(status_text).size(16.0));
                ui.add_space(20.0);
                
                // Main action button - properly sized and styled
                let button_text = if self.is_recording {
                    "⏹ Stop Recording"
                } else {
                    "▶ Start Recording"
                };
                
                let button_color = if self.is_recording {
                    egui::Color32::from_rgb(220, 50, 50) // Red for stop
                } else {
                    egui::Color32::from_rgb(50, 150, 50) // Green for start
                };
                
                let button = egui::Button::new(egui::RichText::new(button_text).size(18.0))
                    .fill(button_color)
                    .min_size(egui::vec2(200.0, 50.0));
                
                if ui.add(button).clicked() {
                    self.handle_recording_toggle();
                }
                
                ui.add_space(20.0);
                
                // Error display
                if let Some(error) = &self.last_error {
                    ui.colored_label(
                        egui::Color32::RED,
                        format!("❌ Error: {}", error)
                    );
                }
                
                // Status message display
                if let Some(message) = &self.status_message {
                    ui.colored_label(
                        egui::Color32::from_rgb(0, 150, 0),
                        format!("✅ {}", message)
                    );
                }
            });
            
            // Footer with instructions
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Click the button to start/stop screen recording")
                    .size(12.0)
                    .color(egui::Color32::GRAY));
            });
        });

        // Only repaint when recording for better performance
        if self.is_recording {
            ctx.request_repaint_after(std::time::Duration::from_millis(1000));
        }
    }
}

impl RecordingApp {
    fn handle_recording_toggle(&mut self) {
        // Clear previous messages
        self.last_error = None;
        self.status_message = None;
        
        if !self.is_recording {
            // Start recording
            match crate::recorder::start_recording() {
                Ok(_) => {
                    self.is_recording = true;
                    self.status_message = Some("Recording started successfully!".to_string());
                }
                Err(e) => {
                    self.last_error = Some(format!("Failed to start recording: {}", e));
                    eprintln!("Failed to start recording: {}", e);
                }
            }
        } else {
            // Stop recording
            match crate::recorder::stop_recording() {
                Ok(_) => {
                    self.is_recording = false;
                    self.status_message = Some("Recording stopped and saved!".to_string());
                }
                Err(e) => {
                    self.last_error = Some(format!("Failed to stop recording: {}", e));
                    self.is_recording = false; // Reset state even on error
                    eprintln!("Failed to stop recording: {}", e);
                }
            }
        }
    }
}