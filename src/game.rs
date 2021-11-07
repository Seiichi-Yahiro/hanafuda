use crate::cards::{CardAssetData, CardType, TextureOffset, CARDS};
use crate::player::{OpponentHand, PlayerHand};
use crate::table::{Pile, Table, CARD_GAP};
use bevy::prelude::*;
use bevy_easings::{Ease, EaseFunction, EaseMethod, EasingType};
use rand::prelude::SliceRandom;
use std::f32::consts::PI;
use std::time::Duration;

pub fn setup_game(
    mut commands: Commands,
    mut pile: ResMut<Pile>,
    card_asset_data: Res<CardAssetData>,
) {
    for (month_index, (month, suits)) in CARDS.iter().enumerate() {
        for (suit_index, suit) in suits.iter().enumerate() {
            let x = suit_index + (month_index / 4) * 4;
            let y = month_index % 4;

            let texture_offset = TextureOffset::new(x as u32, y as u32);
            let card = card_asset_data.create_entity_bundle(texture_offset, Transform::default());

            let entity = commands
                .spawn_bundle(card)
                .insert(CardType {
                    month: *month,
                    suit: *suit,
                })
                .id();

            pile.cards.push(entity);
        }
    }

    pile.cards.shuffle(&mut rand::thread_rng());

    for (index, entity) in pile.cards.iter().enumerate() {
        let transform = Transform {
            translation: Vec3::new(
                -CardAssetData::SIZE_X * 3.0 * CARD_GAP,
                CardAssetData::SIZE_Y / 2.0 + CardAssetData::SIZE_Y * index as f32,
                0.0,
            ),
            rotation: Quat::from_rotation_z(PI),
            scale: Vec3::ONE,
        };

        commands.entity(*entity).insert(transform);
    }
}

pub fn deal_cards(
    mut commands: Commands,
    mut pile: ResMut<Pile>,
    mut table: ResMut<Table>,
    mut player_hand: ResMut<PlayerHand>,
    mut opponent_hand: ResMut<OpponentHand>,
    query: Query<&Transform, With<CardType>>,
) {
    const START_END_ANIMATION_TIME: u64 = 150;
    const MOVE_ANIMATION_TIME: u64 = 500;

    let ease_to = |from: Transform, to: Transform, delay: u64| {
        from.ease_to(
            from,
            EaseMethod::Discrete,
            EasingType::Once {
                duration: Duration::from_millis(delay),
            },
        )
        .ease_to(
            Transform {
                translation: Vec3::new(
                    from.translation.x,
                    from.translation.y + CardAssetData::SIZE_X,
                    from.translation.z,
                ),
                rotation: Quat::from_rotation_z(PI),
                scale: Vec3::ONE,
            },
            EaseFunction::CubicOut,
            EasingType::Once {
                duration: Duration::from_millis(START_END_ANIMATION_TIME),
            },
        )
        .ease_to(
            Transform {
                translation: Vec3::new(
                    to.translation.x,
                    from.translation.y + CardAssetData::SIZE_X,
                    to.translation.z,
                ),
                ..to
            },
            EaseMethod::Linear,
            EasingType::Once {
                duration: Duration::from_millis(MOVE_ANIMATION_TIME),
            },
        )
        .ease_to(
            to,
            EaseFunction::CubicIn,
            EasingType::Once {
                duration: Duration::from_millis(START_END_ANIMATION_TIME),
            },
        )
    };

    const CARDS_TO_DEAL: usize = 2;
    const DEAL_TIMES: usize = 4;
    const CARDS_PER_ROUND: usize = 3 * CARDS_TO_DEAL;

    for round in 0..DEAL_TIMES {
        for card_index in 0..CARDS_TO_DEAL {
            if let Some(entity) = pile.cards.pop() {
                player_hand.cards.push(entity);

                if let Ok(transform) = query.get(entity) {
                    let transform_to =
                        PlayerHand::translation_from_index(round * CARDS_TO_DEAL + card_index);

                    let ease = ease_to(
                        *transform,
                        Transform::from_translation(transform_to),
                        (round * CARDS_PER_ROUND + card_index) as u64 * MOVE_ANIMATION_TIME,
                    );

                    commands.entity(entity).insert(ease);
                }
            }
        }

        for card_index in 0..CARDS_TO_DEAL {
            if let Some(entity) = pile.cards.pop() {
                if let Ok(transform) = query.get(entity) {
                    if let Some(transform_to) =
                        table.add_card(entity).map(Table::translation_from_index)
                    {
                        let ease = ease_to(
                            *transform,
                            Transform::from_translation(transform_to),
                            (round * CARDS_PER_ROUND + CARDS_TO_DEAL + card_index) as u64
                                * MOVE_ANIMATION_TIME,
                        );

                        commands.entity(entity).insert(ease);
                    }
                }
            }
        }

        for card_index in 0..CARDS_TO_DEAL {
            if let Some(entity) = pile.cards.pop() {
                opponent_hand.cards.push(entity);

                if let Ok(transform) = query.get(entity) {
                    let transform_to =
                        OpponentHand::translation_from_index(round * CARDS_TO_DEAL + card_index);

                    let ease = ease_to(
                        *transform,
                        Transform {
                            translation: transform_to,
                            rotation: Quat::from_rotation_z(PI),
                            scale: Vec3::ONE,
                        },
                        (round * CARDS_PER_ROUND + CARDS_TO_DEAL * 2 + card_index) as u64
                            * MOVE_ANIMATION_TIME,
                    );

                    commands.entity(entity).insert(ease);
                }
            }
        }
    }
}
