use bevy::app::App;
use bevy::{prelude::*, color::palettes::css::*};
use bevy_vector_shapes::prelude::*;
use crate::{ProgramState, UIState};
use std::f32::consts::{PI, TAU};
use bevy_egui::{egui, EguiContextPass, EguiContexts};
use rand::prelude::*;
use crate::common::Modifier;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct BubbleSet;

pub struct BubbleArt;

impl Plugin for BubbleArt {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Update,
                            (
                                    BubbleSet.run_if(in_state(ProgramState::Bubbles)),
                                ))
            .init_resource::<Rays>()
            .init_resource::<Bubbles>()
            .add_systems(EguiContextPass, params_ui)
            .add_systems(
                Update,
                    (
                        draw,
                    ).in_set(BubbleSet)
            )
        ;
    }
}

fn params_ui(
    mut contexts: EguiContexts,
    ui_state: Res<UIState>,
    program_state: Res<State<ProgramState>>,
    mut bubbles: ResMut<Bubbles>,
) {
    if !ui_state.params_panel || !program_state.eq(&ProgramState::Bubbles) {
        return;
    }

    egui::Window::new("Params").show(contexts.ctx_mut(), |ui| {
        ui.heading("Thickness");
        ui.add(egui::Slider::new(&mut bubbles.thickness, 0.01..=0.1).text("Outer"));

        ui.heading("Radius");
        ui.add(egui::Slider::new(&mut bubbles.outer_radius_min, 0.2..=2.0).text("Outer Min"));
        ui.add(egui::Slider::new(&mut bubbles.outer_radius_max, 0.2..=2.0).text("Outer Max"));
        ui.add(egui::Slider::new(&mut bubbles.inner_radius_min, 0.2..=2.0).text("Inner Min"));
        ui.add(egui::Slider::new(&mut bubbles.inner_radius_max, 0.2..=2.0).text("Inner Max"));

        ui.heading("Shine");
        ui.add(egui::Slider::new(&mut bubbles.shine_start, 0.0..=TAU).text("Start"));
        ui.add(egui::Slider::new(&mut bubbles.shine_end, 0.0..=TAU).text("End"));
        ui.add(egui::Slider::new(&mut bubbles.shine_thickness, 0.01..=0.1).text("Thickness"));

        ui.heading("Wobble");
        ui.add(egui::Slider::new(&mut bubbles.wobble_size, 0.0..=4.0).text("Size"));
        ui.add(egui::Slider::new(&mut bubbles.wobble_frequency, 0.0..=20.0).text("Frequency"));

        ui.heading("Positioning");
        ui.add(egui::Slider::new(&mut bubbles.starting, 0.0..=10.0).text("Starting"));
        ui.add(egui::Slider::new(&mut bubbles.starting_range, 0.0..=5.0).text("Range"));
    });
}

static CYCLE: f32 = 4.0;

fn draw(mut painter: ShapePainter, time: Res<Time>, windows: Query<&Window>, bubbles: Res<Bubbles>, rays: Res<Rays>) {
    let seconds = time.elapsed_secs();
    let start_pos = painter.transform;

    let window: &Window = windows.single().unwrap();
    let width = window.resolution.width();
    let height = window.resolution.height();

    painter.set_color(BLUE.pastel());
    painter.rect(Vec2::new(width, height));

    // Draw bubbles
    bubbles.draw(&mut painter, seconds);
}

#[derive(Resource)]
struct Bubbles {
    bubbles: Vec<Bubble>,
    thickness: f32,
    shine_start: f32,
    shine_end: f32,
    shine_thickness: f32,
    outer_radius_min: f32,
    outer_radius_max: f32,
    inner_radius_min: f32,
    inner_radius_max: f32,
    wobble_frequency: f32,
    wobble_size: f32,
    starting: f32,
    starting_range: f32,
}

impl Bubbles {
    fn draw(&self, painter: &mut ShapePainter, seconds: f32) {
        painter.set_color(BLUE.pastel_very());
        for bubble in &self.bubbles {
            bubble.draw(painter, seconds, self);
        }
    }
}

impl Default for Bubbles {
    fn default() -> Self {
        let mut rng = rand::rng();
        let mut bubbles = Vec::new();

        let width = 20.0;
        let scale_range = 0.4;

        for _ in 0..50 {
            let x = rng.random::<f32>() * width - (width / 2.0);
            let spawn_offset = rng.random::<f32>();
            let wobble_offset = rng.random::<f32>();
            let scale = rng.random::<f32>() * scale_range + (1.0 - (scale_range / 2.0));
            bubbles.push(Bubble { x, spawn_offset, wobble_offset, scale });
        }

        Self {
            bubbles,

            thickness: 0.01,
            shine_start: PI / 6.0,
            shine_end: PI / 3.0,
            shine_thickness: 0.01,
            outer_radius_min: 0.4,
            outer_radius_max: 0.5,
            inner_radius_min: 0.3,
            inner_radius_max: 0.4,
            wobble_frequency: 10.0,
            wobble_size: 1.5,
            starting: 7.0,
            starting_range: 5.0,
        }
    }
}

struct Bubble {
    x: f32,
    spawn_offset: f32,
    wobble_offset: f32,
    scale: f32,
}

impl Bubble {
    fn draw(&self, painter: &mut ShapePainter, seconds: f32, params: &Bubbles) {
        let pos = self.pos(seconds, params);
        painter.set_translation(pos.extend(1.0));
        painter.hollow = true;
        let r_1 = params.outer_radius_min.lerp(params.outer_radius_max, self.t(seconds, params)) * self.scale;
        let r_2 = params.inner_radius_min.lerp(params.inner_radius_max, self.t(seconds, params)) * self.scale;

        painter.thickness = params.thickness;
        painter.circle(r_1);

        painter.thickness = params.shine_thickness;
        painter.arc(r_2, params.shine_start, params.shine_end);
    }

    fn t(&self, seconds: f32, params: &Bubbles) -> f32 {
        let frac = (seconds % CYCLE) / CYCLE;
        (frac + 1.0 + self.spawn_offset) % 1.0
    }

    fn pos(&self, seconds: f32, params: &Bubbles) -> Vec2 {
        let t = self.t(seconds, params);
        let x = self.x + (seconds + self.wobble_offset * params.wobble_frequency).sin() * params.wobble_size;
        let start = Vec2::new(t, -params.starting - (self.scale * params.starting_range));
        let end = Vec2::new(t, params.starting + (self.scale * params.starting_range));
        let line = start.lerp(end, t);
        Vec2 {
            x: line.x + x,
            y: line.y,
        }
    }
}

#[derive(Resource)]
struct Rays {
    rays: Vec<Ray>,
}

impl Default for Rays {
    fn default() -> Self {
        let mut rays = Vec::new();
        Self { rays }
    }
}

struct Ray {

}
