pub mod windows{
    use egui::{Modifiers, Slider, Ui};


    #[derive(Clone)]
    pub struct SandboxWindow {

    }
    
    impl SandboxWindow {
        pub fn new() -> Self {
            Self {

            }
        }
    
    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        let _ = ctx;
            self.scene_settings(ui);
        
    }
        pub fn scene_settings(&mut self, ui: &mut Ui) {

        }
        
    }
    pub struct MainWindow<'a> {
        pub show_sandbox_window: bool,
        pub sandbox_window: &'a mut SandboxWindow,
    }
    
    impl<'a> MainWindow<'a> {
        pub fn new(sandbox_window: &'a mut SandboxWindow) -> Self {
            Self {
                show_sandbox_window: false,
                sandbox_window,
            }
        }
    
        pub fn ui(&mut self, ctx: &egui::Context) {
            self.desktop_ui(ctx);
        }
    
        pub fn desktop_ui(&mut self, ctx: &egui::Context) {
            egui::SidePanel::left("egui_demo_panel")
                .resizable(true)
                .default_width(250.0)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("✒ Vetracer Template");
                    });
                    ui.separator();
                    use egui::special_emojis::{GITHUB, TWITTER};
                    if self.show_sandbox_window {
                        egui::Window::new("Sandbox Window")
                            .resizable(true)
                            .default_width(400.0)
                            .show(ctx, |ui| {
                                self.sandbox_window.ui(ctx, ui);
                            });
                    }
                    ui.hyperlink_to(
                        format!("{GITHUB} Resource Code"),
                        "https://github.com/OmarDevX",
                    );
                    ui.separator();
                    self.demo_list_ui(ui);
                });
    
            egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    file_menu_button(ui);
                });
            });
        }
    
        pub fn demo_list_ui(&mut self, ui: &mut egui::Ui) {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    ui.label("Placeholder");
                    if ui.button("Default Window").clicked() {
                        self.show_sandbox_window = !self.show_sandbox_window;
                    }
            
                    if ui.button("Organize windows").clicked() {
                        ui.ctx().memory_mut(|mem| mem.reset_areas());
                    }
                });
            });
        }
        pub fn get_sandbox_window(&self) -> &SandboxWindow {
                return self.sandbox_window;
        }
    }
    
        pub fn file_menu_button(ui: &mut Ui) {
        let organize_shortcut =
            egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::O);
        let reset_shortcut =
            egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::R);
    
        // NOTE: we must check the shortcuts OUTSIDE of the actual "File" menu,
        // or else they would only be checked if the "File" menu was actually open!
    
        if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
            ui.ctx().memory_mut(|mem| mem.reset_areas());
        }
    
        if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
            ui.ctx().memory_mut(|mem| *mem = Default::default());
        }
    
        ui.menu_button("File", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
    
            // On the web the browser controls the zoom
            #[cfg(not(target_arch = "wasm32"))]
            {
                egui::gui_zoom::zoom_menu_buttons(ui);
                ui.weak(format!(
                    "Current zoom: {:.0}%",
                    100.0 * ui.ctx().zoom_factor()
                ))
                .on_hover_text("The UI zoom level, on top of the operating system's default value");
                ui.separator();
            }
    
            if ui
                .add(
                    egui::Button::new("Organize Windows")
                        .shortcut_text(ui.ctx().format_shortcut(&organize_shortcut)),
                )
                .clicked()
            {
                ui.ctx().memory_mut(|mem| mem.reset_areas());
                ui.close_menu();
            }
    
            if ui
                .add(
                    egui::Button::new("Reset egui memory")
                        .shortcut_text(ui.ctx().format_shortcut(&reset_shortcut)),
                )
                .on_hover_text("Forget scroll, positions, sizes etc")
                .clicked()
            {
                ui.ctx().memory_mut(|mem| *mem = Default::default());
                ui.close_menu();
            }
        });
    }

}
