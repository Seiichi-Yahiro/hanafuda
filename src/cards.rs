use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::render_graph::{base, RenderGraph, RenderResourcesNode};
use bevy::render::renderer::RenderResources;
use bevy::render::shader::{ShaderStage, ShaderStages};

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
pub struct TextureOffset {
    value: Vec2,
}

impl TextureOffset {
    fn new(x: u32, y: u32) -> Self {
        Self {
            value: Vec2::new(x as f32, y as f32),
        }
    }
}

#[derive(Bundle)]
pub struct CardBundle {
    #[bundle]
    pub pbr: PbrBundle,
    pub texture_offset: TextureOffset,
}

pub struct CardAssetData {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    pipeline: Handle<PipelineDescriptor>,
}

impl CardAssetData {
    pub const SIZE_X: f32 = 0.058;
    pub const SIZE_Y: f32 = 0.001;
    pub const SIZE_Z: f32 = 0.0865;

    pub fn create_entity_bundle(
        &self,
        texture_offset: TextureOffset,
        transform: Transform,
    ) -> CardBundle {
        CardBundle {
            pbr: PbrBundle {
                mesh: self.mesh.clone(),
                material: self.material.clone(),
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    self.pipeline.clone(),
                )]),
                transform,
                ..Default::default()
            },
            texture_offset,
        }
    }
}

pub fn setup_card_asset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let mesh = asset_server.load("mesh/card.gltf#Mesh0/Primitive0");

    #[cfg(not(debug_assertions))]
    let material = {
        let color = asset_server.load("textures/cards.jpg");
        let roughness = asset_server.load("textures/card_roughness.png");
        let normal = asset_server.load("textures/card_normal.png");

        StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(color),
            roughness: 1.0,
            metallic_roughness_texture: Some(roughness),
            normal_map: Some(normal),
            ..Default::default()
        }
    };

    #[cfg(debug_assertions)]
    let material = {
        let color = asset_server.load("textures/cards_debug.jpg");

        StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(color),
            roughness: 0.2,
            ..Default::default()
        }
    };

    let material = materials.add(material);

    let pipeline = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("../assets/shaders/card.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("../assets/shaders/card.frag"),
        ))),
    }));

    let card_asset_data = CardAssetData {
        mesh,
        material,
        pipeline,
    };

    render_graph.add_system_node(
        "texture_offset",
        RenderResourcesNode::<TextureOffset>::new(true),
    );

    render_graph
        .add_node_edge("texture_offset", base::node::MAIN_PASS)
        .unwrap();

    for (month_index, (month, suits)) in CARDS.iter().enumerate() {
        for (suit_index, suit) in suits.iter().enumerate() {
            let x = suit_index + (month_index / 4) * 4;
            let y = month_index % 4;

            let texture_offset = TextureOffset::new(x as u32, y as u32);

            let transform = Transform::from_xyz(
                CardAssetData::SIZE_X * x as f32 - CardAssetData::SIZE_X * 6.0
                    + CardAssetData::SIZE_X / 2.0,
                0.0,
                CardAssetData::SIZE_Z * y as f32 - CardAssetData::SIZE_Z * 2.0
                    + CardAssetData::SIZE_Z / 2.0,
            );

            let card = card_asset_data.create_entity_bundle(texture_offset, transform);
            commands.spawn_bundle(card).insert(*month).insert(*suit);
        }
    }

    commands.insert_resource(card_asset_data);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    Red,
    Blue,
}

const CARDS: [(Month, [Suit; 4]); 12] = [
    (
        Month::January,
        [
            Suit::Kasu,
            Suit::Kasu,
            Suit::Tanzaku(Tanzaku::Poetry),
            Suit::Hikari(Hikari::Crane),
        ],
    ),
    (
        Month::February,
        [
            Suit::Kasu,
            Suit::Tanzaku(Tanzaku::Poetry),
            Suit::Tane(Tane::BushWarbler),
            Suit::Kasu,
        ],
    ),
    (
        Month::March,
        [
            Suit::Hikari(Hikari::Curtain),
            Suit::Kasu,
            Suit::Kasu,
            Suit::Tanzaku(Tanzaku::Poetry),
        ],
    ),
    (
        Month::April,
        [
            Suit::Kasu,
            Suit::Tanzaku(Tanzaku::Red),
            Suit::Kasu,
            Suit::Tane(Tane::Cuckoo),
        ],
    ),
    (
        Month::May,
        [
            Suit::Tane(Tane::Bridge),
            Suit::Kasu,
            Suit::Kasu,
            Suit::Tanzaku(Tanzaku::Red),
        ],
    ),
    (
        Month::June,
        [
            Suit::Kasu,
            Suit::Tanzaku(Tanzaku::Blue),
            Suit::Kasu,
            Suit::Tane(Tane::Butterfly),
        ],
    ),
    (
        Month::July,
        [
            Suit::Tane(Tane::Boar),
            Suit::Kasu,
            Suit::Tanzaku(Tanzaku::Red),
            Suit::Kasu,
        ],
    ),
    (
        Month::August,
        [
            Suit::Kasu,
            Suit::Tane(Tane::Goose),
            Suit::Kasu,
            Suit::Hikari(Hikari::Moon),
        ],
    ),
    (
        Month::September,
        [
            Suit::Kasu,
            Suit::Tane(Tane::Sake),
            Suit::Kasu,
            Suit::Tanzaku(Tanzaku::Blue),
        ],
    ),
    (
        Month::October,
        [
            Suit::Tanzaku(Tanzaku::Blue),
            Suit::Kasu,
            Suit::Kasu,
            Suit::Tane(Tane::Deer),
        ],
    ),
    (
        Month::November,
        [
            Suit::Hikari(Hikari::Michikaze),
            Suit::Tane(Tane::Swallow),
            Suit::Tanzaku(Tanzaku::Red),
            Suit::Kasu,
        ],
    ),
    (
        Month::December,
        [
            Suit::Kasu,
            Suit::Kasu,
            Suit::Kasu,
            Suit::Hikari(Hikari::Phoenix),
        ],
    ),
];
