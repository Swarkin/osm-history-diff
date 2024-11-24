use crate::osm;
use eframe::egui;
use eframe::egui::{Align, Layout};
use egui_extras::{Column, TableBuilder};

pub struct TemplateApp {
	element_id_text: String,
	element_id: u32,
	element_type: osm::ElementType,
	json: serde_json::Value,
	json_str: String,

	data: Option<osm::ElementHistory>,
}

impl Default for TemplateApp {
	fn default() -> Self {
		Self {
			element_id_text: String::from("33538067"),
			element_id: 33538067,
			element_type: osm::ElementType::Way,
			json: serde_json::json!({}),
			json_str: String::new(),
			data: None,
		}
	}
}

impl TemplateApp {
	pub fn new(_cc: &eframe::CreationContext) -> Self {
		Default::default()
	}
}

impl eframe::App for TemplateApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
					egui::widgets::global_theme_preference_buttons(ui);
				});
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("OSM Object ID:");
				ui.text_edit_singleline(&mut self.element_id_text);
				if let Ok(n) = self.element_id_text.parse::<u32>() {
					self.element_id = n;
				}
			});

			if ui.button("Request").clicked() {
				let resp = ureq::get(format!("https://api.openstreetmap.org/api/0.6/{}/{}/history.json", self.element_type, self.element_id)).call().unwrap();
				let mut json = resp.into_body().read_json::<serde_json::Value>().unwrap();
				self.json = json["elements"].take();
				self.json_str = serde_json::ser::to_string_pretty(&self.json).unwrap();

				let mut history = osm::ElementHistory::new();
				for entry in self.json.as_array().unwrap() {
					let entry = entry.as_object().unwrap();
					history.push(osm::HistoryEntry {
						r#type: osm::ElementType::Way,
						version: entry["version"].as_i64().unwrap() as u32,
						tags: {
							// convert serde_json::Map to Vec<osm_primitives::Tag>
							let source_tags = entry["tags"].as_object().unwrap();
							let mut tags = Vec::<osm_primitives::Tag>::with_capacity(source_tags.len());
							for (k, v) in source_tags {
								tags.push(osm_primitives::Tag { key: k.to_owned(), value: v.as_str().unwrap().to_string() });
							}
							tags
						},
					});
				}

				self.data = Some(history);
			}

			ui.collapsing("raw", |ui| {
				egui::ScrollArea::vertical().show(ui, |ui| {
					ui.monospace(&self.json_str);
				});
			});

			if let Some(history) = &self.data {
				egui::ScrollArea::both().show(ui, |ui| {
					ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
						for entry in history {
							ui.label(entry.version.to_string());

							ui.group(|ui| {
								ui.with_layout(Layout::top_down(Align::TOP), |ui| {
									ui.push_id(entry.version, |ui| {
										TableBuilder::new(ui)
											.striped(true)
											.resizable(true)
											.columns(Column::auto().at_least(100.0), 2)
											.header(25.0, |mut header| {
												header.col(|ui| {
													ui.strong("Key");
												});
												header.col(|ui| {
													ui.strong("Value");
												});
											})
											.body(|mut body| {
												for tag in &entry.tags {
													body.row(20.0, |mut row| {
														row.col(|ui| {
															ui.add_space(4.0);
															ui.label(&tag.key);
														});
														row.col(|ui| {
															ui.add_space(4.0);
															ui.label(&tag.value);
														});
													});
												}
											});
									});
								});
							});
						}
					});
				});
			}
		});
	}
}
