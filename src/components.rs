use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub u: f32,
    pub v: f32,
    pub w_uv: f32,
    pub h_uv: f32,
}
