mod circle;
mod gallery;
mod bubbles;
mod common;
mod summer_leaves;

use bevy::color::palettes::css::DIM_GRAY;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::ExitCondition;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiContextPass};
use bevy_vector_shapes::prelude::*;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::bubbles::BubbleArt;
use crate::circle::CircleArt;
use crate::common::CachedRandom;
use crate::gallery::GalleryArt;
use crate::summer_leaves::LeafArt;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Alexander Lowry's Digital Artwork".into(),
                    ..Default::default()
                }),
                exit_condition: ExitCondition::OnPrimaryClosed,
                close_when_requested: true,
            }),
            ShapePlugin::default(),
        ))
        .insert_resource(ClearColor(DIM_GRAY.into()))
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        .add_event::<Quit>()
        .init_resource::<UIState>()
        .init_resource::<CachedRandom>()
        .init_state::<ProgramState>()
        .add_systems(EguiContextPass, ProgramState::selection_system.run_if(in_state(ProgramState::MainMenu)))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            ProgramState::shortcuts,
            exit_system,
            ))
        .add_plugins((
            CircleArt,
            GalleryArt,
            BubbleArt,
            LeafArt,
        ))
        .run();
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, EnumIter)]
enum ProgramState {
    #[default]
    MainMenu,
    Circle,
    Gallery,
    Bubbles,
    SummerLeaves,
}

#[derive(Resource)]
struct UIState {
    params_panel: bool,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            params_panel: false,
        }
    }
}

impl ProgramState {
    pub fn selection_system(
        mut contexts: EguiContexts,
        mut next_state: ResMut<NextState<ProgramState>>,
        mut quit: EventWriter<Quit>,
    ) {
        let ctx = contexts.ctx_mut();

        let button_width = 200.0;
        let button_height = 40.0;
        let margin_b = 20.0;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Main Menu");
                ui.add_space(margin_b);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for item in ProgramState::iter() {
                        if item.eq(&ProgramState::MainMenu) { continue; }

                        if ui.add_sized([button_width, button_height], egui::Button::new(format!("{:?}", item))).clicked() {
                            next_state.set(item);
                        }

                        ui.add_space(margin_b);
                    }
                });

                if ui.add_sized([button_width, button_height], egui::Button::new("Quit")).clicked() {
                    quit.write(Quit);
                }
            });
        });
    }

    pub fn shortcuts(
        program_state: Res<State<ProgramState>>,
        mut next_program_state: ResMut<NextState<ProgramState>>,
        mut ui_state: ResMut<UIState>,
        keys: Res<ButtonInput<KeyCode>>,
        mut quit: EventWriter<Quit>,
    ) {
        if keys.just_pressed(KeyCode::Escape) {
            if program_state.eq(&ProgramState::MainMenu) {
                quit.write(Quit);
            } else {
                next_program_state.set(ProgramState::MainMenu);
            }
        }
        if keys.just_pressed(KeyCode::Backquote) && program_state.ne(&ProgramState::MainMenu) {
            ui_state.params_panel = !ui_state.params_panel;
        }
    }
}

fn setup(mut commands: Commands) {
    // Spawn the camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 16.).looking_at(Vec3::ZERO, Vec3::Y),
        Msaa::Off,
        RenderLayers::default(),
    ));
}

#[derive(Event)]
struct Quit;

fn exit_system(event_reader: EventReader<Quit>, mut exit: EventWriter<AppExit>) {
    if !event_reader.is_empty() {
        exit.write(AppExit::Success);
    }
}
