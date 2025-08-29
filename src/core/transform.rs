use crate::core::data::Vector3;

pub struct Transform {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
}

impl Transform {
    pub fn position(&self) -> &Vector3<f32> {
        &self.position
    }

    pub fn rotation(&self) -> &Vector3<f32> {
        &self.rotation
    }

    pub fn scale(&self) -> &Vector3<f32> {
        &self.scale
    }

    pub fn position_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.position
    }

    pub fn rotation_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.rotation
    }

    pub fn scale_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.scale
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: Vector3<f32>) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
    }
}
