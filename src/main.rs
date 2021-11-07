mod cards;
mod game;
mod player;
mod table;

use crate::player::{OpponentHand, PlayerHand};
use crate::table::{Pile, Table};
use bevy::pbr::AmbientLight;
use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use bevy_easings::EasingsPlugin;

fn main() {
    App::build()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Pile::default())
        .insert_resource(Table::default())
        .insert_resource(PlayerHand::default())
        .insert_resource(OpponentHand::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, cards::setup_card_asset.system())
        .add_startup_system(setup.system())
        .add_startup_system(game::setup_game.system())
        .add_startup_system_to_stage(StartupStage::PostStartup, game::deal_cards.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.5, 0.101)
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
