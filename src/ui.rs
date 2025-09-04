use crate::app::InjectorApp;
use crate::core::injector;
use crate::models::theme::{Theme, ThemeType};
use crate::models::toast::ToastLevel;
use eframe::egui::{self, Color32, Frame, Margin, RichText, Vec2, Rounding, Stroke};

pub fn show(ctx: &egui::Context, app: &mut InjectorApp) {
    apply_theme(ctx, &app.current_theme);
    
    draw_top_bar(ctx, app);
    draw_central_panel(ctx, app);
    draw_toasts(ctx, app);
}

fn draw_top_bar(ctx: &egui::Context, app: &mut InjectorApp) {
    egui::TopBottomPanel::top("selected_info_panel")
        .frame(
            Frame::default()
                .fill(app.current_theme.panel_bg)
                .stroke(Stroke::new(app.current_theme.border_width, app.current_theme.panel_stroke))
                .inner_margin(Margin::same(15.0))
                .rounding(Rounding::same(app.current_theme.rounding)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Process info
                let process_label = match app.selected_process_name() {
                    Some(name) => format!("Process: {} ({})", name, app.selected_process.unwrap()),
                    None => "Process: None".to_string(),
                };
                ui.label(RichText::new(process_label).size(16.0).color(app.current_theme.text_primary).strong());
                
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(20.0);
                
                // DLL info
                let dll_label = match app.dll_manager.selected_path() {
                    Some(path) => format!("DLL: {}", std::path::Path::new(&path).file_name().unwrap_or_default().to_string_lossy()),
                    None => "DLL: None".to_string(),
                };
                ui.label(RichText::new(dll_label).size(16.0).color(app.current_theme.text_secondary).strong());
                
                // Theme selector on the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    draw_theme_selector(ui, app);
                });
            });
        });
}

fn draw_central_panel(ctx: &egui::Context, app: &mut InjectorApp) {
    egui::CentralPanel::default()
        .frame(
            Frame::default()
                .fill(app.current_theme.bg_primary)
                .inner_margin(Margin::same(20.0))
                .rounding(Rounding::same(app.current_theme.rounding)),
        )
        .show(ctx, |ui| {
            let total_width = ui.available_width();
            let total_height = ui.available_height();
            let left_width = total_width * 0.45;
            let right_width = total_width * 0.55;

            ui.horizontal(|ui| {
                // Left panel with cute styling
                ui.allocate_ui(Vec2::new(left_width, total_height), |ui| {
                    Frame::default()
                        .fill(app.current_theme.bg_secondary)
                        .stroke(Stroke::new(app.current_theme.border_width, app.current_theme.panel_stroke))
                        .inner_margin(Margin::same(18.0))
                        .rounding(Rounding::same(app.current_theme.rounding))
                        .show(ui, |ui| {
                            draw_process_panel(ui, app);
                        });
                });
                
                ui.add_space(12.0);
                
                // Right panel with cute styling
                ui.allocate_ui(Vec2::new(right_width, total_height), |ui| {
                    Frame::default()
                        .fill(app.current_theme.bg_secondary)
                        .stroke(Stroke::new(app.current_theme.border_width, app.current_theme.panel_stroke))
                        .inner_margin(Margin::same(18.0))
                        .rounding(Rounding::same(app.current_theme.rounding))
                        .show(ui, |ui| {
                            draw_dll_panel(ui, app);
                        });
                });
            });
        });
}

