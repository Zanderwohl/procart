use bevy::app::App;
use bevy::{prelude::*, color::palettes::css::*};
use bevy_vector_shapes::prelude::*;
use crate::{ProgramState, UIState};
use std::f32::consts::{PI, TAU};
use bevy_egui::{egui, EguiContextPass, EguiContexts};
use crate::common::{CachedRandom, Modifier};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct LeafSet;

pub struct LeafArt;

impl Plugin for LeafArt {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Update,
                            (
                                LeafSet.run_if(in_state(ProgramState::SummerLeaves)),
                                ))
            .init_resource::<LeafParams>()
            .add_systems(EguiContextPass, params_ui)
            .add_systems(
                Update,
                (
                    draw,
                ).in_set(LeafSet)
            )
        ;
    }
}

#[derive(Resource)]
struct LeafParams {
    show_debug_grid: bool,
    debug_spacing: f32,
    leaf_size: f32,
    line_thickness: f32,
}

impl Default for LeafParams {
    fn default() -> Self {
        Self {
            show_debug_grid: true,
            debug_spacing: 0.5,
            leaf_size: 0.5,
            line_thickness: 0.01,
        }
    }
}

impl LeafParams {
    fn draw_debug_grid(&self, painter: &mut ShapePainter) {
        for i in 0..10 {
            for j in 0..10 {
                let idx: usize = i * 10 + j;
                self.draw_leaf(painter, Vec3::new(i as f32 * self.debug_spacing, j as f32 * self.debug_spacing, 1.1), idx as f32, idx)
            }
        }
    }

    fn draw_leaf(&self, painter: &mut ShapePainter, pos: Vec3, rotation: f32, idx: usize) {
        painter.set_translation(pos);
        painter.hollow = true;
        painter.thickness = self.line_thickness;
        painter.arc(self.leaf_size, 0.0 + rotation, TAU / 6.0 + rotation);
    }
}

fn params_ui(
    mut contexts: EguiContexts,
    mut params: ResMut<LeafParams>,
    ui_state: Res<UIState>,
    program_state: Res<State<ProgramState>>,
) {
    if !ui_state.params_panel || !program_state.eq(&ProgramState::SummerLeaves) {
        return;
    }

    egui::Window::new("Parameters").show(contexts.ctx_mut(), |ui| {
        ui.heading("Debug");
        ui.checkbox(&mut params.show_debug_grid, "Debug grid") ;
        ui.add(egui::Slider::new(&mut params.debug_spacing, 0.1..=10.0).text("Spacing"));

        ui.heading("Leaves");
        ui.add(egui::Slider::new(&mut params.leaf_size, 0.01..=0.5).text("Size"));
        ui.add(egui::Slider::new(&mut params.line_thickness, 0.001..=0.1).text("Line Thickness"));
    });
}

fn draw(mut painter: ShapePainter, time: Res<Time>, windows: Query<&Window>, params: Res<LeafParams>, rand: Res<CachedRandom>) {
    painter.thickness_type = ThicknessType::Screen;

    let seconds = time.elapsed_secs();
    if params.show_debug_grid {
        params.draw_debug_grid(&mut painter);
    }
}
