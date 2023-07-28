mod cards;

use crate::cards::{Card, CardAssetData};
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::PI;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .init_resource::<CardAssetData>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    card_asset_data: Res<CardAssetData>,
) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, -0.01, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(ScreenSpaceAmbientOcclusionBundle::default())
        .insert(TemporalAntiAliasBundle::default());

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 25_000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 50.0),
            rotation: Quat::from_rotation_x(PI / 4.0),
            ..default()
        },
        ..default()
    });

    commands
        .spawn(PbrBundle {
            mesh: card_asset_data.get_mesh(Card::NovemberHikari),
            material: materials.add(card_asset_data.create_material()),
            ..default()
        })
        .insert(Name::new("Card"));
}
