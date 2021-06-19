use amethyst::{
    animation::AnimationBundle,
    assets::PrefabLoaderSystemDesc,
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::InputBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::SpriteRender,
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    Application, GameDataBuilder,
};

mod animation;
mod audio;
mod components;
mod input;
mod prefabs;
mod resources;
mod state;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");
    let bindings = app_root.join("config").join("bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<prefabs::BackgroundPrefab>::default(),
            "scene_loader",
            &[],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<prefabs::PlayerPrefab>::default(),
            "player_loader",
            &[],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<prefabs::ThrownPrefab>::default(),
            "thrown_loader",
            &[],
        )
        .with_bundle(AudioBundle::default())?
        .with_bundle(
            AnimationBundle::<animation::AnimationId, SpriteRender>::new(
                "sprite_animation_control",
                "sprite_sampler_interpolation",
            ),
        )?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with_bundle(
            InputBundle::<input::GameBindingTypes>::new().with_bindings_from_file(bindings)?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(systems::TimingSystem::default(), "timing_system", &[])
        .with(
            systems::UserInputSystem::default(),
            "user_input_system",
            &[],
        )
        .with(
            systems::PhysicsSystem,
            "physics_system",
            &["user_input_system", "timing_system"],
        );

    let mut game = Application::new(assets_dir, state::Loading::default(), game_data)?;
    game.run();

    Ok(())
}
