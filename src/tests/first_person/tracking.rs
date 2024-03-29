use bevy::{math::EulerRot, prelude::*};

use crate::{impulse::*, physics::*, tracking::*};

#[allow(dead_code)]
pub fn spawn_tracking_ships(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model = asset_server.load("models/ship_small_thrust.glb#Scene0");

    let id = {
        // Leader
        commands
            .spawn(ShipBundle {
                thrust_characteristics: ThrustCharacteristics {
                    min: Vec3::from_slice(&[-1.0, -2.0, -1.0]),
                    max: Vec3::from_slice(&[1.0, 2.0, 1.0]),
                    rot: Vec3::from_slice(&[0.01, 0.01, 0.01]),
                },
                physics: PhysicsBundle {
                    velocity: Velocity(Vec3::from_slice(&[0.0, 0.0, -0.0])),
                    ..Default::default()
                },
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.2, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(SceneBundle {
                    scene: model.clone(),
                    ..Default::default()
                });
            })
            .id()
    };

    // Follower
    commands
        .spawn(ShipBundle {
            impulse: Impulse(Vec3::from_slice(&[0.0, 0.0, -0.0])),
            thrust_characteristics: ThrustCharacteristics {
                min: Vec3::from_slice(&[-1.0, -2.0, -1.0]),
                max: Vec3::from_slice(&[1.0, 2.0, 1.0]),
                rot: Vec3::from_slice(&[0.01, 0.01, 0.01]),
            },
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0.1, -0.0, -0.0).with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    0.0,
                    1.0,
                    0.0,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: model.clone(),
                ..Default::default()
            });
        })
        .insert(TargetEntity(id))
        .insert(Target(Vec3::from_slice(&[0.0, 0.0, 0.0])));
}
