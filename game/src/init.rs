use crate::{
    config::{MAX_ENERGY, MAX_HEALTH},
    Game,
};
use common::{
    hecs,
    rand::{self, Rng},
    Camera,
};
use components::{
    Beacon, Collider, Dave, GLTFAsset, Health, Info, Inventory, PlaceOfWork, Resource, Storage,
    Transform, Velocity, Viking,
};

pub fn init_game() -> Game {
    let mut camera = Camera::default();
    let mut world = hecs::World::new();

    // dave
    let dave = world.spawn((
        GLTFAsset::new("droid.glb"),
        Dave::new(MAX_ENERGY),
        Health::new(MAX_HEALTH),
        Transform::from_position([0., 2., 0.].into()),
        Velocity::default(),
        Info::new("DAVE"),
    ));

    // terrain
    world.spawn((
        Transform::default(),
        Info::new("Ground"),
        GLTFAsset::new("environment.glb"),
    ));

    let mut rng = rand::thread_rng();

    const STARTING_VIKINGS: usize = 10;
    for i in 0..STARTING_VIKINGS {
        let x = (rng.gen::<f32>() * 50.) - 25.;
        let z = (rng.gen::<f32>() * 50.) - 25.;
        let intelligence = rng.gen_range(1..5);
        let strength = rng.gen_range(1..5);
        let stamina = rng.gen_range(1..5);
        world.spawn((
            Collider::default(),
            GLTFAsset::new("viking_1.glb"),
            Viking::new(intelligence, strength, stamina),
            Transform::from_position([x, 1., z].into()),
            Velocity::default(),
            Info::new(format!("Viking {i}")),
        ));
    }

    // beacon
    world.spawn((
        Collider::default(),
        GLTFAsset::new("ship.glb"),
        Beacon::default(),
        Transform::default(),
        Info::new("Ship"),
        Inventory::new([]),
        Storage,
    ));

    // mine
    world.spawn((
        Collider::default(),
        GLTFAsset::new("mine.glb"),
        Transform::from_position([30.0, 0.0, 0.0].into()),
        Velocity::default(),
        PlaceOfWork::mine(),
        Inventory::new([(Resource::RawIron, 5000)]),
        Info::new("Mine"),
    ));

    // forge
    world.spawn((
        Collider::default(),
        GLTFAsset::new("forge.glb"),
        Transform::from_position([-30., 0.0, 0.0].into()),
        Velocity::default(),
        PlaceOfWork::forge(),
        Inventory::new([]),
        Info::new("Forge"),
    ));

    // factory
    world.spawn((
        Collider::default(),
        GLTFAsset::new("factory.glb"),
        Transform::from_position([20., 0.0, 30.0].into()),
        Velocity::default(),
        PlaceOfWork::factory(),
        Inventory::new([]),
        Info::new("Factory"),
    ));

    camera.position.y = 3.;
    camera.position.z = 12.;
    camera.distance = 50.;
    camera.desired_distance = camera.distance;
    camera.start_distance = camera.distance;
    Game {
        camera,
        dave,
        world,
        ..Default::default()
    }
}
