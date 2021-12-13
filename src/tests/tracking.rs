use bevy::{math::EulerRot, prelude::*};

use crate::{physics::PhysicsBundle, ship::*, tracking::*};

#[allow(dead_code)]
pub fn spawn_tracking_ships(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model = asset_server.load("ship.glb#Scene0");

    let id = {
        commands
            .spawn_bundle(ShipBundle {
                impulse: Impulse(Vec3::from_slice(&[0.0, 0.0, -0.5])),
                angular_impulse: AngularImpulse(Vec3::from_slice(&[0.0, 0.5, 0.0])),
                thrust_characteristics: ThrustCharacteristics {
                    min: Vec3::from_slice(&[-1.0, -2.0, -1.0]),
                    max: Vec3::from_slice(&[1.0, 2.0, 1.0]),
                    rot: Vec3::from_slice(&[0.5, 0.5, 0.5]),
                },
                physics: PhysicsBundle {
                    transform: Transform::from_xyz(0.0, 5.0, 0.0),
                    ..Default::default()
                },
            })
            .with_children(|parent| {
                parent.spawn_scene(model.clone());
            })
            .id()
    };

    commands
        .spawn_bundle(ShipBundle {
            impulse: Impulse(Vec3::from_slice(&[0.0, 0.0, -0.5])),
            thrust_characteristics: ThrustCharacteristics {
                min: Vec3::from_slice(&[-1.0, -1.0, -1.0]),
                max: Vec3::from_slice(&[1.0, 2.0, 1.0]),
                rot: Vec3::from_slice(&[5.0, 5.0, 5.0]),
            },
            physics: PhysicsBundle {
                transform: Transform::from_xyz(5.0, -0.0, -0.0).with_rotation(Quat::from_euler(
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
            parent.spawn_scene(model.clone());
        })
        .insert(TargetEntity(id))
        .insert(Target(Vec3::from_slice(&[0.0, 0.0, 0.0])));
}
