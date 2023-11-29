use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_player);
    }
}

fn add_player(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(200.0, 20.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 600.0, 0.0)),
        ..default()
    });
}