fn draw_process_panel(ui: &mut egui::Ui, app: &mut InjectorApp) {
    let mut errors_to_toast: Vec<String> = Vec::new();
    let mut process_selected: Option<(u32, String)> = None; // Store selection info
    
    ui.vertical(|ui| {
        ui.add_space(12.0);
        
        // Header
        ui.label(RichText::new("SEARCH PROCESSES").size(18.0).strong().color(app.current_theme.text_primary));
        
        ui.add_space(8.0);
        
        // Search input
        Frame::default()
            .fill(app.current_theme.bg_secondary)
            .stroke(Stroke::new(app.current_theme.border_width, app.current_theme.panel_stroke))
            .inner_margin(Margin::same(12.0))
            .rounding(Rounding::same(app.current_theme.rounding))
            .show(ui, |ui| {
                ui.text_edit_singleline(&mut app.process_search);
            });
        
        ui.add_space(10.0);
        
        // Separator
        ui.separator();
        
        ui.add_space(8.0);
        
        // Y2K Buttons with bubble styling
        ui.horizontal(|ui| {
            let refresh_btn = egui::Button::new(RichText::new("REFRESH").size(13.0).color(Color32::WHITE))
                .fill(app.current_theme.accent_secondary)
                .rounding(Rounding::same(app.current_theme.rounding))
                .min_size([100.0, 35.0].into());
            if ui.add(refresh_btn).clicked() {
                app.refresh_processes();
            }
            ui.add_space(15.0);
            ui.checkbox(&mut app.auto_refresh, RichText::new("Auto Refresh").size(14.0).color(app.current_theme.text_primary));
        });
        ui.add_space(12.0);
        // Process list
        Frame::default()
            .fill(app.current_theme.panel_bg)
            .stroke(Stroke::new(app.current_theme.border_width, app.current_theme.panel_stroke))
            .inner_margin(Margin::same(15.0))
            .rounding(Rounding::same(app.current_theme.rounding))
            .show(ui, |ui| {
                egui::ScrollArea::vertical().id_source("process_list").show(ui, |ui| {
                    if app.is_loading_processes && app.processes.is_empty() {
                        ui.horizontal(|ui| {
                            ui.add(egui::Spinner::new());
                            ui.label(RichText::new("Loading processes...").color(app.current_theme.text_secondary));
                        });
                    } else {
                        let search_lower = app.process_search.to_lowercase();
                        for proc in &app.processes {
                            if !app.process_search.is_empty() && !proc.name.to_lowercase().contains(&search_lower) {
                                continue;
                            }
                            
                            let is_selected = app.selected_process == Some(proc.pid);
                            let bg_color = if is_selected {
                                app.current_theme.accent_primary
                            } else {
                                app.current_theme.bg_secondary
                            };
                            
                            Frame::default()
                                .fill(bg_color)
                                .stroke(if is_selected {
                                    Stroke::new(app.current_theme.border_width, app.current_theme.accent_secondary)
                                } else {
                                    Stroke::new(app.current_theme.border_width, app.current_theme.panel_stroke)
                                })
                                .inner_margin(Margin::same(6.0))
                                .rounding(Rounding::same(app.current_theme.rounding))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        if let Some(texture) = app.icon_cache.get(&proc.pid) {
                                            ui.image((texture.id(), Vec2::new(18.0, 18.0)));
                                        } else {
                                            ui.label("•");
                                        }
                                        ui.add_space(6.0);
                                        let label = format!("{} ({})", proc.name, proc.pid);
                                        if ui.selectable_label(is_selected, RichText::new(label).color(app.current_theme.text_primary)).clicked() {
                                            let proc_name = proc.name.clone();
                                            process_selected = Some((proc.pid, proc_name.clone()));
                                            if let Err(e) = app.config.save() {
                                                errors_to_toast.push(format!("Failed to save config: {}", e));
                                            }
                                        }
                                    });
                                });
                            ui.add_space(3.0);
                        }
                    }
                });
            });
    });
    
    // Handle process selection outside of borrowing loop
    if let Some((pid, proc_name)) = process_selected {
        app.selected_process = Some(pid);
        app.config.last_selected_app = Some(proc_name);
    }
    
    for error_msg in errors_to_toast {
        app.add_toast(ToastLevel::Error, error_msg);
    }
}

