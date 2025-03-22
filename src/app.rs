use crate::nodes::{NodeViewer, Nodes};
use egui::Id;
use egui_snarl::Snarl;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    snarl: Snarl<Nodes>,
    snarl_ui_id: Option<Id>,
    // #[serde(skip)]
    // value: f32,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        cc.egui_ctx.style_mut(|style| style.animation_time *= 1.5);
        cc.egui_ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            ..Default::default()
        });

        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:
        //
        //     egui::menu::bar(ui, |ui| {
        //         let is_web = cfg!(target_arch = "wasm32");
        //         if !is_web {
        //             // egui::widgets::global_theme_preference_switch(ui);
        //
        //             ui.menu_button("File", |ui| {
        //                 if ui.button("Quit").clicked() {
        //                     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        //                 }
        //             });
        //             ui.add_space(16.0);
        //         }
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.snarl_ui_id = Some(ui.id());
            self.snarl
                .show(&mut NodeViewer, &crate::nodes::snarl_style(), "snarl", ui);
        });
    }
}
