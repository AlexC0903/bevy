use bevy::prelude::*;

pub struct AlienPlugin;

use crate::resolution;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_aliens);
        app.add_systems(Update, update_aliens);
    }
}

#[derive(Component)]
pub struct Alien;

// Controls alien behaviour
#[derive(Resource)]
pub struct AlienManager {
    pub direction: f32,
}

// width and height represent the number of aliens we want to spawn in each block
const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 24.0;
const SPEED: f32 = 100.0;

fn setup_aliens(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
) {
    // default path for asset server is /assets
    let alien_texture = asset_server.load("alien.png");

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let position = Vec3::new(x as f32 * SPACING, y as f32 * SPACING, 0.0)
                - (Vec3::X * WIDTH as f32 * SPACING * 0.5) // Center the aliens on the x axis
            -(Vec3::Y * HEIGHT as f32 * SPACING * 1.0) // Displace aliens below Y axis for next line
            +(Vec3::Y * resolution.screen_dimension.y * 0.5); // Displace aliens to the top of the screen
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(position)
                        .with_scale(Vec3::splat(resolution.pixel_ratio)),
                    texture: alien_texture.clone(),
                    ..default()
                },
                Alien {},
            ));
        }
    }
}

fn update_aliens(
    mut alien_query: Query<(&Alien, &mut Transform)>,
    mut alien_manager: ResMut<AlienManager>,
    time: Res<Time>,
) {
    for (alien, mut transform) in alien_query.iter_mut() {
        transform.translation.x += time.delta_seconds() * alien_manager.direction * SPEED;
    }
}
