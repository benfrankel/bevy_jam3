use bevy::prelude::*;
use iyes_progress::prelude::*;

use crate::common::config::ConfigHandle;
use crate::common::window::WindowRoot;
use crate::sequence::SequenceState::*;

pub struct BootStatePlugin;

impl Plugin for BootStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressPlugin::new(Boot).continue_to(Game))
            .add_systems(OnEnter(Boot), enter_boot)
            .add_systems(OnExit(Boot), exit_boot);

        app.add_systems(
            Update,
            wait_for_config.track_progress().run_if(in_state(Boot)),
        );
    }
}

fn enter_boot(window_root: Res<WindowRoot>, mut window_query: Query<&mut Window>) {
    let Ok(mut window) = window_query.get_mut(window_root.primary) else {
        return;
    };

    window.visible = false;
}

fn exit_boot(window_root: Res<WindowRoot>, mut window_query: Query<&mut Window>) {
    let Ok(mut window) = window_query.get_mut(window_root.primary) else {
        return;
    };

    window.visible = true;
}

fn wait_for_config(ass: Res<AssetServer>, config_handle: Res<ConfigHandle>) -> Progress {
    ass.is_loaded_with_dependencies(&config_handle.0).into()
}
