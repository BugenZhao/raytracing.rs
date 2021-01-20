use super::Scene;

pub struct RenderSession<'a> {
    pub width: u32,
    pub max_depth: usize,
    pub samples_per_pixel_axis: u32,
    pub scene: Scene<'a>,

    pub height: u32,
    pub samples_per_pixel: u32,
    pub sample_step: f64,
}

impl<'a> RenderSession<'a> {
    pub fn new(
        width: u32,
        max_depth: usize,
        samples_per_pixel_axis: u32,
        scene: Scene<'a>,
    ) -> Self {
        let height = (width as f64 / scene.camera.aspect_ratio) as u32;
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

    pub fn new_default(scene: Scene<'a>) -> Self {
        Self::new(800, 50, if cfg!(debug_assertions) { 2 } else { 16 }, scene)
    }
}
