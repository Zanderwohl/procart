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
    center_offset: f32,
    debug_rotation: f32,
    bounds: Vec2,
    debug_show_bounds: bool,
}

impl Default for LeafParams {
    fn default() -> Self {
        Self {
            show_debug_grid: true,
            debug_spacing: 0.5,
            leaf_size: 0.5,
            line_thickness: 0.2,
            center_offset: -0.12,
            debug_rotation: 0.0,
            bounds: Vec2::new(3.0, 3.0),
            debug_show_bounds: true,
        }
    }
}

impl LeafParams {
    fn draw_debug_grid(&self, painter: &mut ShapePainter, rand: Res<CachedRandom>) {
        painter.set_color(BLACK);
        for i in 0..10 {
            for j in 0..10 {
                let idx: usize = i * 10 + j;
                self.draw_leaf(painter, Vec3::new(i as f32 * self.debug_spacing, j as f32 * self.debug_spacing, 1.1), rand.f32(idx) * TAU * self.debug_rotation, idx)
            }
        }
    }

    fn draw_leaf(&self, painter: &mut ShapePainter, pos: Vec3, rotation: f32, idx: usize) {
        let pos = pos + Vec3::new(self.center_offset * rotation.cos(), self.center_offset * rotation.sin(), 0.0);
        painter.set_translation(pos);
        painter.set_rotation(Quat::from_rotation_z(rotation));
        painter.hollow = false;
        painter.thickness = self.line_thickness;
        painter.line(Vec3::ZERO.with_z(1.0), Vec3::new(self.leaf_size, 0.0, 1.0));
        painter.circle(self.leaf_size / 2.0);
    }

    fn draw_bounds(&self, painter: &mut ShapePainter) {
        painter.set_color(RED.pastel().with_alpha(0.6));
        painter.set_translation(Vec3::ZERO);
        painter.set_rotation(Quat::from_rotation_z(0.0));
        // painter.hollow = true;
        painter.thickness = self.line_thickness;
        painter.line(Vec3::new(-self.bounds.x, -self.bounds.y, 5.0), Vec3::new(self.bounds.x, -self.bounds.y, 5.0));
        painter.line(Vec3::new(self.bounds.x, -self.bounds.y, 5.0), Vec3::new(self.bounds.x, self.bounds.y, 5.0));
        painter.line(Vec3::new(self.bounds.x, self.bounds.y, 5.0), Vec3::new(-self.bounds.x, self.bounds.y, 5.0));
        painter.line(Vec3::new(-self.bounds.x, self.bounds.y, 5.0), Vec3::new(-self.bounds.x, -self.bounds.y, 5.0));
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
        ui.add(egui::Slider::new(&mut params.debug_rotation, 0.0..=1.0).text("Rotation"));

        ui.heading("Leaves");
        ui.add(egui::Slider::new(&mut params.leaf_size, 0.01..=0.5).text("Size"));
        ui.add(egui::Slider::new(&mut params.line_thickness, 0.01..=1.0).text("Line Thickness"));
        ui.add(egui::Slider::new(&mut params.center_offset, -0.5..=0.5).text("Center Offset"));

        ui.heading("Bounds");
        ui.add(egui::Slider::new(&mut params.bounds.x, 0.0..=10.0).text("Bounds X"));
        ui.add(egui::Slider::new(&mut params.bounds.y, 0.0..=10.0).text("Bounds Y"));
        ui.checkbox(&mut params.debug_show_bounds, "Debug show bounds");
    });
}

fn draw(mut painter: ShapePainter, time: Res<Time>, windows: Query<&Window>, params: Res<LeafParams>, rand: Res<CachedRandom>) {
    painter.thickness_type = ThicknessType::Screen;

    let seconds = time.elapsed_secs();
    if params.show_debug_grid {
        params.draw_debug_grid(&mut painter, rand);
    }
    if params.debug_show_bounds {
        params.draw_bounds(&mut painter);
    }
}
