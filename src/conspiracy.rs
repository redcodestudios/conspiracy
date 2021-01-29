use amethyst::{
    animation::{
        Animation,AnimationSet,AnimationControlSet,
        InterpolationFunction, Sampler, SpriteRenderPrimitive,
    },
    prelude::*,
    renderer::{Camera, SpriteRender, sprite::Sprites, SpriteSheet, Texture},
    assets::{DefaultLoader, Handle, Loader, ProcessingQueue, ProgressCounter},
    core::transform::Transform,
};
use serde::{Deserialize, Serialize};


#[derive(Default)]
pub struct Conspiracy{
    pub progress_counter: Option<ProgressCounter>,
}


pub const ARENA_HEIGHT: f32 = 640f32;
pub const ARENA_WIDTH: f32 = 960f32;

impl SimpleState for Conspiracy {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let world = data.world;
        self.progress_counter = Some(Default::default());

        let sprite_sheet_handle = load_sprite_sheet(data.resources);
        let anim_handle = load_animation(self, data.resources);

        initialise_player(world, sprite_sheet_handle, anim_handle);
        initialise_camera(world);
    }

    fn update(&mut self, _: &mut StateData<'_, GameData>) -> SimpleTrans {
        Trans::None
    }
}


#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum BiluAnimations {
    WalkRight,
    WalkLeft,
    Action,
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

fn load_animation(state: &mut Conspiracy,resources:&mut Resources) -> AnimationSet<BiluAnimations,SpriteRender> {
    let loader = resources
                .get_mut::<DefaultLoader>()
                .expect("Missing loader");

    let _sprites: Handle<Sprites> = loader.load("textures/bilu_sheet.ron");
    let anims = loader.load_from_data(
        Sampler::<SpriteRenderPrimitive> {
            input: vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0],
            output: vec![
                SpriteRenderPrimitive::SpriteIndex(4),
                SpriteRenderPrimitive::SpriteIndex(3),
                SpriteRenderPrimitive::SpriteIndex(2),
                SpriteRenderPrimitive::SpriteIndex(1),
                SpriteRenderPrimitive::SpriteIndex(0),
            ],
            function: InterpolationFunction::Step,
        },
        state.progress_counter.as_mut().unwrap(),
        &resources.get().expect("queue for Sampler"),
    );


    let anim_handle = loader.load_from_data(
        Animation::<SpriteRender>::new_single(
            0,
            amethyst::animation::SpriteRenderChannel::SpriteIndex,
            anims,
        ),
        state.progress_counter.as_mut().unwrap(),
        &resources.get().expect("queue for Animation"),
    );

    let mut anim_set = AnimationSet::new();
    anim_set.insert(BiluAnimations::WalkRight, anim_handle);

    anim_set
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
fn initialise_player(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, anim_handle:AnimationSet<BiluAnimations,SpriteRender>) {
    let mut bilu_transform = Transform::default();

    
    let y = ARENA_HEIGHT / 2.0;
    bilu_transform.set_translation_xyz(40f32, y, 0.0);

    
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 5); // paddle is the first sprite in the sprite_sheet

    world.push((
        sprite_render.clone(),
        Bilu::new(),
        bilu_transform,
        anim_handle,
        AnimationControlSet::<BiluAnimations,SpriteRender>::default(),
    ));
}