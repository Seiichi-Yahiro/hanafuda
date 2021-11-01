mod cards;

use bevy::pbr::AmbientLight;
use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;

fn main() {
    App::build()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(cards::setup_card_asset.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.5, 0.001)
            .looking_at(Vec3::new(0.0, 0.001, 0.0), Vec3::Y),
        perspective_projection: PerspectiveProjection {
            near: 0.01,
            far: 5.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.5, 5.5, 0.3),
        ..Default::default()
    });
}
