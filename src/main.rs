use bevy::{pbr::AmbientLight, prelude::*};
use bevy_kira_audio::AudioPlugin;
use controls::ControlsPlugin;
use debug::DebugPlugin;
use dust::DustPlugin;
use impulse::ImpulsePlugin;
use physics::PhysicsPlugin;
use route::RoutePlugin;
use thrust::ThrustPlugin;
use tracking::TrackingPlugin;

mod controls;
mod debug;
mod dust;
mod impulse;
mod physics;
mod route;
mod station;
mod tests;
mod thrust;
mod tracking;
mod ui;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 2.0f32,
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(TrackingPlugin)
        .add_plugin(ImpulsePlugin)
        .add_plugin(ControlsPlugin)
        .add_plugin(RoutePlugin)
        .add_plugin(ThrustPlugin)
        .add_plugin(DustPlugin)
        .add_system(ui::follow_object_system)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let a = tests::station::spawn_station(
        &mut commands,
        &asset_server,
        Vec3::from_slice(&[30.0, 10.0, 30.0]),
        -0.1,
    );
    let b = tests::station::spawn_station(
        &mut commands,
        &asset_server,
        -Vec3::from_slice(&[30.0, 10.0, 30.0]),
        0.3,
    );
    tests::route::spawn_route_ship(&mut commands, &asset_server, vec![a.into(), b.into()]);

    tests::controls::spawn_player_ship(&mut commands, asset_server);
    //commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 50000.0,
            range: 1000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(3.0, 40.0, 30.0),
        ..Default::default()
    });
}
