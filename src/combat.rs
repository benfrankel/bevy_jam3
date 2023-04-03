use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
    animation::{Lifetime, Offset},
    mob::{
        player::{Gold, PlayerControl},
        Health,
    },
    util::{DespawnSet, VirtualParent}, asset::{Handles, AudioKey},
};

pub const COLLISION_GROUP: Group = Group::GROUP_1;
pub const HITBOX_GROUP: Group = Group::GROUP_2;
pub const PLAYER_HURTBOX_GROUP: Group = Group::GROUP_3;
pub const ENEMY_HURTBOX_GROUP: Group = Group::GROUP_4;

#[derive(Copy, Clone)]
pub enum Faction {
    Player,
    Enemy,
}

impl Faction {
    pub fn hitbox_groups(&self) -> CollisionGroups {
        CollisionGroups {
            memberships: HITBOX_GROUP,
            filters: match self {
                Faction::Player => ENEMY_HURTBOX_GROUP,
                Faction::Enemy => PLAYER_HURTBOX_GROUP,
            },
        }
    }

    pub fn hurtbox_groups(&self) -> CollisionGroups {
        CollisionGroups {
            memberships: match self {
                Faction::Player => PLAYER_HURTBOX_GROUP,
                Faction::Enemy => ENEMY_HURTBOX_GROUP,
            },
            filters: HITBOX_GROUP,
        }
    }
}

pub struct HitboxTemplate {
    pub offset: Vec2,
    pub radius: f32,
    pub damage: f32,
    pub knockback: f32,
    pub faction: Faction,
    pub lifetime: f32,
    pub parent: Entity,
}

impl HitboxTemplate {
    pub fn spawn(self, commands: &mut Commands, handle: &Handles) -> Entity {
        let mut entity = commands.spawn((
            Offset(self.offset),
            TransformBundle::default(),
            Collider::ball(self.radius),
            Sensor,
            self.faction.hitbox_groups(),
            ActiveEvents::COLLISION_EVENTS,
            HitEffects {
                damage: self.damage,
                knockback: self.knockback,
				sound: Some(handle.audio[&AudioKey::PlayerAttack2].clone()),
            },
            Lifetime(self.lifetime),
            VirtualParent(self.parent),
        ));
        #[cfg(feature = "debug_mode")]
        entity.insert(Name::new("Hitbox"));

        entity.id()
    }
}

pub struct HitEvent {
    actor: Entity,
    hitbox: Entity,
    target: Entity,
}

impl HitEvent {
    pub fn detect(
        mut collision_events: EventReader<CollisionEvent>,
        mut hit_events: EventWriter<HitEvent>,
        hit_query: Query<&VirtualParent, With<HitEffects>>,
    ) {
        for &event in collision_events.iter() {
            let CollisionEvent::Started(entity1, entity2, _) = event else {
                continue
            };

            let mut handle_collision = |hitbox: Entity, target: Entity| {
                let Ok(&VirtualParent(actor)) = hit_query.get(hitbox) else { return };
                hit_events.send(HitEvent {
                    actor,
                    hitbox,
                    target,
                });
            };

            handle_collision(entity1, entity2);
            handle_collision(entity2, entity1);
        }
    }
}

#[derive(Component, Reflect)]
pub struct HitEffects {
    damage: f32,
    knockback: f32,
	sound: Option<Handle<AudioSource>>,
}

impl HitEffects {
    pub fn apply(
        mut hit_events: EventReader<HitEvent>,
        mut death_events: EventWriter<DeathEvent>,
        hit_effects_query: Query<&HitEffects>,
        mut health_query: Query<&mut Health>,
        mut velocity_query: Query<&mut Velocity>,
        transform_query: Query<&Transform>,
		mut audio: ResMut<Audio>,
    ) {
        for &HitEvent {
            actor,
            hitbox,
            target,
        } in hit_events.iter()
        {
            let Ok(effect) = hit_effects_query.get(hitbox) else { return };
			
			if let Some(sound) = &effect.sound {
				audio.play(sound.clone());
			}

            // Damage
            if let Ok(mut health) = health_query.get_mut(target) {
                if 0.0 < health.0 && health.0 <= effect.damage {
                    death_events.send(DeathEvent(target));
                }
                health.0 -= effect.damage;
            }

            // Knockback
            if let Ok(mut velocity) = velocity_query.get_mut(target) {
                let Ok(actor_transform) = transform_query.get(actor) else {
                    return
                };
                let Ok(target_transform) = transform_query.get(target) else {
                    return
                };

                let scale = 40.0;
                let direction = (target_transform.translation.xy()
                    - actor_transform.translation.xy())
                .normalize_or_zero();
                velocity.linvel = effect.knockback * scale * direction;
            }
        }
    }
}

pub struct DeathEvent(pub Entity);

#[derive(Component, Reflect)]
pub struct DeathEffects {
    pub reward_gold: f32,
    // TODO: Animation, sound effect
}

impl Default for DeathEffects {
    fn default() -> Self {
        Self { reward_gold: 10.0 }
    }
}

impl DeathEffects {
    pub fn apply(
        mut death_events: EventReader<DeathEvent>,
        mut despawn: ResMut<DespawnSet>,
        death_effects_query: Query<&DeathEffects>,
        mut player_query: Query<&mut Gold, With<PlayerControl>>,
    ) {
        for &DeathEvent(entity) in death_events.iter() {
            // Despawn
            despawn.0.insert(entity);

            let Ok(death_effects) = death_effects_query.get(entity) else {
                continue
            };

            // Reward gold
            for mut player_gold in &mut player_query {
                player_gold.0 += death_effects.reward_gold;
            }
        }
    }
}
