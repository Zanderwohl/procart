use bevy::app::App;
use bevy::{prelude::*, color::palettes::css::*};
use bevy_vector_shapes::prelude::*;
use crate::ProgramState;
use std::f32::consts::{PI, TAU};
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
            .init_resource::<Bubbles>()
            //.add_systems(Startup, setup)
            .add_systems(
                Update,
                    (
                        draw,
                    ).in_set(BubbleSet)
            )
        ;
    }
}

static CYCLE: f32 = 2.0;

fn draw(mut painter: ShapePainter, time: Res<Time>, windows: Query<&Window>, bubbles: Res<Bubbles>) {
    let seconds = time.elapsed_secs();
    let start_pos = painter.transform;

    let window: &Window = windows.single().unwrap();
    let width = window.resolution.width();
    let height = window.resolution.height();

    painter.set_color(BLUE.pastel());
    painter.rect(Vec2::new(width, height));

    // Draw a circle
    painter.set_color(BLUE.pastel_very());
    for bubble in &bubbles.bubbles {
        bubble.draw(&mut painter, seconds);
    }
    //painter.circle(1.0);
}

#[derive(Resource)]
struct Bubbles {
    bubbles: Vec<Bubble>,
}

impl Default for Bubbles {
    fn default() -> Self {
        let mut rng = rand::rng();
        let mut bubbles = Vec::new();

        let width = 20.0;

        for _ in 0..50 {
            let x = rng.random::<f32>() * width - (width / 2.0);
            let spawn_offset = rng.random::<f32>();
            let wobble_offset = rng.random::<f32>();
            bubbles.push(Bubble { x, spawn_offset, wobble_offset });
        }

        Self {
            bubbles,
        }
    }
}

struct Bubble {
    x: f32,
    spawn_offset: f32,
    wobble_offset: f32,
}

impl Bubble {
    fn draw(&self, painter: &mut ShapePainter, seconds: f32) {
        let pos = self.pos(seconds);
        painter.set_translation(pos.extend(1.0));
        painter.hollow = true;
        let r_1 = 0.4.lerp(0.5, self.t(seconds));
        let r_2 = 0.3.lerp(0.4, self.t(seconds));

        painter.circle(r_1);

        painter.arc(r_2, PI / 6.0, PI / 3.0);
    }

    fn t(&self, seconds: f32) -> f32 {
        let frac = (seconds % CYCLE) / CYCLE;
        (frac + 1.0 + self.spawn_offset) % 1.0
    }

    fn pos(&self, seconds: f32) -> Vec2 {
        let t = self.t(seconds);
        let x = self.x + (seconds + self.wobble_offset * 10.0).sin() * 2.0;
        //let x = self.x;
        let start = Vec2::new(t, -7.0);
        let end = Vec2::new(t, 7.0);
        let line = start.lerp(end, t);
        Vec2 {
            x: line.x + x,
            y: line.y,
        }
    }
}
