mod circle;

use bevy::prelude::*;
use bevy::window::ExitCondition;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiContextPass};
use bevy_vector_shapes::prelude::*;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::circle::CircleArt;

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
            Shape2dPlugin::default(),
        ))
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        .add_event::<Quit>()
        .init_state::<ProgramState>()
        .add_systems(EguiContextPass, ProgramState::selection_system.run_if(in_state(ProgramState::MainMenu)))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            ProgramState::shortcuts,
            exit_system,
            ))
        .add_plugins(
            CircleArt,
        )
        .run();
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, EnumIter)]
enum ProgramState {
    #[default]
    MainMenu,
    Circle,
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
    }
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn setup(mut commands: Commands) {
    // Spawn the camera
    commands.spawn(Camera2d);
}

#[derive(Event)]
struct Quit;

fn exit_system(event_reader: EventReader<Quit>, mut exit: EventWriter<AppExit>) {
    if !event_reader.is_empty() {
        exit.write(AppExit::Success);
    }
}
