use egui::{Vec2, Widget};
use serde::{Deserialize, Serialize};

use crate::tr;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(default)]
pub struct Panel {
    pub brand: String,
    pub model: String,
    pub size_cm: Vec2,
    pub price_eur: f32,
    pub energy_wp: f32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(default)]
pub struct Battery {
    pub brand: String,
    pub model: String,
    pub price_eur: f32,
    pub energy_ahr: f32,
    pub voltage: f32,
}

impl Widget for &mut Battery {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Grid::new("batt")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label(tr!("Marke"));
                ui.text_edit_singleline(&mut self.brand);
                ui.end_row();
                ui.label(tr!("Modell"));
                ui.text_edit_singleline(&mut self.model);
                ui.end_row();
                ui.label(tr!("Preis"));
                ui.add(egui::DragValue::new(&mut self.price_eur).suffix(" Eur"));
                ui.end_row();
                ui.label(tr!("Leistung"));
                ui.add(egui::DragValue::new(&mut self.energy_ahr).suffix(" ahr"));
                ui.end_row();
                ui.label(tr!("Spannung"));
                ui.add(egui::DragValue::new(&mut self.voltage).suffix(" V"));
                ui.end_row();
            })
            .response
    }
}

impl Widget for &mut Panel {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Grid::new("panel")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label(tr!("Marke"));
                ui.text_edit_singleline(&mut self.brand);
                ui.end_row();
                ui.label(tr!("Modell"));
                ui.text_edit_singleline(&mut self.model);
                ui.end_row();
                ui.label(tr!("Groesse (W x H)"));
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
/// The library holds all things you can use in your project.
pub struct Library {
    pub panels: Vec<Panel>,
    #[serde(default)]
    pub batteries: Vec<Battery>,
}

impl Default for Library {
    fn default() -> Self {
        serde_json::from_str(include_str!("../lib.json")).expect("Library must load")
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Project {
    pub panels: Vec<usize>,
    pub batteries: Vec<usize>,
    /// Specific yield (regional / time based)
    pub yield_kwh_kwp: f32,
    pub consumption_kwh: f32,
    pub price_kwh_eur_buy: f32,
    pub price_kwh_eur_sell: f32,
    /// how much the panel deviates from right-angle to the sun. 0=facing sun
    pub panel_angle_deg: f32,
    /// 0-180. how much the panel deviates from facing south. 0=facing south
    pub panel_orientation: f32,
    pub interest_rate_deposit: f32
}

impl Default for Project {
    fn default() -> Self {
        Self {
            panels: Default::default(),
            batteries: Default::default(),
            yield_kwh_kwp: 1000.0,
            consumption_kwh: 2500.0,
            price_kwh_eur_buy: 0.4229,
            price_kwh_eur_sell: 0.082,
            panel_angle_deg: 0.0,
            panel_orientation: 0.0,
            interest_rate_deposit: 0.04
        }
    }
}

impl Project {
    pub fn sum(&self, library: &Library) -> ProjectResult {
        self.panels
            .iter()
            .map(|id| library.panels.get(*id))
            .filter_map(|x| x)
            .fold(ProjectResult::default(), |acc, p| ProjectResult {
                energy_sum_wp: acc.energy_sum_wp + p.energy_wp,
                price_sum: acc.price_sum + p.price_eur,
                area_sum: acc.area_sum + p.size_cm.x * p.size_cm.y,
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct ProjectResult {
    pub energy_sum_wp: f32,
    pub price_sum: f32,
    pub area_sum: f32,
}

fn wp_to_kwh(kwp: f32) -> f32 {
    1.
}

pub fn compound_interest(start_capital:f32, interest: f32, years: f32) -> f32 {
    start_capital * (1.0+interest).powf(years)
}