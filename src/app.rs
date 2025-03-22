use crate::nodes::{NodeViewer, Nodes};
use egui::Id;
use egui_snarl::Snarl;
use std::collections::HashMap;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    snarl: Snarl<Nodes>,
    snarl_ui_id: Option<Id>,
    // Field to store the preset name for saving
    preset_name: String,
    // Map of preset name to the snarl (list of nodes) snapshot
    available_presets: HashMap<String, Snarl<Nodes>>,
    // Currently selected preset for loading
    selected_preset: Option<String>,
}

impl App {
    /// Called once before the first frame
    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        cc.egui_ctx.style_mut(|style| style.animation_time *= 1.5);
        cc.egui_ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            ..Default::default()
        });

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Preset name:");
                    ui.text_edit_singleline(&mut self.preset_name);
                    if ui.button("Save preset").clicked() {
                        if !self.preset_name.is_empty() {
                            self.available_presets
                                .insert(self.preset_name.clone(), self.snarl.clone());
                        }
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Load preset:");
                    let combo_id = ui.make_persistent_id("load_preset_combo");
                    egui::ComboBox::new(combo_id, "")
                        .selected_text(self.selected_preset.as_deref().unwrap_or("Select a preset"))
                        .show_ui(ui, |ui| {
                            for preset in self.available_presets.keys() {
                                ui.selectable_value(
                                    &mut self.selected_preset,
                                    Some(preset.clone()),
                                    preset,
                                );
                            }
                        });
                    if ui.button("Load preset").clicked() {
                        if let Some(ref name) = self.selected_preset {
                            if let Some(preset_snarl) = self.available_presets.get(name) {
                                // Clone the loaded preset back into snarl.
                                self.snarl = preset_snarl.clone();
                            }
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.snarl_ui_id = Some(ui.id());
            self.snarl
                .show(&mut NodeViewer, &crate::nodes::snarl_style(), "snarl", ui);
        });
    }
}
