mod boot;
pub mod game;

use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::ui::Val::*;
use strum::EnumIter;

use crate::common::theme::ThemeBackgroundColor;
use crate::common::theme::ThemeColor;
use crate::util::animation::transition::FadeIn;
use crate::util::animation::transition::FadeOut;

pub struct SequencePlugin;

impl Plugin for SequencePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SequenceState>()
            .add_plugins((boot::BootStatePlugin, game::GameStatePlugin));
    }
}

#[derive(States, Reflect, Default, Copy, Clone, Eq, PartialEq, Hash, Debug, EnumIter)]
pub enum SequenceState {
    #[default]
    Boot,
    // TODO: Workaround for https://github.com/bevyengine/bevy/issues/9130
    RestartGame,
    Game,
}

const FADE_IN_SECS: f32 = 0.1;

fn fade_in(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("ScreenFadeIn"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Percent(100.0),
                    height: Percent(100.0),
                    ..default()
                },
                focus_policy: FocusPolicy::Block,
                z_index: ZIndex::Global(1000),
                ..default()
            },
            ThemeBackgroundColor(ThemeColor::Body),
            FadeIn::new(FADE_IN_SECS),
        ))
        .id()
}

const FADE_OUT_SECS: f32 = 0.1;

fn fade_out(commands: &mut Commands, next_state: SequenceState) -> Entity {
    commands
        .spawn((
            Name::new("ScreenFadeOut"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Percent(100.0),
                    height: Percent(100.0),
                    ..default()
                },
                focus_policy: FocusPolicy::Block,
                z_index: ZIndex::Global(1000),
                ..default()
            },
            ThemeBackgroundColor(ThemeColor::Body),
            FadeOut::new(FADE_OUT_SECS, next_state),
        ))
        .id()
}
