use egui::Vec2;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PVModule {
    pub brand: String,
    pub model: String,
    pub size_cm: Vec2,
    pub price_eur: f32,
    pub energy_wp: f32
}



#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Library {
    pv_modules: Vec<PVModule>,
}


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Project {
    pv_modules: Vec<PVModule>,
}
