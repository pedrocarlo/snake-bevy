use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::SpriteSheet;

use super::terrain::{SpriteParts, TILE_H, TILE_W};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<SnakeAction>::default())
            .add_systems(Startup, spawn_snake)
            .add_systems(Update, move_snake);
    }
}

#[derive(Component, Default)]
#[require(Sprite, Collider)]
pub struct Snake;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Debug, Reflect)]
pub enum SnakeAction {
    #[actionlike(DualAxis)]
    Move,
}

impl SnakeAction {
    /// Define the default bindings to the input
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default kbm input bindings
        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());

        input_map
    }
}

fn spawn_snake(
    mut commands: Commands,
    texture: Res<SpriteSheet>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2 {
            x: TILE_W as u32,
            y: TILE_H as u32,
        },
        6,
        1,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Spawn the player with the default input_map
    commands
        .spawn(InputManagerBundle::with_map(
            SnakeAction::default_input_map(),
        ))
        .insert(Snake)
        .insert(Sprite::from_atlas_image(
            texture.0.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: SpriteParts::SnakeHead.into(),
            },
        ))
        .insert(Transform::from_xyz(50.0, 50.0, 2.0));
}

fn move_snake(query: Query<&ActionState<SnakeAction>, With<Snake>>) {
    let action_state = query.single();
    let axis_pair = action_state.clamped_axis_pair(&SnakeAction::Move);
    if axis_pair != Vec2::ZERO {
        // Virtual direction pads are one of the types which return a DualAxis. The values will be
        // represented as `-1.0`, `0.0`, or `1.0` depending on the combination of buttons pressed.
        println!("Move:");
        println!("   distance: {}", axis_pair.length());
        println!("          x: {}", axis_pair.x);
        println!("          y: {}", axis_pair.y);
    }
}
