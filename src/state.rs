use crate::{animation::AnimationId, prefabs};
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    assets::ProgressCounter,
    core::transform::Transform,
    ecs::{Entities, Join, ReadStorage, WriteStorage},
    prelude::{Builder, World, WorldExt},
    renderer::{camera::Camera, sprite::SpriteRender},
    window::ScreenDimensions,
    GameData, SimpleState, SimpleTrans, StateData, Trans,
};

#[derive(Default)]
pub struct Game {
    pub progress_counter: Option<ProgressCounter>,
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;
        // Crates new progress counter
        self.progress_counter = Some(Default::default());
        prefabs::load_player(&mut world, self.progress_counter.as_mut().unwrap());
        // Creates a new camera
        initialise_camera(&mut world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // Checks if we are still loading data

        if let Some(ref progress_counter) = self.progress_counter {
            // Checks progress
            if progress_counter.is_complete() {
                println!("LOADED");
                let StateData { world, .. } = data;
                // Execute a pass similar to a system
                world.exec(
                    |(entities, animation_sets, mut control_sets): (
                        Entities,
                        ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
                        WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
                    )| {
                        // For each entity that has AnimationSet
                        for (entity, animation_set) in (&entities, &animation_sets).join() {
                            // Creates a new AnimationControlSet for the entity
                            let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                            // Adds the `Idle` animation to AnimationControlSet and loops infinitely
                            control_set.add_animation(
                                AnimationId::Idle,
                                &animation_set.get(&AnimationId::Idle).unwrap(),
                                EndControl::Loop(None),
                                1.0,
                                AnimationCommand::Start,
                            );
                        }
                    },
                );
                // All data loaded
                self.progress_counter = None;
            }
        }
        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_z(1.0);

    world
        .create_entity()
        .with(camera_transform)
        .with(Camera::standard_2d(width, height))
        .build();
}