fn draw_dll_panel(ui: &mut egui::Ui, app: &mut InjectorApp) {
    
    ui.vertical(|ui| {
        ui.add_space(12.0);
        
        // DLL header
        ui.label(RichText::new("DLL FILES").size(18.0).strong().color(app.current_theme.text_primary));
        
        ui.add_space(10.0);
        
        // Separator
        ui.separator();
        
        ui.add_space(8.0);

        // DLL list
        Frame::default()
            .fill(app.current_theme.panel_bg)
            .stroke(Stroke::new(app.current_theme.border_width, app.current_theme.panel_stroke))
            .inner_margin(Margin::same(15.0))
            .rounding(Rounding::same(app.current_theme.rounding))
            .show(ui, |ui| {
                egui::ScrollArea::vertical().id_source("dll_list").show(ui, |ui| {
                    if app.dll_manager.get_dlls().is_empty() {
                        ui.label(RichText::new("No DLLs added yet. Add some DLL files!").color(app.current_theme.text_secondary));
                    } else {
                        for i in 0..app.dll_manager.get_dlls().len() {
                            let is_selected = app.dll_manager.selected_dll() == Some(i);
                            let file_name = std::path::Path::new(&app.dll_manager.get_dlls()[i])
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            
                            let bg_color = if is_selected {
                                app.current_theme.accent_primary
                            } else {
                                app.current_theme.bg_secondary
                            };
                            
                            Frame::default()
                                .fill(bg_color)
                                .stroke(if is_selected {
                                    Stroke::new(app.current_theme.border_width, app.current_theme.accent_secondary)
                                } else {
                                    Stroke::new(app.current_theme.border_width, app.current_theme.panel_stroke)
                                })
                                .inner_margin(Margin::same(6.0))
                                .rounding(Rounding::same(app.current_theme.rounding))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        if let Some(tex) = &app.default_dll_texture {
                                            ui.image((tex.id(), Vec2::new(18.0, 18.0)));
                                        } else {
                                            ui.label("•");
                                        }
                                        ui.add_space(6.0);
                                        
                                        if ui.selectable_label(is_selected, RichText::new(file_name).color(app.current_theme.text_primary)).clicked() {
                                            app.dll_manager.select(Some(i));
                                        }
                                    });
                                });
                            ui.add_space(3.0);
                        }
                    }
                });
            });
        
        ui.add_space(16.0);
        
        // Separator
        ui.separator();
        
        ui.add_space(12.0);

        // Y2K Cyber Buttons
        ui.vertical(|ui| {
            let button_size = Vec2::new(160.0, 45.0);
            
            // Add DLL button
            let add_btn = egui::Button::new(RichText::new("ADD DLL").size(15.0).color(Color32::WHITE))
                .fill(app.current_theme.accent_primary)
                .rounding(Rounding::same(app.current_theme.rounding))
                .min_size(button_size);
            if ui.add(add_btn).clicked() {
                if let Some(path) = crate::core::dll_selector::select_dll() {
                    if !app.dll_manager.get_dlls().contains(&path) {
                        app.dll_manager.add(path.clone());
                        app.config.dlls.push(path);
                        let _ = app.config.save();
                        app.add_toast(ToastLevel::Success, "◇ DLL added to cyber collection! ◇");
                    } else {
                        app.add_toast(ToastLevel::Warning, "◇ This DLL already exists in the matrix! ◇");
                    }
                }
            }
            
            ui.add_space(8.0);
            
            // Inject DLL button - Y2K cyber style
            let inject_enabled = app.selected_process.is_some() && app.dll_manager.selected_dll().is_some();
            let inject_color = if inject_enabled {
                app.current_theme.success
            } else {
                app.current_theme.text_disabled
            };
            let inject_btn = egui::Button::new(RichText::new("INJECT DLL").size(15.0).color(Color32::WHITE))
                .fill(inject_color)
                .rounding(Rounding::same(app.current_theme.rounding))
                .min_size(button_size);
            if ui.add_enabled(inject_enabled, inject_btn).clicked() {
                if let (Some(pid), Some(dll_path)) = (app.selected_process, app.dll_manager.selected_path()) {
                    let _process_name = app.selected_process_name().unwrap_or("Unknown").to_string();
                    
                    let result = injector::inject_dll(pid, &dll_path);
                    
                    match result {
                        Ok(_) => {
                            app.add_toast(
                                ToastLevel::Success, 
                                "◇ Standard injection successful! Welcome to the matrix! ◇"
                            );
                        },
                        Err(e) => {
                            app.add_toast(ToastLevel::Error, format!("◇ Injection failed: {} ◇", e));
                        },
                    }
                }
            }
            
            ui.add_space(8.0);
            
            // Remove DLL button - Y2K cyber style
            let remove_enabled = app.dll_manager.selected_dll().is_some();
            let remove_color = if remove_enabled {
                app.current_theme.error
            } else {
                app.current_theme.text_disabled
            };
            let remove_btn = egui::Button::new(RichText::new("REMOVE DLL").size(15.0).color(Color32::WHITE))
                .fill(remove_color)
                .rounding(Rounding::same(app.current_theme.rounding))
                .min_size(button_size);
            if ui.add_enabled(remove_enabled, remove_btn).clicked() {
                if let Some(selected_index) = app.dll_manager.selected_dll() {
                    app.dll_manager.remove(selected_index);
                    app.config.dlls.remove(selected_index);
                    let _ = app.config.save();
                    app.add_toast(ToastLevel::Info, "◇ DLL removed from cyber space! ◇");
                }
            }
        });
        
        // GitHub link section
        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);
        
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            let github_link = egui::Button::new(
                RichText::new("🌟 GitHub Repository")
                    .size(14.0)
                    .color(app.current_theme.text_primary)
            )
            .fill(app.current_theme.accent_tertiary)
            .stroke(Stroke::new(1.5, app.current_theme.accent_primary))
            .rounding(Rounding::same(app.current_theme.rounding));
            
            if ui.add(github_link).clicked() {
                if let Err(_) = webbrowser::open("https://github.com/Kiy0w0/baam-injector") {
                    app.add_toast(ToastLevel::Error, "◇ Failed to open GitHub link ◇");
                }
            }
            
            ui.add_space(8.0);
            ui.label(
                RichText::new("Check for updates & report issues")
                    .size(12.0)
                    .color(app.current_theme.text_secondary)
            );
        });
        
        ui.add_space(12.0);
    });
}

