pub mod time;

use common::{
    glam::{Affine3A, Quat, Vec3},
    Camera, Geometry, Mesh,
};
use std::f32::consts::TAU;
use time::Time;

const PLAYER_SPEED: f32 = 7.;
const CAMERA_ZOOM_SPEED: f32 = 10.;
const CAMERA_ROTATE_SPEED: f32 = 3.;

#[derive(Clone, Debug, Default)]
pub struct Game {
    pub meshes: Vec<Mesh>,
    pub time: Time,
    pub dave: Dave,
    pub input: Input,
    pub camera: Camera,
}

// objectively terrible input handling
#[derive(Clone, Debug)]
pub struct Input {
    pub movement: Vec3,
    pub camera_rotate: f32,
    pub camera_zoom: f32,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            movement: Vec3::ZERO,
            camera_rotate: 0.,
            camera_zoom: 0.,
        }
    }
}

impl Input {
    pub fn reset(&mut self) {
        *self = Default::default();
    }
}

impl Game {
    pub fn new() -> Self {
        let mut camera = Camera::default();
        let dave = Dave::default();
        camera.position.y = 3.;
        camera.position.z = 12.;
        camera.focus_point = dave.position;
        camera.distance = 10.;
        camera.desired_distance = camera.distance;
        camera.start_distance = camera.distance;
        Self {
            camera,
            dave,
            meshes: get_grid(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Dave {
    pub position: Vec3,
    pub velocity: Vec3,
}

impl Dave {
    pub fn update(&mut self, dt: f32, input: &Input, camera_transform: &Affine3A) {
        // Camera relative controls
        let mut forward = camera_transform.transform_vector3(Vec3::Z);
        forward.y = 0.;
        forward = forward.normalize();

        let mut right = camera_transform.transform_vector3(Vec3::X);
        right.y = 0.;
        right = right.normalize();

        let mut movement = forward * input.movement.z + right * input.movement.x;
        movement.y = input.movement.y;
        movement = movement.normalize();
        self.velocity = if !movement.is_nan() {
            movement
        } else {
            Vec3::ZERO
        };

        // Velocity, baby!
        let displacement = self.velocity * PLAYER_SPEED * dt;
        self.position += displacement;
        self.position.y = self.position.y.clamp(1., 5.);
    }
}

impl Default for Dave {
    fn default() -> Self {
        Self {
            position: [0., 2., 0.].into(),
            velocity: Default::default(),
        }
    }
}

fn get_grid() -> Vec<Mesh> {
    let plane_rotation = Quat::from_rotation_x(TAU / 4.0); // 90 degrees
    let mut meshes = Vec::new();

    let grid_size = 32;
    let square_size = 2.0;

    for row in 0..grid_size {
        for column in 0..grid_size {
            let x = (column as f32) * square_size - (grid_size as f32 * square_size / 2.0);
            let y = (row as f32) * square_size - (grid_size as f32 * square_size / 2.0);
            // let colour = if column == 0 || row == 0 {
            //     [0.5, 0.3, 0.1]
            // } else {
            //     [0., 0.8, 0.0]
            // };
            let colour = if (column + row) % 2 > 0 {
                [0.5, 0.3, 0.1] // brown
            } else {
                [0., 0.8, 0.0] // green
            };

            meshes.push(Mesh {
                geometry: Geometry::Plane,
                transform: Affine3A::from_rotation_translation(plane_rotation, [x, 0., y].into()),
                colour: Some(colour.into()),
                ..Default::default()
            })
        }
    }

    meshes
}

pub fn dave(game: &mut Game) {
    let sphere_rotation = Quat::from_rotation_y(TAU / 8.0);
    let dt = game.time.delta();

    game.dave.update(dt, &game.input, &game.camera.transform());
    *game.meshes.last_mut().unwrap() = Mesh {
        geometry: Geometry::Sphere,
        transform: Affine3A::from_rotation_translation(sphere_rotation, game.dave.position),
        colour: Some([0., 0., 0.9].into()),
        ..Default::default()
    };
}
pub fn update_camera(game: &mut Game) {
    let camera = &mut game.camera;
    let input = &game.input;
    let dt = game.time.delta();

    let focus_radius = 1.0;
    let focus_centering = 0.5;
    let distance_to_target = camera.target.distance(camera.focus_point);

    let mut t = 1.0;
    if distance_to_target > 0.01 {
        t = ((1. - focus_centering) as f32).powf(dt);
    }
    if distance_to_target > focus_radius {
        t = t.min(focus_radius / distance_to_target);
    }
    camera.focus_point = camera.target.lerp(camera.focus_point, t);

    camera.yaw += input.camera_rotate * CAMERA_ROTATE_SPEED * dt;

    set_camera_distance(input, camera, dt);

    camera.pitch = -45_f32.to_radians();
    let look_rotation = Quat::from_euler(common::glam::EulerRot::YXZ, camera.yaw, camera.pitch, 0.);
    let look_direction = look_rotation * Vec3::NEG_Z;
    let look_position = camera.focus_point - look_direction * camera.distance;

    camera.position = look_position;
}

fn set_camera_distance(input: &Input, camera: &mut Camera, dt: f32) {
    if input.camera_zoom.abs() > 0. {
        println!("camera zoom: {}", input.camera_zoom);
        camera.start_distance = camera.distance;
        camera.desired_distance += input.camera_zoom;
        camera.desired_distance = camera.desired_distance.clamp(5., 50.);
    }

    let current_delta = camera.desired_distance - camera.distance;

    let epsilon = 0.01;
    if current_delta.abs() > epsilon {
        camera.distance += current_delta * CAMERA_ZOOM_SPEED * dt;
    } else {
        camera.distance = camera.desired_distance;
    }
}

#[no_mangle]
pub fn tick(game: &mut Game) {
    while game.time.start_update() {
        game.camera.target = game.dave.position;
        update_camera(game);
        dave(game);
    }
}
