use std::fs::File;

use egui::{Vec2, Widget};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PVModule {
    pub brand: String,
    pub model: String,
    pub size_cm: Vec2,
    pub price_eur: f32,
    pub energy_wp: f32,
}

impl Widget for &mut PVModule {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Grid::new("Module")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Brand");
                ui.text_edit_singleline(&mut self.brand);
                ui.end_row();
                ui.label("Model");
                ui.text_edit_singleline(&mut self.model);
                ui.end_row();
                ui.label("Size (W x H)");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut self.size_cm.x).suffix(" cm"));
                    ui.add(egui::DragValue::new(&mut self.size_cm.y).suffix(" cm"));
                });
                ui.end_row();
                ui.label("Price");
                ui.add(egui::DragValue::new(&mut self.price_eur).suffix(" Eur"));
                ui.end_row();
                ui.label("Energy output");
                ui.add(egui::DragValue::new(&mut self.energy_wp).suffix(" wp"));
                ui.end_row();
            })
            .response
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    pub pv_modules: Vec<PVModule>,
}

impl Default for Library {
    fn default() -> Self {
        serde_json::from_str(include_str!("../lib.json")).expect("Library must load")
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Project {
    pub pv_modules: Vec<usize>,
    /// Specific yield (regional / time based)
    pub yield_kwh_kwp: f32,
}

impl Project {
    pub fn sum(&self, library: &Library) -> ProjectResult {
        self.pv_modules
            .iter()
            .map(|id| library.pv_modules.get(*id))
            .filter_map(|x| x)
            .fold(ProjectResult::default(), |acc, p| ProjectResult {
                energy_sum: acc.energy_sum + p.energy_wp,
                price_sum: acc.price_sum + p.price_eur,
                area_sum: acc.area_sum + p.size_cm.x * p.size_cm.y,
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]

pub struct ProjectResult {
    pub energy_sum: f32,
    pub price_sum: f32,
    pub area_sum: f32,
}

fn wp_to_kwh(kwp: f32) -> f32 {
    1.
}
