pub fn presets(&mut ) {
    egui::Window::new("Preset Manager")
        .open(&mut self.window_state.presets)
        .show(ctx, |ui| {
            // Save section
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.presets_manager.name);
                if ui.button("Save").clicked() && !self.presets_manager.name.is_empty() {
                    self.presets_manager.saved.insert(
                        self.presets_manager.name.clone(),
                        self.snarl_state.snarl.clone(),
                    );
                    self.presets_manager.selected = Some(self.presets_manager.name.clone());
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
                                if let Some(preset_snarl) = self.presets_manager.saved.get(preset) {
                                    self.snarl_state.snarl = preset_snarl.clone();
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
