use amethyst::{
    prelude::*,
};

#[derive(Default)]
pub struct Conspiracy;

impl SimpleState for Conspiracy {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}
