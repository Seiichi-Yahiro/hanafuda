use crate::cards::CardAssetData;
use crate::table::CARD_GAP;
use bevy::prelude::*;

#[derive(Debug)]
pub struct PlayerHand {
    pub cards: Vec<Entity>,
}

impl Default for PlayerHand {
    fn default() -> Self {
        Self {
            cards: Vec::with_capacity(8),
        }
    }
}

impl PlayerHand {
    pub fn translation_from_index(index: usize) -> Vec3 {
        Vec3::new(
            CardAssetData::SIZE_X * index as f32 * CARD_GAP
                - CardAssetData::SIZE_X * 4.0 * CARD_GAP,
            CardAssetData::SIZE_Y / 2.0,
            CardAssetData::SIZE_Z * 1.75,
        )
    }
}

#[derive(Debug)]
pub struct OpponentHand {
    pub cards: Vec<Entity>,
}

impl Default for OpponentHand {
    fn default() -> Self {
        Self {
            cards: Vec::with_capacity(8),
        }
    }
}

impl OpponentHand {
    pub fn translation_from_index(index: usize) -> Vec3 {
        Vec3::new(
            -CardAssetData::SIZE_X * index as f32 * CARD_GAP
                + CardAssetData::SIZE_X * 3.0 * CARD_GAP,
            CardAssetData::SIZE_Y / 2.0,
            -CardAssetData::SIZE_Z * 1.75,
        )
    }
}
