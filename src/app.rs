use std::{collections::HashMap, fmt::format, fs::File};

use egui::{vec2, Rounding};
use egui_phosphor::regular::*;
use log::info;

use crate::{
    components::{compound_interest, Battery, Library, Panel, Project, Inverter},
    panel_orientation::efficiency,
    tr,
};

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

            fonts.font_data.insert(
                "my_font".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/Inter-Regular.ttf")),
            );

            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "my_font".to_owned());
            cc.egui_ctx.set_fonts(fonts);

            let mut style = (*cc.egui_ctx.style()).clone();
            style.text_styles = [
                (
                    egui::TextStyle::Heading,
                    egui::FontId::new(30.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(14.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Monospace,
                    egui::FontId::new(14.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(14.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Small,
                    egui::FontId::new(10.0, egui::FontFamily::Proportional),
                ),
            ]
            .into();
            cc.egui_ctx.set_style(style);

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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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
            ui.collapsing(format!("{RECTANGLE} Panels"), |ui| {
                let mut delete: Option<usize> = None;

                for (id, module) in self.library.panels.iter_mut().enumerate() {
                    ui.push_id(id, |ui| {
                        ui.add(module);
                    });
                    if ui.button(tr!("Hinzufuegen")).clicked() {
                        self.project.panels.push(id);
                    }
                    if ui.button(TRASH_SIMPLE).clicked() {
                        delete = Some(id);
                    }
                }

                if let Some(id) = delete {
                    self.library.panels.remove(id);
                }

                ui.separator();

                if ui
                    .add(
                        egui::Button::new(PLUS).rounding(Rounding::same(20.)), // .fill(ui.style().visuals.hyperlink_color)
                                                                               // .min_size(vec2(30., 30.)),
                    )
                    .clicked()
                {
                    self.library.panels.push(Panel::default());
                }
            });

            ui.collapsing(tr!("{RECTANGLE} Inverter"), |ui| {
                let mut delete: Option<usize> = None;

                for (id, inverter) in self.library.inverters.iter_mut().enumerate() {
                    ui.push_id(id, |ui| {
                        ui.add(inverter);
                    });
                    if ui.button(tr!("Hinzufuegen")).clicked() {
                        self.project.inverters.push(id);
                    }
                    if ui.button(TRASH_SIMPLE).clicked() {
                        delete = Some(id);
                    }
                }

                if let Some(id) = delete {
                    self.library.inverters.remove(id);
                }

                ui.separator();

                if ui
                    .add(
                        egui::Button::new(PLUS).rounding(Rounding::same(20.)), // .fill(ui.style().visuals.hyperlink_color)
                                                                               // .min_size(vec2(30., 30.)),
                    )
                    .clicked()
                {
                    self.library.inverters.push(Inverter::default());
                }
            });

            ui.collapsing(tr!("{BATTERY_FULL} Batterien"), |ui| {
                let mut delete: Option<usize> = None;

                for (id, battery) in self.library.batteries.iter_mut().enumerate() {
                    ui.push_id(id, |ui| {
                        ui.add(battery);
                    });
                    if ui.button(tr!("Hinzufuegen")).clicked() {
                        self.project.batteries.push(id);
                    }
                    if ui.button(TRASH_SIMPLE).clicked() {
                        delete = Some(id);
                    }
                }

                if let Some(id) = delete {
                    self.library.panels.remove(id);
                }

                ui.separator();

                if ui
                    .add(
                        egui::Button::new(PLUS).rounding(Rounding::same(20.)), // .fill(ui.style().visuals.hyperlink_color)
                                                                               // .min_size(vec2(30., 30.)),
                    )
                    .clicked()
                {
                    self.library.batteries.push(Battery::default());
                }
            });

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
            let res = self.project.sum(&self.library);

            ui.heading("Anlage");
            ui.label(tr!(
                "{} Panels installiert auf {} qm ",
                self.project.panels.len(),
                res.area_sum / 10000.
            ));
            ui.label(tr!("Leistung Peak: {:?} kWp", res.energy_sum_wp));

            ui.horizontal(|ui| {
                ui.label(tr!("Installationskosten Panels"));
                ui.add(
                    egui::DragValue::new(&mut self.project.price_installation_panels)
                        .speed(0.1)
                        .suffix(" €"),
                );
            });

            ui.horizontal(|ui| {
                ui.label(tr!("Installationskosten Elektrik"));
                ui.add(
                    egui::DragValue::new(&mut self.project.price_installation_electricity)
                        .speed(0.1)
                        .suffix(" €"),
                );
            });

            ui.label(tr!("Gesamtkosten: {:?} €", res.price_sum));

            ui.horizontal(|ui| {
                ui.label(tr!("Globalstrahlung"));
                ui.add(egui::DragValue::new(&mut self.project.yield_kwh_kwp));
                egui::ComboBox::from_label("")
                    .selected_text(tr!("{CLOUD_SUN} Exposure"))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.project.yield_kwh_kwp, 1000., "Sunny");
                        ui.selectable_value(&mut self.project.yield_kwh_kwp, 400., "Light clouds");
                        ui.selectable_value(&mut self.project.yield_kwh_kwp, 150., "Heavy clouds");
                        ui.selectable_value(&mut self.project.yield_kwh_kwp, 50., "Rain");
                    });
            });

            ui.horizontal(|ui| {
                ui.label(tr!("Ausrichtung (Abweichung von Sueden)"));
                ui.add(
                    egui::DragValue::new(&mut self.project.panel_orientation)
                        .clamp_range(0.0..=180.)
                        .speed(0.1)
                        .suffix(" Grad"),
                );
            });

            ui.horizontal(|ui| {
                ui.label(tr!("Neigungswinkel (0 = flach)"));
                ui.add(
                    egui::DragValue::new(&mut self.project.panel_angle_deg)
                        .clamp_range(0.0..=90.)
                        .speed(0.1)
                        .suffix(" Grad"),
                );
            });
            ui.label(tr!(
                "Effizienz: {}",
                efficiency(self.project.panel_orientation, self.project.panel_angle_deg)
            ));
            ui.separator();

            ui.heading("Markt");
            ui.horizontal(|ui| {
                ui.label(tr!("Preis pro kWh"));
                ui.add(
                    egui::DragValue::new(&mut self.project.price_kwh_eur_buy)
                        .speed(0.01)
                        .suffix(" €"),
                );
            });

            ui.horizontal(|ui| {
                ui.label(tr!("Einspeiseverguetung pro kWh"));
                ui.add(
                    egui::DragValue::new(&mut self.project.price_kwh_eur_sell)
                        .speed(0.01)
                        .suffix(" €"),
                );
            });

            ui.horizontal(|ui| {
                ui.label(tr!("Zins Festgeld"));
                ui.add(
                    egui::DragValue::new(&mut self.project.interest_rate_deposit)
                        .speed(0.01)
                        .suffix(" %/100"),
                );
            });

            ui.horizontal(|ui| {
                ui.label(tr!("Verbrauch kWh/Jahr"));
                ui.add(egui::DragValue::new(&mut self.project.consumption_kwh).suffix(" kWh"));
                egui::ComboBox::from_id_source("v")
                    .selected_text(tr!("{CLOUD_SUN} Verbrauch"))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.project.consumption_kwh,
                            1500.,
                            tr!("1 Person"),
                        );
                        ui.selectable_value(
                            &mut self.project.consumption_kwh,
                            2500.,
                            tr!("2 Personen"),
                        );
                        ui.selectable_value(
                            &mut self.project.consumption_kwh,
                            3500.,
                            tr!("3 Personen"),
                        );
                        ui.selectable_value(
                            &mut self.project.consumption_kwh,
                            4250.,
                            tr!("4 Personen"),
                        );
                    });
            });

            ui.heading(tr!("Analyse"));
            ui.separator();

            let efficiency =
                efficiency(self.project.panel_orientation, self.project.panel_angle_deg);
            let yield_year_kwh =
                (res.energy_sum_wp / 1000. * self.project.yield_kwh_kwp) * efficiency;
            ui.label(tr!("Ertrag pro Jahr: {:?} kWh", yield_year_kwh));

            let regular_energy_cost = self.project.consumption_kwh * self.project.price_kwh_eur_buy;
            ui.label(tr!(
                "Stromkosten pro Jahr bei ausschliesslicher Netznutzung: {regular_energy_cost} €"
            ));

            // how much of the power we generate can be self-used
            let consumption_covered = yield_year_kwh.min(self.project.consumption_kwh);
            let amount_to_sell = (yield_year_kwh - self.project.consumption_kwh).max(0.0);

            let combined_benefit = consumption_covered * self.project.price_kwh_eur_buy
                + amount_to_sell * self.project.price_kwh_eur_sell;

            ui.label(tr!("Gesamteinnahmen {}", combined_benefit));
            ui.label(tr!(
                "Amortisiert nach {:.1} Jahren",
                res.price_sum / combined_benefit
            ));
            let alternative_investment = compound_interest(
                res.price_sum,
                self.project.interest_rate_deposit,
                res.price_sum / combined_benefit,
            );
            ui.label(tr!(
                "Alternativ: Investitionssumme verzinsen: {:.0}€ ({:.0}€ mehr)",
                alternative_investment,
                alternative_investment - res.price_sum
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if ui.button("Reset").clicked() {
                    self.project = Default::default();
                }
                ui.label(tr!("Alle Angeben ohne Gewaehr!"));
                ui.add(egui::github_link_file!(
                    "https://github.com/woelper/pvcalc",
                    "Source code."
                ));
            });
        });
    }
}
