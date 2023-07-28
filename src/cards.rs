use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::utils::HashMap;
use std::cmp::Ordering;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Resource)]
pub struct CardAssetData {
    color_texture: Handle<Image>,
    roughness_texture: Handle<Image>,
    normal_texture: Handle<Image>,
    meshes: HashMap<Card, Handle<Mesh>>,
}

impl CardAssetData {
    pub const SIZE_X: f32 = 3.2;
    pub const SIZE_Y: f32 = 5.4;
    pub const SIZE_Z: f32 = 0.1;

    pub const TEXTURE_ROWS: u32 = 13;
    pub const TEXTURE_COLUMNS: u32 = 4;

    fn create_mesh(x: u32, y: u32) -> Mesh {
        assert!(
            x < Self::TEXTURE_COLUMNS,
            "x index only valid for 0..{} but was {}",
            Self::TEXTURE_COLUMNS,
            x
        );
        assert!(
            y < Self::TEXTURE_ROWS,
            "y index only valid for 0..{} but was {}",
            Self::TEXTURE_ROWS,
            y
        );

        let half_x_size = Self::SIZE_X / 2.0;
        let half_y_size = Self::SIZE_Y / 2.0;
        let half_z_size = Self::SIZE_Z / 2.0;

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                // cover
                [-half_x_size, half_y_size, half_z_size],
                [-half_x_size, -half_y_size, half_z_size],
                [half_x_size, -half_y_size, half_z_size],
                [half_x_size, half_y_size, half_z_size],
                // back_cover
                [-half_x_size, half_y_size, -half_z_size],
                [-half_x_size, -half_y_size, -half_z_size],
                [half_x_size, -half_y_size, -half_z_size],
                [half_x_size, half_y_size, -half_z_size],
                // bottom
                [-half_x_size, -half_y_size, half_z_size],
                [-half_x_size, -half_y_size, -half_z_size],
                [half_x_size, -half_y_size, -half_z_size],
                [half_x_size, -half_y_size, half_z_size],
                // top
                [-half_x_size, half_y_size, half_z_size],
                [-half_x_size, half_y_size, -half_z_size],
                [half_x_size, half_y_size, -half_z_size],
                [half_x_size, half_y_size, half_z_size],
                // left
                [-half_x_size, half_y_size, half_z_size],
                [-half_x_size, half_y_size, -half_z_size],
                [-half_x_size, -half_y_size, -half_z_size],
                [-half_x_size, -half_y_size, half_z_size],
                // right
                [half_x_size, half_y_size, half_z_size],
                [half_x_size, half_y_size, -half_z_size],
                [half_x_size, -half_y_size, -half_z_size],
                [half_x_size, -half_y_size, half_z_size],
            ],
        );

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                //
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                //
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                //
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                //
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                //
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
            ],
        );

        let uv_x = 1.0 / Self::TEXTURE_COLUMNS as f32;
        let uv_y = 1.0 / Self::TEXTURE_ROWS as f32;

        let x = x as f32;
        let y = y as f32;

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                [uv_x * x, uv_y * y],
                [uv_x * x, uv_y * (y + 1.0)],
                [uv_x * (x + 1.0), uv_y * (y + 1.0)],
                [uv_x * (x + 1.0), uv_y * y],
                //
                [uv_x * x, 1.0 - uv_y],
                [uv_x * x, 1.0],
                [uv_x * (x + 1.0), 1.0],
                [uv_x * (x + 1.0), 1.0 - uv_y],
                //
                [uv_x * x, 1.0 - uv_y * 0.03],
                [uv_x * x, 1.0],
                [uv_x * (x + 1.0), 1.0],
                [uv_x * (x + 1.0), 1.0 - uv_y * 0.03],
                //
                [uv_x * x, 1.0 - uv_y * 0.03],
                [uv_x * x, 1.0],
                [uv_x * (x + 1.0), 1.0],
                [uv_x * (x + 1.0), 1.0 - uv_y * 0.03],
                //
                [uv_x * x, 1.0 - uv_y],
                [uv_x * x, 1.0],
                [uv_x * x + uv_x * 0.05, 1.0],
                [uv_x * x + uv_x * 0.05, 1.0 - uv_y],
                //
                [uv_x * x, 1.0 - uv_y],
                [uv_x * x, 1.0],
                [uv_x * x + uv_x * 0.05, 1.0],
                [uv_x * x + uv_x * 0.05, 1.0 - uv_y],
            ],
        );

        mesh.set_indices(Some(Indices::U32(vec![
            0, 1, 3, 3, 1, 2, //
            4, 7, 5, 5, 7, 6, //
            8, 9, 11, 11, 9, 10, //
            12, 15, 13, 13, 15, 14, //
            16, 17, 19, 19, 17, 18, //
            20, 23, 21, 21, 23, 22, //
        ])));

        mesh.generate_tangents().unwrap();

        mesh
    }

    pub fn get_color_texture(&self) -> Handle<Image> {
        self.color_texture.clone()
    }

    pub fn get_roughness_texture(&self) -> Handle<Image> {
        self.roughness_texture.clone()
    }

    pub fn get_normal_texture(&self) -> Handle<Image> {
        self.normal_texture.clone()
    }

    pub fn get_mesh(&self, card: Card) -> Handle<Mesh> {
        self.meshes.get(&card).unwrap().clone()
    }

    pub fn create_material(&self) -> StandardMaterial {
        StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(self.get_color_texture()),
            perceptual_roughness: 1.0,
            metallic_roughness_texture: Some(self.get_roughness_texture()),
            normal_map_texture: Some(self.get_normal_texture()),
            ..default()
        }
    }
}

