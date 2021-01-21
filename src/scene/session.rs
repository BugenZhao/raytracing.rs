use crate::world::World;

use super::Scene;

pub struct RenderSession<'a, W: World> {
    pub width: u32,
    pub max_depth: usize,
    pub samples_per_pixel_axis: u32,
    pub scene: Scene<'a, W>,

    pub height: u32,
    pub samples_per_pixel: u32,
    pub sample_step: f64,
}

impl<'a, W: World> RenderSession<'a, W> {
    pub fn new(
        width: u32,
        max_depth: usize,
        mut samples_per_pixel_axis: u32,
        scene: Scene<'a, W>,
    ) -> Self {
        let height = (width as f64 / scene.camera.aspect_ratio) as u32;

        if cfg!(debug_assertions) {
            samples_per_pixel_axis = samples_per_pixel_axis.min(2);
        }
        let samples_per_pixel = samples_per_pixel_axis.pow(2);
        let sample_step = 1. / samples_per_pixel_axis as f64;

        Self {
            width,
            max_depth,
            samples_per_pixel_axis,
            scene,
            height,
            samples_per_pixel,
            sample_step,
        }
    }

    pub fn new_default(scene: Scene<'a, W>) -> Self {
        Self::new(800, 50, 16, scene)
    }
}
