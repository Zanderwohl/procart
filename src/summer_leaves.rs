use bevy::app::App;
use bevy::{prelude::*, color::palettes::css::*};
use bevy_vector_shapes::prelude::*;
use crate::{ProgramState, UIState};
use std::f32::consts::{PI, TAU};
use bevy_egui::{egui, EguiContextPass, EguiContexts};
use crate::common::Modifier;

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
}

impl Default for LeafParams {
    fn default() -> Self {
        Self {
            show_debug_grid: true,
        }
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
        ui.checkbox(&mut params.show_debug_grid, "Debug grid");
    });
}

fn draw(mut painter: ShapePainter, time: Res<Time>, windows: Query<&Window>, params: Res<LeafParams>) {

}