impl FromWorld for CardAssetData {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let color_texture = asset_server.load("textures/card_color.jpg");
        let roughness_texture = asset_server.load("textures/card_roughness.png");
        let normal_texture = asset_server.load("textures/card_normal.png");

        let mut mesh_assets = world.resource_mut::<Assets<Mesh>>();

        let meshes = Card::iter()
            .map(|card| (card, mesh_assets.add(card.create_mesh().into())))
            .collect();

        Self {
            color_texture,
            roughness_texture,
            normal_texture,
            meshes,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter)]
pub enum Card {
    JanuaryHikari,
    JanuaryPoetryTanzaku,
    JanuaryKasu1,
    JanuaryKasu2,
    FebruaryTane,
    FebruaryPoetryTanzaku,
    FebruaryKasu1,
    FebruaryKasu2,
    MarchHikari,
    MarchPoetryTanzaku,
    MarchKasu1,
    MarchKasu2,
    AprilTane,
    AprilPlainTanzaku,
    AprilKasu1,
    AprilKasu2,
    MayTane,
    MayPlainTanzaku,
    MayKasu1,
    MayKasu2,
    JuneTane,
    JuneBlueTanzaku,
    JuneKasu1,
    JuneKasu2,
    JulyTane,
    JulyPlainTanzaku,
    JulyKasu1,
    JulyKasu2,
    AugustHikari,
    AugustTane,
    AugustKasu1,
    AugustKasu2,
    SeptemberTane,
    SeptemberBlueTanzaku,
    SeptemberKasu1,
    SeptemberKasu2,
    OctoberTane,
    OctoberBlueTanzaku,
    OctoberKasu1,
    OctoberKasu2,
    NovemberHikari,
    NovemberTane,
    NovemberPlainTanzaku,
    NovemberKasu,
    DecemberHikari,
    DecemberKasu1,
    DecemberKasu2,
    DecemberKasu3,
}

impl Card {
    fn create_mesh(&self) -> Mesh {
        Self::iter()
            .enumerate()
            .find_map(|(index, card)| {
                if *self == card {
                    Some(CardAssetData::create_mesh(
                        index as u32 % CardAssetData::TEXTURE_COLUMNS,
                        index as u32 / CardAssetData::TEXTURE_COLUMNS,
                    ))
                } else {
                    None
                }
            })
            .unwrap()
    }

