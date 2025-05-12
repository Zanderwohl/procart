use bevy::app::App;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use crate::ProgramState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct CircleSet;

pub struct CircleArt;
impl Plugin for CircleArt {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Update,
                            (
                                CircleSet.run_if(in_state(ProgramState::Circle)),
                            )
            )
            .add_systems(Update,
                         (
                             draw,
                         ).in_set(CircleSet)
            )
        ;
    }
}

fn draw(mut painter: ShapePainter) {
    // Draw a circle
    painter.circle(1.0);
}
