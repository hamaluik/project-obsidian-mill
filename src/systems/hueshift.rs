
use specs::{Read, WriteStorage, System};

pub struct HueShift;

impl<'a> System<'a> for HueShift {
    type SystemData = (Read<'a, crate::DeltaTime>, WriteStorage<'a, crate::components::Colour>);

    fn run(&mut self, data: Self::SystemData) {
        let (dt, mut colour) = data;
        let dt = dt.0;

        use specs::Join;
        for colour in (&mut colour).join() {
            colour.hue += dt * 90.0;
            if colour.hue >= 360.0 {
                colour.hue -= 360.0;
            }
        }
    }
}