    fn to_suit(self) -> Suit {
        match self {
            Card::JanuaryHikari => Suit::Hikari(Hikari::Crane),
            Card::MarchHikari => Suit::Hikari(Hikari::Curtain),
            Card::AugustHikari => Suit::Hikari(Hikari::Moon),
            Card::NovemberHikari => Suit::Hikari(Hikari::Michikaze),
            Card::DecemberHikari => Suit::Hikari(Hikari::Phoenix),

            Card::FebruaryTane => Suit::Tane(Tane::BushWarbler),
            Card::AprilTane => Suit::Tane(Tane::Cuckoo),
            Card::MayTane => Suit::Tane(Tane::Bridge),
            Card::JuneTane => Suit::Tane(Tane::Butterfly),
            Card::JulyTane => Suit::Tane(Tane::Boar),
            Card::AugustTane => Suit::Tane(Tane::Goose),
            Card::SeptemberTane => Suit::Tane(Tane::Sake),
            Card::OctoberTane => Suit::Tane(Tane::Deer),
            Card::NovemberTane => Suit::Tane(Tane::Swallow),

            Card::JanuaryPoetryTanzaku | Card::FebruaryPoetryTanzaku | Card::MarchPoetryTanzaku => {
                Suit::Tanzaku(Tanzaku::Poetry)
            }
            Card::AprilPlainTanzaku
            | Card::MayPlainTanzaku
            | Card::JulyPlainTanzaku
            | Card::NovemberPlainTanzaku => Suit::Tanzaku(Tanzaku::Plain),
            Card::JuneBlueTanzaku | Card::SeptemberBlueTanzaku | Card::OctoberBlueTanzaku => {
                Suit::Tanzaku(Tanzaku::Blue)
            }

            Card::JanuaryKasu1
            | Card::JanuaryKasu2
            | Card::FebruaryKasu1
            | Card::FebruaryKasu2
            | Card::MarchKasu1
            | Card::MarchKasu2
            | Card::AprilKasu1
            | Card::AprilKasu2
            | Card::MayKasu1
            | Card::MayKasu2
            | Card::JuneKasu1
            | Card::JuneKasu2
            | Card::JulyKasu1
            | Card::JulyKasu2
            | Card::AugustKasu1
            | Card::AugustKasu2
            | Card::SeptemberKasu1
            | Card::SeptemberKasu2
            | Card::OctoberKasu1
            | Card::OctoberKasu2
            | Card::NovemberKasu
            | Card::DecemberKasu1
            | Card::DecemberKasu2
            | Card::DecemberKasu3 => Suit::Kasu,
        }
    }

    pub fn to_month(self) -> Month {
        match self {
            Card::JanuaryHikari
            | Card::JanuaryPoetryTanzaku
            | Card::JanuaryKasu1
            | Card::JanuaryKasu2 => Month::January,
            Card::FebruaryTane
            | Card::FebruaryPoetryTanzaku
            | Card::FebruaryKasu1
            | Card::FebruaryKasu2 => Month::February,
            Card::MarchHikari | Card::MarchPoetryTanzaku | Card::MarchKasu1 | Card::MarchKasu2 => {
                Month::March
            }
            Card::AprilTane | Card::AprilPlainTanzaku | Card::AprilKasu1 | Card::AprilKasu2 => {
                Month::April
            }
            Card::MayTane | Card::MayPlainTanzaku | Card::MayKasu1 | Card::MayKasu2 => Month::May,
            Card::JuneTane | Card::JuneBlueTanzaku | Card::JuneKasu1 | Card::JuneKasu2 => {
                Month::June
            }
            Card::JulyTane | Card::JulyPlainTanzaku | Card::JulyKasu1 | Card::JulyKasu2 => {
                Month::July
            }
            Card::AugustHikari | Card::AugustTane | Card::AugustKasu1 | Card::AugustKasu2 => {
                Month::August
            }
            Card::SeptemberTane
            | Card::SeptemberBlueTanzaku
            | Card::SeptemberKasu1
            | Card::SeptemberKasu2 => Month::September,
            Card::OctoberTane
            | Card::OctoberBlueTanzaku
            | Card::OctoberKasu1
            | Card::OctoberKasu2 => Month::October,
            Card::NovemberHikari
            | Card::NovemberTane
            | Card::NovemberPlainTanzaku
            | Card::NovemberKasu => Month::November,
            Card::DecemberHikari
            | Card::DecemberKasu1
            | Card::DecemberKasu2
            | Card::DecemberKasu3 => Month::December,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Suit {
    Hikari(Hikari),
    Tane(Tane),
    Tanzaku(Tanzaku),
    Kasu,
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Hikari(_), Self::Hikari(_))
            | (Self::Tane(_), Self::Tane(_))
            | (Self::Tanzaku(_), Self::Tanzaku(_))
            | (Self::Kasu, Self::Kasu) => Some(Ordering::Equal),

            (Self::Hikari(_), Self::Tane(_) | Self::Tanzaku(_) | Self::Kasu)
            | (Self::Tane(_), Self::Tanzaku(_) | Self::Kasu)
            | (Self::Tanzaku(_), Self::Kasu) => Some(Ordering::Greater),

            (Self::Kasu, Self::Tanzaku(_) | Self::Tane(_) | Self::Hikari(_))
            | (Self::Tanzaku(_), Self::Tane(_) | Self::Hikari(_))
            | (Self::Tane(_), Self::Hikari(_)) => Some(Ordering::Less),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Hikari {
    Crane,
    Curtain,
    Moon,
    Michikaze,
    Phoenix,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tane {
    BushWarbler,
    Cuckoo,
    Bridge,
    Butterfly,
    Boar,
    Goose,
    Sake,
    Deer,
    Swallow,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tanzaku {
    Poetry,
    Plain,
    Blue,
}
