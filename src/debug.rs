use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::animation::AttackAnimation;
use crate::animation::DeathAnimation;
use crate::animation::Facing;
use crate::animation::FlinchAnimation;
use crate::animation::Lifetime;
use crate::animation::Offset;
use crate::animation::VirtualParent;
use crate::animation::WalkAnimation;
use crate::asset::Handles;
use crate::camera::GameCamera;
use crate::combat::DeathEffects;
use crate::combat::HitEffects;
use crate::combat::HurtEffects;
use crate::cutscene::Cutscene;
use crate::cutscene::Message;
use crate::hud::AlarmMeter;
use crate::hud::FontSizeHack;
use crate::hud::HealthBar;
use crate::map::Exit;
use crate::map::Gate;
use crate::map::Plate;
use crate::map::Wall;
use crate::mob::enemy::Alarm;
use crate::mob::enemy::DifficultyCurve;
use crate::mob::enemy::EnemyAi;
use crate::mob::player::PlayerControl;
use crate::mob::player::Playthrough;
use crate::mob::Body;
use crate::mob::Health;
use crate::mob::Mob;
use crate::mob::MobInputs;
use crate::music::Music;
use crate::util::DespawnSet;
use crate::util::ZRampByY;

const TOGGLE_KEY: KeyCode = KeyCode::F3;

#[derive(Default)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // Plugins
        app.add_plugins((
            RapierDebugRenderPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ));
        app.add_plugins(bevy_editor_pls::EditorPlugin::new().in_new_window(Window {
            title: "bevy_editor_pls".to_string(),
            focused: false,
            ..default()
        }));

        // Systems
        app.add_systems(
            Update,
            DebugPlugin::toggle.run_if(input_just_pressed(TOGGLE_KEY)),
        );

        // Types
        app.register_type::<Handles>()
            .register_type::<Health>()
            .register_type::<Mob>()
            .register_type::<MobInputs>()
            .register_type::<Body>()
            .register_type::<PlayerControl>()
            .register_type::<Playthrough>()
            .register_type::<DifficultyCurve>()
            .register_type::<EnemyAi>()
            .register_type::<HitEffects>()
            .register_type::<HurtEffects>()
            .register_type::<DeathEffects>()
            .register_type::<VirtualParent>()
            .register_type::<ZRampByY>()
            .register_type::<DespawnSet>()
            .register_type::<Offset>()
            .register_type::<WalkAnimation>()
            .register_type::<AttackAnimation>()
            .register_type::<FlinchAnimation>()
            .register_type::<DeathAnimation>()
            .register_type::<Facing>()
            .register_type::<Lifetime>()
            .register_type::<Wall>()
            .register_type::<Exit>()
            .register_type::<Plate>()
            .register_type::<Gate>()
            .register_type::<Music>()
            .register_type::<GameCamera>()
            .register_type::<Cutscene>()
            .register_type::<Message>()
            .register_type::<FontSizeHack>()
            .register_type::<HealthBar>()
            .register_type::<Alarm>()
            .register_type::<AlarmMeter>();

        // Disable Rapier debug initially
        app.world.resource_mut::<DebugRenderContext>().enabled = false;
    }
}

impl DebugPlugin {
    fn toggle(mut debug_render_context: ResMut<DebugRenderContext>) {
        debug_render_context.enabled = !debug_render_context.enabled;
    }
}
