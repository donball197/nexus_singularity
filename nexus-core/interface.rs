use eframe::egui;

pub fn launch_ui() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "NEXUS ORACLE",
        options,
        Box::new(|_cc| Ok(Box::new(NexusApp::default()))),
    )
}

struct NexusApp {
    connection_status: String,
}

impl Default for NexusApp {
    fn default() -> Self {
        Self { connection_status: "LISTENING PORT 7340".to_owned() }
    }
}

impl eframe::App for NexusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🏛️ Nexus Singularity v0.2.2");
            ui.separator();
            ui.label(format!("Oracle Status: {}", self.connection_status));
            
            if ui.button("Clean Slate (pkill)").clicked() {
                // Future: Hook into your pkill logic
                self.connection_status = "RESTARTING...".to_owned();
            }
        });
    }
}
