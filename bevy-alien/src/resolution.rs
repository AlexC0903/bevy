use bevy::prelude::*;

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_resolution);
    }
}

#[derive(Resource)]
pub struct Resolution {
    // Pixel Dimensions of the Screen
    pub screen_dimension: Vec2,
    // the ratio of pixels in the sprite to pixels on the screen
    pub pixel_ratio: f32,
}

fn setup_resolution(mut commands: Commands, window_query: Query<&Window>) {
    // query for window info
    let window = window_query.single();

    commands.insert_resource(Resolution {
        screen_dimension: Vec2::new(window.width(), window.height()),
        pixel_ratio: 2.0,
    })
}
