use amethyst::{
    audio::output::init_output,
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{UiCreator, UiTransform, UiEventType, UiEvent},
    winit::VirtualKeyCode,
};

use crate::state::GameplayState;
use crate::resource::game::{GameResource, GameState};

pub struct MainMenuState;

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        init_output(&mut world.res);

        world.add_resource::<GameResource>(GameResource::default());
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/main_menu.ron", ())
        });
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let StateData { world, .. } = data;
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                let ui_transform_components = world.read_storage::<UiTransform>();
                let selected_component = ui_transform_components.get(ui_event.target);

                if pressed_play_button(world, selected_component.unwrap(), ui_event) {
                    return Trans::Push(Box::new(GameplayState::new()));
                }
                Trans::None
            }
            _ => Trans::None
        }
    }
}

fn pressed_play_button(world: &World, selected_component: &UiTransform, ui_event: &UiEvent) -> bool {
    if selected_component.id == "play_button" && ui_event.event_type == UiEventType::Click {
        world.write_resource::<GameResource>().current_state = GameState::Gameplay;
        return true;
    } else {
        return false;
    }
}
