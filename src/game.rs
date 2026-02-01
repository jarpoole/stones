use avian3d::prelude::*;
use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
pub struct Gameboard;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Transform::default(),
        GlobalTransform::default(),
        DespawnOnExit(AppState::Playing),
        // need to explicitly set visibility because the board collider must be explicitly hidden
        // https://bevy.org/learn/errors/b0004/
        Visibility::Visible,
        Name::new("Root"),
        children![
            (
                Gameboard,
                SceneRoot(
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/board.glb"))
                ),
                Name::new("Board"),
                Pickable {
                    should_block_lower: true,
                    ..default()
                },
                Observer::new(on_board_click_handler),
                children![(
                    Name::new("Collider"),
                    RigidBody::Static,
                    SceneRoot(
                        asset_server
                            .load(GltfAssetLabel::Scene(0).from_asset("models/board_collider.glb")),
                    ),
                    Restitution::new(0.0),
                    ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
                    MeshMaterial3d(materials.add(Color::WHITE)),
                    Transform::from_xyz(0.0, 0.025, 0.0),
                    Visibility::Hidden,
                )]
            ),
            // Light
            (
                PointLight {
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_xyz(4.0, 8.0, 4.0),
            )
        ],
    ));
}

fn on_board_click_handler(mut trigger: On<Pointer<Click>>, query: Query<&Name>) {
    if let Ok(name) = query.get(trigger.observer()) {
        info!("'{}' was clicked", name);
    }
    trigger.propagate(false);
}

#[derive(Debug)]
pub enum PieceVariant {
    P1,
    P2,
    NEUTRAL,
}

impl std::fmt::Display for PieceVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct SpawnPieceOptions<'a, 'w, 's> {
    pub commands: &'a mut Commands<'w, 's>,
    pub asset_server: &'a Res<'w, AssetServer>,
    pub variant: PieceVariant,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn spawn_piece(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    variant: PieceVariant,
    x: f32,
    y: f32,
    z: f32,
    number: u8,
) {
    let model = match variant {
        PieceVariant::NEUTRAL => "neutral_piece.glb",
        PieceVariant::P1 => "player1_piece.glb",
        PieceVariant::P2 => "player2_piece.glb",
    };
    commands.spawn((
        Name::new(format!("{} Piece {}", variant, number)),
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(format!("models/{}", model))),
        ),
        RigidBody::Dynamic,
        ColliderConstructorHierarchy::new(
            // dramatically improves performance
            ColliderConstructor::ConvexDecompositionFromMesh,
        ),
        Restitution::new(0.0),
        Transform::from_xyz(x, y, z).with_scale((1.3, 1.3, 1.3).into()),
    ));
}
