mod cards;
mod game;
mod player;
mod table;

use crate::cards::CardAssetData;
use crate::player::{OpponentHand, PlayerHand};
use crate::table::{Pile, Table};
use bevy::asset::LoadState;
use bevy::pbr::AmbientLight;
use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use bevy_easings::EasingsPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Load,
    Game,
}

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
        .add_state(AppState::Load)
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        .add_startup_system(cards::setup_card_asset.system())
        .add_system_set(SystemSet::on_update(AppState::Load).with_system(check_load_state.system()))
        .add_system_set(
            SystemSet::on_exit(AppState::Load)
                .with_system(setup.system())
                .with_system(game::setup_game.system()),
        )
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(game::deal_cards.system()))
        .run();
}

fn check_load_state(
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
    card_asset_data: Res<CardAssetData>,
) {
    match asset_server.get_group_load_state(card_asset_data.handle_ids()) {
        LoadState::Failed => {
            eprintln!("Failed to load Assets!")
        }
        LoadState::Loaded => {
            state.set(AppState::Game).unwrap();
        }
        LoadState::Loading | LoadState::NotLoaded => {}
    }
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
