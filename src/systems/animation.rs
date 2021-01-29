use amethyst::{
    ecs::SystemBuilder,
    animation::{
        AnimationControlSet, AnimationCommand, AnimationSet, EndControl,
    },
    input::{InputHandler},
    prelude::*,
    renderer::{SpriteRender},
};

use crate::conspiracy::{Bilu, BiluAnimations};

pub struct AnimationSystem;

impl System<'_> for AnimationSystem {
    fn build(self) -> Box<dyn ParallelRunnable> {
        Box::new(
            SystemBuilder::new("AnimationSystem")
                .with_query(<(&Bilu, &mut AnimationControlSet<BiluAnimations,SpriteRender>,&mut AnimationSet<BiluAnimations,SpriteRender>)>::query())
                .read_resource::<InputHandler>()
                .build(move |_commands, world, _input, query_bilus| {
                    for (_bilu,control_set, anim_set) in query_bilus.iter_mut(world) {
                            if control_set.is_empty() {
                                control_set.add_animation(
                                    BiluAnimations::WalkRight,
                                    &anim_set.get(&BiluAnimations::WalkRight).unwrap(),
                                    EndControl::Loop(None),
                                    1.0,
                                    AnimationCommand::Start,
                                );
                            }
                        }
                }),
        )
    }
}
