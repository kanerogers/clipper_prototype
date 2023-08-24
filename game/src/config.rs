use common::glam::Vec3;

pub const FREE_COLOUR: Vec3 = Vec3::new(1., 0., 0.);
pub const FOLLOWING_COLOUR: Vec3 = Vec3::new(0., 0., 1.);
pub const AWAITING_ASSIGNMENT_COLOUR: Vec3 = Vec3::new(1., 0.85, 0.);
pub const WORKING_COLOUR: Vec3 = Vec3::new(0., 0.85, 0.);
pub const BRAINWASH_DISTANCE_THRESHOLD: f32 = 5.0;
pub const BRAINWASH_TIME: f32 = 1.;