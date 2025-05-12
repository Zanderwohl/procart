use bevy::color::Srgba;
use bevy::prelude::Alpha;

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
