use crate::saves::Meta;

use egui::{Context, Label};

/// This is used to give the to_title_case method on str
use inflector::Inflector;

/// This is used to allow us to use the fuzzy_match method on FuzzyMatcher
use fuzzy_matcher::FuzzyMatcher;

pub fn draw_meta(meta: &mut Meta, ctx: &Context, ui: &mut egui::Ui) {
    let window_size = ctx.screen_rect().size();
    let num_columns = 2;
    let scroll_height = window_size.y * 0.4;
    let search_width = window_size.x / num_columns as f32 * 0.65;

    ui.columns(num_columns, |columns| {
        columns[0].with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add(Label::new(
                egui::RichText::new("Jokers").color(egui::Color32::GREEN),
            ));
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut meta.filters.joker)
                        .desired_width(search_width)
                        .hint_text("Filter Jokers"),
                );
                if ui.button("Unlock All").clicked() {
                    meta.unlock_all_type("j_");
                }
            });

            egui::containers::ScrollArea::both()
                .auto_shrink(false)
                // .max_height(scroll_height)
                .min_scrolled_height(scroll_height)
                .id_salt("Joker Table")
                .show(ui, |ui| {
                    let mut joker_names = meta.get_joker_names();

                    if meta.filters.joker != "" {
                        joker_names.retain(|name| {
                            let score = meta.matcher.fuzzy_match(
                                &name[2..].to_lowercase(),
                                &meta.filters.joker.to_lowercase(),
                            );
                            if score.is_some_and(|x| x > 1) {
                                return true;
                            }
                            return false;
                        });
                    }
                    for joker_name in joker_names.iter() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}", &joker_name[2..].to_title_case()));
                            let joker = meta.get_item(&joker_name);

                            let joker_val = joker.unwrap();

                            if joker_val.can_be_alerted() {
                                ui.checkbox(&mut joker_val.alerted, "Alerted");
                            }

                            if joker_val.can_be_discovered() {
                                ui.checkbox(&mut joker_val.discovered, "Discovered");
                            }

                            if joker_val.can_be_unlocked() {
                                ui.checkbox(&mut joker_val.unlocked, "Unlocked");
                            }
                        });
                    }
                });

            ui.separator();

            ui.add(Label::new(
                egui::RichText::new("Cards").color(egui::Color32::LIGHT_BLUE),
            ));
            ui.horizontal(|ui| {
                // ui.label("Search");

                ui.add(
                    egui::TextEdit::singleline(&mut meta.filters.card)
                        .desired_width(search_width)
                        .hint_text("Filter Cards"),
                );
                if ui.button("Unlock All").clicked() {
                    meta.unlock_all_type("c_");
                }
            });
            egui::containers::ScrollArea::both()
                .auto_shrink(false)
                .max_height(scroll_height)
                .min_scrolled_height(scroll_height)
                .id_salt("Card Table")
                .show(ui, |ui| {
                    let mut card_names = meta.get_card_names();

                    if meta.filters.card != "" {
                        card_names.retain(|name| {
                            let score = meta.matcher.fuzzy_match(
                                &name[2..].to_lowercase(),
                                &meta.filters.card.to_lowercase(),
                            );
                            if score.is_some_and(|x| x > 1) {
                                return true;
                            }
                            return false;
                        });
                    }

                    for card_name in card_names.iter() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}", &card_name[2..].to_title_case()));
                            let card = meta.get_item(&card_name);

                            let card_val = card.unwrap();

                            if card_val.can_be_alerted() {
                                ui.checkbox(&mut card_val.alerted, "Alerted");
                            }
                            if card_val.can_be_discovered() {
                                ui.checkbox(&mut card_val.discovered, "Discovered");
                            }
                            if card_val.can_be_unlocked() {
                                ui.checkbox(&mut card_val.unlocked, "Unlocked");
                            }
                        });
                    }
                });
            ui.separator();
        });

        columns[1].with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.add(Label::new(
                egui::RichText::new("Vouchers").color(egui::Color32::PURPLE),
            ));
            ui.horizontal(|ui| {
                // ui.label("Search");

                ui.add(
                    egui::TextEdit::singleline(&mut meta.filters.voucher)
                        .desired_width(search_width)
                        .hint_text("Filter Vouchers"),
                );
                if ui.button("Unlock All").clicked() {
                    meta.unlock_all_type("v_");
                }
            });
            egui::containers::ScrollArea::both()
                .auto_shrink(false)
                .max_height(scroll_height)
                .min_scrolled_height(scroll_height)
                .id_salt("Voucher Table")
                .show(ui, |ui| {
                    let mut voucher_names = meta.get_voucher_names();

                    if meta.filters.voucher != "" {
                        voucher_names.retain(|name| {
                            let score = meta.matcher.fuzzy_match(
                                &name[2..].to_lowercase(),
                                &meta.filters.voucher.to_lowercase(),
                            );
                            if score.is_some_and(|x| x > 1) {
                                return true;
                            }
                            false
                        });
                    }
                    for voucher_name in voucher_names.iter() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}", &voucher_name[2..].to_title_case()));
                            let voucher = meta.get_item(&voucher_name);

                            let voucher_val = voucher.unwrap();

                            if voucher_val.can_be_alerted() {
                                ui.checkbox(&mut voucher_val.alerted, "Alerted");
                            }
                            if voucher_val.can_be_discovered() {
                                ui.checkbox(&mut voucher_val.discovered, "Discovered");
                            }
                            if voucher_val.can_be_unlocked() {
                                ui.checkbox(&mut voucher_val.unlocked, "Unlocked");
                            }
                        });
                    }
                });
            ui.separator();
            ui.add(Label::new(
                egui::RichText::new("Misc").color(egui::Color32::LIGHT_RED),
            ));
            ui.horizontal(|ui| {
                // ui.label("Search");

                ui.add(
                    egui::TextEdit::singleline(&mut meta.filters.misc)
                        .desired_width(search_width)
                        .hint_text("Filter Decks, Blinds, Tags, Edtions, and Booster Packs"),
                );
                if ui.button("Unlock All").clicked() {
                    meta.unlock_all_type("b_");
                    meta.unlock_all_type("e_");
                    meta.unlock_all_type("tag_");
                    meta.unlock_all_type("p_");
                    meta.unlock_all_type("bl_");
                }

            });

            egui::containers::ScrollArea::both()
                .auto_shrink(false)
                .max_height(scroll_height)
                .min_scrolled_height(scroll_height)
                .id_salt("Deck Table")
                .show(ui, |ui| {
                    let mut misc_names = meta.get_misc_names();

                    if meta.filters.misc != "" {
                        misc_names.retain(|name| {
                            let score = meta.matcher.fuzzy_match(
                                &name[2..].to_lowercase(),
                                &meta.filters.misc.to_lowercase(),
                            );
                            if score.is_some_and(|x| x > 1) {
                                return true;
                            }
                            return false;
                        });
                    }
                    for deck_name in misc_names.iter() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}", &deck_name[2..].to_title_case()));
                            let misc = meta.get_item(&deck_name);

                            let misc_val = misc.unwrap();
                            if misc_val.can_be_alerted() {
                                ui.checkbox(&mut misc_val.alerted, "Alerted");
                            }
                            if misc_val.can_be_discovered() {
                                ui.checkbox(&mut misc_val.discovered, "Discovered");
                            }
                            if misc_val.can_be_unlocked() {
                                ui.checkbox(&mut misc_val.unlocked, "Unlocked");
                            }
                        });
                    }
                });

            ui.separator();
        });
    });
}
