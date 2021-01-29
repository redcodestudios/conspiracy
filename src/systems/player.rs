use amethyst::{
    core::transform::Transform,
    ecs::SystemBuilder,
    input::{get_input_axis_simple, InputHandler},
    prelude::*,
};

use crate::conspiracy::{Bilu, ARENA_HEIGHT, ARENA_WIDTH};

pub struct PlayerSystem;

impl System<'_> for PlayerSystem {
    fn build(self) -> Box<dyn ParallelRunnable> {
        Box::new(
            SystemBuilder::new("PlayerSystem")
                .with_query(<(&Bilu, &mut Transform)>::query())
                .read_resource::<InputHandler>()
                .build(move |_commands, world, input, query_bilus| {
                    for (bilu, transform) in query_bilus.iter_mut(world) {
                        let movement_x =  input.axis_value("left_right").unwrap();
                        let movement_y =  get_input_axis_simple(&Some("up_down".into()), input);
                        
                        let scaled_amount_x = 1.0 * movement_x;
                        let scaled_amount_y = 1.0 * movement_y;
                        let bilu_y = transform.translation().y;
                        let bilu_x = transform.translation().x;

                        transform.set_translation_y(
                            (bilu_y + scaled_amount_y)
                                .min(ARENA_HEIGHT - bilu.height * 0.5)
                                .max(bilu.height * 0.5),
                        );
                        transform.set_translation_x(
                            (bilu_x + scaled_amount_x)
                                .min(ARENA_WIDTH - bilu.width * 0.5)
                                .max(bilu.width * 0.5),
                        );
                    }
                }),
        )
    }
}
