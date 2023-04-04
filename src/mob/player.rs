use bevy::{
    math::{vec2, Vec3Swizzles},
    prelude::*,
};

use super::{Health, Mob, MobBundle, MobInputs};
use crate::{
    asset::{Handles, ImageKey},
    camera::CameraFollow,
    combat::Faction,
    hud::{HealthBarTemplate, NametagTemplate},
    mob::BodyTemplate,
    vfx::DropShadowTemplate,
};

const PLAYER_NAME: &str = "Sai";

#[derive(Component, Reflect, Default)]
pub struct Gold(pub f32);

#[derive(Component, Reflect, Default, Debug)]
pub struct PlayerControl;

impl PlayerControl {
    pub fn record_inputs(
        mut player_query: Query<(&mut MobInputs, &GlobalTransform), With<PlayerControl>>,
        key_input_resource: Res<Input<KeyCode>>,
        mouse_input_resource: Res<Input<MouseButton>>,
        windows: Query<&Window>,
        camera: Query<(&Camera, &GlobalTransform), With<CameraFollow<PlayerControl>>>,
    ) {
        for (mut mob_inputs, mob_gt) in &mut player_query {
            let mut movement = Vec2::ZERO;

            // It'd be nice to make bindings for this, but hey, it's a gamejam.
            // we could look at leafwing_input_manager
            if key_input_resource.pressed(KeyCode::A) {
                movement.x -= 1.0;
            }
            if key_input_resource.pressed(KeyCode::D) {
                movement.x += 1.0;
            }
            if key_input_resource.pressed(KeyCode::W) {
                movement.y += 1.0;
            }
            if key_input_resource.pressed(KeyCode::S) {
                movement.y -= 1.0;
            }
            mob_inputs.movement = movement;

            let window = windows.single();
            let (camera, cam_gt) = camera.single();

            mob_inputs.attack = None;
            if mouse_input_resource.just_pressed(MouseButton::Left) {
                if let Some(position) = window.cursor_position() {
                    if let Some(pos) = camera.viewport_to_world_2d(cam_gt, position) {
                        let dir = pos - mob_gt.translation().xy();
                        mob_inputs.attack = Some(dir.normalize());
                    }
                }
            }
        }
    }
}

#[derive(Component, Reflect)]
pub struct PlayerTemplate {
    pub position: Vec2,
    pub health: f32,
}

impl Default for PlayerTemplate {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            health: 100.0,
        }
    }
}

impl PlayerTemplate {
    pub fn spawn(self, commands: &mut Commands, handle: &Handles) -> Entity {
        const HEALTH: f32 = 100.0;
        const FACTION: Faction = Faction::Player;

        // Children
        let body = BodyTemplate {
            texture: ImageKey::GreenGnoll,
            offset: vec2(2.0, 11.0),
        }
        .spawn(commands, handle);
        let drop_shadow = DropShadowTemplate::default().spawn(commands, handle);
        let nametag = NametagTemplate {
            offset: vec2(0.0, 26.0),
            name: PLAYER_NAME.to_string(),
        }
        .spawn(commands, handle);
        let health_bar = HealthBarTemplate {
            offset: vec2(0.0, -6.0),
        }
        .spawn(commands);

        // Parent
        let mut player = commands.spawn((
            SpatialBundle {
                transform: Transform::from_translation(self.position.extend(0.0)),
                ..default()
            },
            MobBundle {
                mob: Mob::player(),
                health: Health::full(HEALTH),
                ..default()
            }
            .with_faction(FACTION),
            PlayerControl,
        ));
        #[cfg(feature = "debug_mode")]
        player.insert(Name::new("Player"));

        player.add_child(body);
        player.add_child(drop_shadow);
        player.add_child(nametag);
        player.add_child(health_bar);
        let player = player.id();

        player
    }
}