fn draw_toasts(ctx: &egui::Context, app: &mut InjectorApp) {
    app.toasts.retain(|toast| toast.is_alive());
    egui::Area::new("toasts".into())
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-12.0, -12.0))
        .show(ctx, |ui| {
            for toast in &app.toasts {
                let (icon, bg_color, text_color) = match toast.level {
                    ToastLevel::Info => ("ℹ", app.current_theme.accent_secondary, app.current_theme.text_primary),
                    ToastLevel::Success => ("✓", app.current_theme.success, app.current_theme.text_primary),
                    ToastLevel::Warning => ("⚠", app.current_theme.warning, app.current_theme.text_primary),
                    ToastLevel::Error => ("✗", app.current_theme.error, app.current_theme.text_primary),
                };
                
                Frame::default()
                    .fill(bg_color)
                    .stroke(Stroke::new(app.current_theme.border_width, app.current_theme.panel_stroke))
                    .inner_margin(Margin::same(12.0))
                    .rounding(Rounding::same(app.current_theme.rounding))
                    .show(ui, |ui| {
                        ui.set_min_width(220.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(icon).size(16.0).color(text_color));
                            ui.add_space(8.0);
                            ui.label(RichText::new(&toast.message).size(14.0).color(text_color).strong());
                        });
                    });
                ui.add_space(8.0);
            }
        });
}

fn apply_theme(ctx: &egui::Context, theme: &Theme) {
    let mut visuals = if matches!(theme, Theme { bg_primary, .. } if bg_primary.r() < 50) {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    
    // Apply theme colors
    visuals.override_text_color = Some(theme.text_primary);
    visuals.panel_fill = theme.bg_primary;
    visuals.window_fill = theme.bg_primary;
    visuals.extreme_bg_color = theme.bg_secondary;
    visuals.code_bg_color = theme.bg_tertiary;
    visuals.faint_bg_color = theme.bg_secondary;
    
    // Selection colors
    visuals.selection.bg_fill = theme.accent_primary;
    visuals.selection.stroke = Stroke::new(theme.border_width, theme.accent_secondary);
    
    // Widget colors
    visuals.widgets.noninteractive.bg_fill = theme.panel_bg;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, theme.text_secondary);
    visuals.widgets.inactive.bg_fill = theme.button_bg;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, theme.button_text);
    visuals.widgets.hovered.bg_fill = theme.button_bg_hovered;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, theme.button_text);
    visuals.widgets.active.bg_fill = theme.button_bg_active;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, theme.button_text);
    visuals.widgets.open.bg_fill = theme.accent_secondary;
    
    // Rounding
    let rounding = Rounding::same(theme.rounding);
    visuals.widgets.noninteractive.rounding = rounding;
    visuals.widgets.inactive.rounding = rounding;
    visuals.widgets.hovered.rounding = rounding;
    visuals.widgets.active.rounding = rounding;
    visuals.widgets.open.rounding = rounding;
    visuals.window_rounding = Rounding::same(theme.rounding + 5.0);
    visuals.menu_rounding = rounding;
    
    // Borders
    let border_stroke = Stroke::new(theme.border_width, theme.panel_stroke);
    visuals.widgets.noninteractive.bg_stroke = border_stroke;
    visuals.widgets.inactive.bg_stroke = border_stroke;
    visuals.widgets.hovered.bg_stroke = Stroke::new(theme.border_width, theme.accent_primary);
    visuals.widgets.active.bg_stroke = Stroke::new(theme.border_width, theme.accent_primary);
    
    ctx.set_visuals(visuals);
}

fn draw_theme_selector(ui: &mut egui::Ui, app: &mut InjectorApp) {
    egui::ComboBox::from_label("🎨 Theme")
        .selected_text(app.config.selected_theme.name())
        .show_ui(ui, |ui| {
            for theme_type in ThemeType::all() {
                let is_selected = app.config.selected_theme == theme_type;
                if ui.selectable_label(is_selected, theme_type.name()).clicked() {
                    app.set_theme(theme_type);
                }
            }
        });
}