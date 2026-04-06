// systems/egui_ui.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::{AspectRatio, AspectRatioState, EguiLayoutState, GridState, TextureMode, TextureModeState};
use crate::constants::{EGUI_LEFT_PANEL_WIDTH, EGUI_TOP_BAR_HEIGHT};

pub fn egui_controls_ui(
    mut contexts: EguiContexts,
    mut layout_state: ResMut<EguiLayoutState>,
    mut grid_state: ResMut<GridState>,
    mut aspect_ratio_state: ResMut<AspectRatioState>,
    mut texture_mode_state: ResMut<TextureModeState>,
    mut camera_projection: Query<&mut Projection, (With<Camera3d>, With<crate::components::RightCamera>)>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    let title_bar = egui::TopBottomPanel::top("title_bar")
        .resizable(false)
        .exact_height(EGUI_TOP_BAR_HEIGHT)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.strong("texture-bevy-egui-app");
            });
        });

    egui::SidePanel::left("controls_panel")
        .resizable(false)
        .exact_width(EGUI_LEFT_PANEL_WIDTH)
        .show_separator_line(false)
        .frame(
            egui::Frame::default()
                .fill(ctx.style().visuals.panel_fill)
                .stroke(egui::Stroke::NONE)
                .outer_margin(egui::Margin::ZERO),
        )
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Controls");
                ui.separator();

                ui.label("Camera");
                ui.label("Perspective projection");

                if let Ok(mut projection) = camera_projection.single_mut()
                    && let Projection::Perspective(perspective) = &mut *projection
                {
                    let mut fov_degrees = perspective.fov.to_degrees();
                    ui.label("Field of View");
                    if ui
                        .add(
                            egui::Slider::new(&mut fov_degrees, 30.0..=120.0)
                                .text("FOV")
                                .step_by(1.0),
                        )
                        .changed()
                    {
                        perspective.fov = fov_degrees.to_radians();
                    }
                }

                ui.separator();
                ui.label("Grid size");

                ui.horizontal(|ui| {
                    ui.label("Width");
                    ui.add(
                        egui::DragValue::new(&mut grid_state.size_x)
                            .range(1..=100)
                            .speed(1)
                            .suffix(" m"),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Depth");
                    ui.add(
                        egui::DragValue::new(&mut grid_state.size_z)
                            .range(1..=100)
                            .speed(1)
                            .suffix(" m"),
                    );
                });

                ui.separator();
                ui.label("Texture aspect ratio");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut aspect_ratio_state.current, AspectRatio::Ratio16_9, "16:9");
                    ui.selectable_value(&mut aspect_ratio_state.current, AspectRatio::Square, "Square");
                });

                ui.separator();
                ui.label("Texture fit");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut texture_mode_state.current, TextureMode::Normal, "Preserve");
                    ui.selectable_value(&mut texture_mode_state.current, TextureMode::Stretch, "Stretch");
                });
            });
        });

    // Hard split: world starts exactly where the fixed left column ends.
    let viewport_rect = ctx.viewport_rect();
    layout_state.viewport_left = EGUI_LEFT_PANEL_WIDTH;
    layout_state.viewport_top = title_bar.response.rect.bottom();
    layout_state.viewport_right = viewport_rect.right();
    layout_state.viewport_bottom = viewport_rect.bottom();
}
