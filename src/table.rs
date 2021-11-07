use crate::cards::CardAssetData;
use bevy::prelude::*;

pub const CARD_GAP: f32 = 1.05;

#[derive(Debug)]
pub struct Pile {
    pub cards: Vec<Entity>,
}

impl Default for Pile {
    fn default() -> Self {
        Self {
            cards: Vec::with_capacity(12 * 4),
        }
    }
}

#[derive(Debug, Default)]
pub struct Table {
    cards: [Option<Entity>; 12],
}

impl Table {
    pub fn add_card(&mut self, entity: Entity) -> Option<usize> {
        let found_slot = self
            .cards
            .iter_mut()
            .enumerate()
            .find(|(_, slot)| slot.is_none());

        if let Some((index, slot)) = found_slot {
            *slot = Some(entity);
            return Some(index);
        }

        None
    }

    pub fn translation_from_index(index: usize) -> Vec3 {
        Vec3::new(
            CardAssetData::SIZE_X * (index / 2) as f32 * CARD_GAP
                - CardAssetData::SIZE_X * 2.0 * CARD_GAP,
            CardAssetData::SIZE_Y / 2.0,
            CardAssetData::SIZE_Z * (index % 2) as f32 * CARD_GAP
                - CardAssetData::SIZE_Z / 2.0 * CARD_GAP,
        )
    }
}
