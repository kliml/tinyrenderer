use std::ops::{Add, BitXor, Div, Mul, Sub};

use tobj::Mesh;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy + MyNum,
{
    pub fn norm(&self) -> f32 {
        let norm: f32 = (self.x * self.x + self.y * self.y + self.z * self.z).to_f32();
        norm.sqrt()
    }

    pub fn normalize(&self) -> Vec3<T> {
        unimplemented!();
    }

    pub fn scalar_mul(&self, other: &Vec3<T>) -> T {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(v1: &Vec3<T>, v2: &Vec3<T>) -> Vec3<T> {
        Vec3::<T> {
            x:v1.y*v2.z - v1.z*v2.y,
            y:v1.z*v2.x - v1.x*v2.z,
            z:v1.x*v2.y - v1.y*v2.x,
        }
    }
}

pub trait MyNum {
    fn to_f32(self) -> f32;
}

impl MyNum for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl MyNum for f32 {
    fn to_f32(self) -> f32 {
        self
    }
}

impl<T> Mul for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, other: Vec3<T>) -> Vec3<T> {
        Vec3::<T> {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, other: T) -> Vec3<T> {
        Vec3::<T> {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T> + Copy + MyNum,
{
    type Output = Self;
    fn div(self, other: T) -> Vec3<T> {
        Vec3::<T> {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3::<T> {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3::<T> {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> BitXor for Vec3<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;
    fn bitxor(self, other: Vec3<T>) -> Vec3<T> {
        Vec3::<T> {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

pub type Vec3f = Vec3<f32>;
pub type Vec3i = Vec3<i32>;

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
