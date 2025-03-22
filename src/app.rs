use crate::nodes::{NodeViewer, Nodes};
use egui::Id;
use egui_snarl::Snarl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    /// The snarl grapth to display
    snarl: Snarl<Nodes>,
    /// The optional ID of the snarl UI element
    snarl_ui_id: Option<Id>,
    /// Presets manager
    presets_manager: PresetsManager,
    /// Window states
    #[serde(skip)]
    window_states: WindowStates,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct PresetsManager {
    /// Field to store the preset name for saving
    name: String,
    /// Map of preset name to the snarl (list of nodes) snapshot
    saved: HashMap<String, Snarl<Nodes>>,
    /// Currently selected preset for loading
    selected: Option<String>,
}

#[derive(Default)]
pub struct WindowStates {
    presets: bool,
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
                if ui.button("Presets").clicked() {
                    self.window_states.presets = true;
                }
            });
        });

        if self.window_states.presets {
            egui::Window::new("Preset Manager")
                .open(&mut self.window_states.presets)
                .show(ctx, |ui| {
                    // Save section
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.presets_manager.name);
                        if ui.button("Save").clicked() {
                            if !self.presets_manager.name.is_empty() {
                                self.presets_manager
                                    .saved
                                    .insert(self.presets_manager.name.clone(), self.snarl.clone());
                                self.presets_manager.selected =
                                    Some(self.presets_manager.name.clone());
                            }
                        }
                    });

                    ui.separator();

                    // Load and Delete section
                    ui.horizontal(|ui| {
                        ui.label("Preset:");
                        egui::ComboBox::from_label("")
                            .selected_text(
                                self.presets_manager
                                    .selected
                                    .as_deref()
                                    .unwrap_or("Select a preset"),
                            )
                            .show_ui(ui, |ui| {
                                for preset in self.presets_manager.saved.keys() {
                                    if ui
                                        .selectable_value(
                                            &mut self.presets_manager.selected,
                                            Some(preset.clone()),
                                            preset,
                                        )
                                        .clicked()
                                    {
                                        if let Some(preset_snarl) =
                                            self.presets_manager.saved.get(preset)
                                        {
                                            self.snarl = preset_snarl.clone();
                                            self.presets_manager.name = preset.clone();
                                        }
                                    }
                                }
                            });
                        if ui.button("Delete").clicked() {
                            if let Some(ref name) = self.presets_manager.selected {
                                self.presets_manager.saved.remove(name);
                                self.presets_manager.selected = None;
                            }
                        }
                    });
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.snarl_ui_id = Some(ui.id());
            self.snarl
                .show(&mut NodeViewer, &crate::nodes::snarl_style(), "snarl", ui);
        });
    }
}
