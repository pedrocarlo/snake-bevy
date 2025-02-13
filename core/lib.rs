mod plugins;

use avian2d::prelude::*;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_ecs_tilemap::TilemapPlugin;

const SPRITE_SHEET_PATH: &str = "snake-sprite-sheet.png";

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

#[derive(Resource, Deref, DerefMut)]
pub struct SpriteSheet(Handle<Image>);

impl FromWorld for SpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture = world.load_asset(SPRITE_SHEET_PATH);

        SpriteSheet(texture.clone())
    }
}

struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(plugins::terrain::TerrainPlugin)
            .add(plugins::snake::SnakePlugin)
    }
}

pub fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

    app.init_resource::<SpriteSheet>();

    app.add_systems(PreStartup, (setup, spawn_camera));
    app.add_plugins(CorePlugins);
    app.add_plugins(TilemapPlugin);
    app.add_plugins(PhysicsPlugins::default());

    app
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load(SPRITE_SHEET_PATH);

    commands.insert_resource(SpriteSheet(texture.clone()));
}
