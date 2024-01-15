use std::{fmt::format, fs::File};

use egui_phosphor::regular::*;
use log::info;

use crate::components::{Library, PVModule, Project};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PVApp {
    library: Library,
    project: Project,
}

impl PVApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for PVApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::SidePanel::right("library").show(ctx, |ui| {
            if ui.button("Add panel").clicked() {
                self.library.pv_modules.push(PVModule::default());
            }

            for (id, module) in self.library.pv_modules.iter_mut().enumerate() {
                ui.add(module);
                if ui.button("Add to project").clicked() {
                    self.project.pv_modules.push(id);
                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                if ui.button("Save").clicked() {
                    _ = serde_json::to_writer_pretty(
                        File::create("lib.json").unwrap(),
                        &self.library,
                    );
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // this is a bit inefficient, but readable. TODO: This should be a result struct that is calculated once.

            let res = self.project.sum(&self.library);

            ui.label(format!("You have {} modules installed.", self.project.pv_modules.len()));


            ui.label(format!("{:?}", res));
            ui.label(format!("Peak output: {:?} watts", res.energy_sum));
            ui.label(format!("Cost: {:?} Eur", res.price_sum));
            ui.label(format!("Area: {:?} sqm", res.area_sum / 10000.));
            
            ui.label(format!("Actual output: {:?} watts", res.energy_sum/1000. * self.project.yield_kwh_kwp));
            
            ui.add(egui::DragValue::new(&mut self.project.yield_kwh_kwp));

            egui::ComboBox::from_label("")
                .selected_text(format!("{CLOUD_SUN} Exposure"))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.project.yield_kwh_kwp, 1000., "Sunny");
                    ui.selectable_value(&mut self.project.yield_kwh_kwp, 400., "Light clouds");
                    ui.selectable_value(&mut self.project.yield_kwh_kwp, 150., "Heavy clouds");
                    ui.selectable_value(&mut self.project.yield_kwh_kwp, 50., "Rain");
                });

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/woelper/pvcalc",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
