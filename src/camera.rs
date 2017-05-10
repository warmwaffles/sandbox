extern crate nalgebra;

use self::nalgebra::Vector3;
use self::nalgebra::Matrix4;
use self::nalgebra::Orthographic3;
use self::nalgebra::Point3;
use self::nalgebra::Isometry3;

// SEE: https://github.com/MichaelShaw/rust-game-24h/blob/master/src/camera.rs
// for more examples and what not for camera positioning.

pub struct OrthoCamera {
    position: Point3<f32>,
    target:   Point3<f32>,
    up:       Vector3<f32>,
    width:    f32,
    height:   f32,
    zoom:     f32,
    near:     f32,
    far:      f32,
    /// Pixels Per Unit
    ppu:      f32,
}

impl OrthoCamera {
    pub fn new(width: f32, height: f32) -> Self {
        OrthoCamera {
            position: Point3::new(0.0, 0.0, 0.0),
            target:   Point3::new(0.0, 0.0, -1.0),
            up:       Vector3::new(0.0, 1.0, 0.0),
            width:    width,
            height:   height,
            zoom:     1.0,
            near:     -100.0,
            far:      100.0,
            ppu:      256.0
        }
    }

    pub fn set_dimensions(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    pub fn translate(&mut self, dx: f32, dy: f32)
    {
        self.position.x += dx;
        self.position.y += dy;
    }

    pub fn projection(&self) -> [[f32; 4]; 4] {
        // This needs to take into account that the width is fixed but the
        // height is elastic so to speak. Basically when the window resizes, I
        // want to keep the aspect the same. We'll figure that out later.
        let effective_width = self.width / (self.zoom * self.ppu);
        let effective_height = self.height / (self.zoom * self.ppu);

        let half_width  = effective_width / 2.0;
        let half_height = effective_height / 2.0;

        Orthographic3::new(
            -half_width,
            half_width,
            -half_height,
            half_height,
            self.near,
            self.far
        ).unwrap()
        .into()
    }

    pub fn view(&self) -> [[f32; 4]; 4] {
        Isometry3::look_at_rh(
            &self.position,
            &self.target,
            &self.up
        ).to_homogeneous()
        .into()
    }
}
