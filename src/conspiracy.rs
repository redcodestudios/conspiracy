use amethyst::{
    prelude::*,
    renderer::{Camera, SpriteRender, SpriteSheet, Texture},
    assets::{DefaultLoader, Handle, Loader, ProcessingQueue},
    core::transform::Transform,
};

#[derive(Default)]
pub struct Conspiracy;


pub const ARENA_HEIGHT: f32 = 640f32;
pub const ARENA_WIDTH: f32 = 960f32;

impl SimpleState for Conspiracy {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(data.resources);

        initialise_player(world, sprite_sheet_handle);
        initialise_camera(world);
    }

    fn update(&mut self, _: &mut StateData<'_, GameData>) -> SimpleTrans {
        Trans::None
    }
}


pub struct Bilu {
    pub width: f32,
    pub height: f32,
}

impl Bilu {
    fn new() -> Bilu {
        Bilu {
            width: 40f32,
            height: 43f32,
        }
    }
}


fn load_sprite_sheet(resources: &mut Resources) -> Handle<SpriteSheet> {
    let texture: Handle<Texture> = {
        let loader = resources.get::<DefaultLoader>().unwrap();
        loader.load("sprites/bilu_sheet.png")
    };
    let loader = resources.get::<DefaultLoader>().unwrap();
    let sprites = loader.load("textures/bilu_sheet.ron");

    loader.load_from_data(
        SpriteSheet { texture, sprites },
        (),
        &resources.get::<ProcessingQueue<SpriteSheet>>().unwrap(),
    )
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world.push((Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT), transform));
}

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_player(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut bilu_transform = Transform::default();

    // Correctly position the paddles.
    let y = ARENA_HEIGHT / 2.0;
    bilu_transform.set_translation_xyz(40f32, y, 0.0);

    // Assign the sprites for the paddles
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 5); // paddle is the first sprite in the sprite_sheet

    world.push((
        sprite_render.clone(),
        Bilu::new(),
        bilu_transform,
    ));
}