mod config;
mod jump;
mod systems;

use amethyst::{
    core::{
        frame_limiter::FrameRateLimitStrategy,
        transform::TransformBundle,
    },
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use std::time::Duration;

use crate::config::JumpConfig;
use crate::jump::Jump;

fn main() -> amethyst::Result<()> {

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let config_path = app_root.join("config").join("config.ron");
    let config = JumpConfig::load(&config_path)?;

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    // TODO Why isn't this RGB 59,104,135
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.231, 0.407, 0.529, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::MovementSystem, "movement_system", &["input_system"])
        .with(systems::CollisionSystem, "collision_system", &["input_system"])
        .with(systems::AnimationSystem, "animation_system", &["movement_system"])
        .with(systems::FacingSystem, "facing_system", &["animation_system"])
        .with(systems::ProgressSystem, "progress_system", &["collision_system"]);

    let assets_dir = app_root.join("assets");

    let mut game = Application::build(
            assets_dir,
            Jump {
                config: config
            },
        )?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            60,
        )
        .build(game_data)?;

    game.run();

    Ok(())
}
