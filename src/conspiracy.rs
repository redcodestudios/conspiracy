use amethyst::{
    prelude::*,
};

#[derive(Default)]
pub struct Conspiracy;

impl SimpleState for Conspiracy {
    fn on_start(&mut self, data: StateData<'_, GameData>) {

    }

    fn update(&mut self, _: &mut StateData<'_, GameData>) -> SimpleTrans {
        Trans::None
    }
}
