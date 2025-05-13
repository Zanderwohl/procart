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
    mut rays: ResMut<Rays>,
) {
    if !ui_state.params_panel || !program_state.eq(&ProgramState::Bubbles) {
        return;
    }

    egui::Window::new("Params").show(contexts.ctx_mut(), |ui| {
        ui.heading("Steps");
        ui.checkbox(&mut bubbles.render, "Bubbles");
        ui.checkbox(&mut rays.render, "Rays");

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

        ui.heading("Rays");
        ui.add(egui::Slider::new(&mut rays.angle, 0.0..=TAU).text("Angle"));
        ui.add(egui::Slider::new(&mut rays.min_length, 0.0..=10.0).text("Min Length").step_by(0.1));
        ui.add(egui::Slider::new(&mut rays.max_length, 0.1..=10.0).text("Max Length").step_by(0.1));
        ui.add(egui::Slider::new(&mut rays.thickness, 0.0..=1.0).text("Thickness"));
        ui.add(egui::Slider::new(&mut rays.alpha, 0.0..=1.0).text("Alpha"));
        ui.add(egui::Slider::new(&mut rays.speed, 0.0..=10.0).text("Speed"));
    });
}

static CYCLE: f32 = 4.0;

fn draw(mut painter: ShapePainter, time: Res<Time>, windows: Query<&Window>, bubbles: Res<Bubbles>, rays: Res<Rays>) {
    let seconds = time.elapsed_secs();
    // let start_pos = painter.transform;

    let window: &Window = windows.single().unwrap();
    let width = window.resolution.width();
    let height = window.resolution.height();

    painter.set_color(BLUE.pastel());
    painter.rect(Vec2::new(width, height));

    // Draw bubbles
    if bubbles.render {
        bubbles.draw(&mut painter, seconds);
    }

    if rays.render {
        rays.draw(&mut painter, seconds);
    }
}

#[derive(Resource)]
struct Bubbles {
    bubbles: Vec<Bubble>,
    render: bool,
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

            render: true,
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
        let r_1 = params.outer_radius_min.lerp(params.outer_radius_max, self.t(seconds)) * self.scale;
        let r_2 = params.inner_radius_min.lerp(params.inner_radius_max, self.t(seconds)) * self.scale;

        painter.thickness = params.thickness;
        painter.circle(r_1);

        painter.thickness = params.shine_thickness;
        painter.arc(r_2, params.shine_start, params.shine_end);
    }

    fn t(&self, seconds: f32) -> f32 {
        let frac = (seconds % CYCLE) / CYCLE;
        (frac + 1.0 + self.spawn_offset) % 1.0
    }

    fn pos(&self, seconds: f32, params: &Bubbles) -> Vec2 {
        let t = self.t(seconds);
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
    origin: Vec2,
    render: bool,
    angle: f32,
    min_length: f32,
    max_length: f32,
    thickness: f32,
    alpha: f32,
    speed: f32,
}

impl Rays {
    fn draw(&self, painter: &mut ShapePainter, seconds: f32) {
        for ray in &self.rays {
            ray.draw(painter, seconds, self);
        }
    }
}

impl Default for Rays {
    fn default() -> Self {
        let mut rays = Vec::new();
        let mut rng = rand::rng();

        let mut acc: f32 = 0.0;
        for _ in 0..60 {
            let thickness = rng.random::<f32>();
            let length = rng.random::<f32>();
            let offset = rng.random::<f32>();
            let frequency = rng.random::<f32>();
            let x = acc + (thickness / 2.0);
            acc += thickness;
            rays.push(Ray {
                thickness,
                length,
                offset,
                frequency,
                x,
            })
        }

        Self {
            rays,
            origin: Vec2::new(-6.5, 2.5),
            angle: 5.1,
            render: true,
            min_length: 0.5,
            max_length: 5.5,
            thickness: 1.0,
            alpha: 0.03,
            speed: 8.5,
        }
    }
}

struct Ray {
    x: f32,
    length: f32,
    thickness: f32,
    offset: f32,
    frequency: f32,
}

impl Ray {
    fn draw(&self, painter: &mut ShapePainter, seconds: f32, params: &Rays) {
        let origin = params.origin;
        painter.set_translation(origin.extend(2.0));
        painter.thickness = params.thickness * self.thickness;

        let alpha = params.alpha * f32::sin(self.frequency * (seconds - self.offset) * params.speed);
        painter.set_color(YELLOW.pastel_very().with_alpha(alpha));

        let length = (params.max_length - params.min_length) * self.length + params.min_length;
        let start = origin + Vec2::new(self.x * params.thickness, 0.0);
        let end = start + Vec2::new(length * params.angle.cos(), length * params.angle.sin());
        painter.line(start.extend(2.0), end.extend(2.0));
    }
}
