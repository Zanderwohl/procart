use bevy::color::Srgba;
use bevy::prelude::{Alpha, Resource};

pub trait Modifier {
    fn pastel(&self) -> Srgba;
    fn pastel_very(&self) -> Srgba;
}


impl Modifier for Srgba {
    fn pastel(&self) -> Srgba {
        let mut out = *self + Srgba::WHITE * 0.25;
        out.set_alpha(1.0);
        out
    }

    fn pastel_very(&self) -> Srgba {
        let mut out = *self + Srgba::WHITE * 0.4;
        out.set_alpha(1.0);
        out
    }
}

#[derive(Resource)]
pub struct CachedRandom {
    size: usize,
    floats: Vec<f32>,
}

impl Default for CachedRandom {
    fn default() -> Self {
        Self::new(1031) // A prime so there won't be loops for a long time.
    }
}

impl CachedRandom {
    pub fn new(size: usize) -> Self {
        let mut floats = Vec::with_capacity(size);

        for _ in 0..size {
            floats.push(rand::random::<f32>());
        }

        Self {
            size,
            floats,
        }
    }

    pub fn f32(&self, idx: usize) -> f32 {
        let idx = idx % self.size;
        self.floats[idx]
    }
}
