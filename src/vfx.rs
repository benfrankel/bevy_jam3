use bevy::prelude::*;

use crate::{
    animation::Offset,
    asset::{Handles, ImageKey},
};

pub struct DropShadowTemplate {
    pub parent_z: f32,
    pub offset: Vec2,
}

impl DropShadowTemplate {
    pub fn spawn(self, commands: &mut Commands, handle: &Handles) -> Entity {
        let mut drop_shadow = commands.spawn((
            SpriteBundle {
                texture: handle.image[&ImageKey::DropShadow].clone(),
                transform: Transform::from_xyz(0.0, 0.0, 50.0 - self.parent_z),
                ..default()
            },
            Offset(self.offset),
        ));
        #[cfg(feature = "debug_mode")]
        drop_shadow.insert(Name::new("DropShadow"));

        drop_shadow.id()
    }
}
