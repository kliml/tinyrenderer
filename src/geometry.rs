use std::ops::{Add, AddAssign, Mul, Neg, Sub};
use tobj::Mesh;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Mul for Vec3f {
    type Output = Self;
    fn mul(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3f {
    type Output = Self;
    fn mul(self, other: f32) -> Vec3f {
        Vec3f {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Add for Vec3f {
    type Output = Self;
    fn add(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3f {
    type Output = Self;
    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

pub struct Triangle {
    pub vertices: [Vec3f; 3],
}

impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        Triangle {
            vertices: self.vertices,
        }
    }
}

pub struct SimpleMesh {
    pub triangles: Vec<Triangle>,
}

pub trait ToSimpleMesh {
    fn to_simple_mesh(&self) -> SimpleMesh;
}

impl ToSimpleMesh for Mesh {
    fn to_simple_mesh(&self) -> SimpleMesh {
        let mut triangles = vec![
            Triangle {
                vertices: [Vec3f {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                }; 3],
            };
            self.indices.len() / 3
        ];
        for (x, tri) in triangles.iter_mut().enumerate() {
            tri.vertices[0].x = self.positions[(self.indices[x * 3] * 3) as usize];
            tri.vertices[0].y = self.positions[(self.indices[x * 3] * 3 + 1) as usize];
            tri.vertices[0].z = self.positions[(self.indices[x * 3] * 3 + 2) as usize];
            tri.vertices[1].x = self.positions[(self.indices[x * 3 + 1] * 3) as usize];
            tri.vertices[1].y = self.positions[(self.indices[x * 3 + 1] * 3 + 1) as usize];
            tri.vertices[1].z = self.positions[(self.indices[x * 3 + 1] * 3 + 2) as usize];
            tri.vertices[2].x = self.positions[(self.indices[x * 3 + 2] * 3) as usize];
            tri.vertices[2].y = self.positions[(self.indices[x * 3 + 2] * 3 + 1) as usize];
            tri.vertices[2].z = self.positions[(self.indices[x * 3 + 2] * 3 + 2) as usize];
        }
        SimpleMesh { triangles }
    }
}
