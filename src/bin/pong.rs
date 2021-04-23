use amethyst::{
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    Application, GameDataBuilder,
};

/// Main function.
fn main() -> amethyst::Result<()> {
    println!("Hello world!");

    // Logger.
    amethyst::start_logger(Default::default());

    // Configuration.
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // Application.
    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default()),
    )?;
    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();

    Ok(())
}

use amethyst::SimpleState;

/// Main game structure.
pub struct Pong;

/// Time state.
impl SimpleState for Pong {}
