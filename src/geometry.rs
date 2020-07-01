use nalgebra::Vector3;
use tobj::Mesh;

pub struct Triangle {
    pub v1: Vector3<f32>,
    pub v2: Vector3<f32>,
    pub v3: Vector3<f32>,
}

impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        Triangle {
            v1: self.v1,
            v2: self.v2,
            v3: self.v3,
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
                // Repeat this triangle for all faces in polygon
                v1: Vector3::new(0.0, 0.0, 0.0),
                v2: Vector3::new(0.0, 0.0, 0.0),
                v3: Vector3::new(0.0, 0.0, 0.0)
            };
            self.indices.len() / 3
        ];
        for (x, tri) in triangles.iter_mut().enumerate() {
            tri.v1.x = self.positions[(self.indices[x * 3] * 3) as usize];
            tri.v1.y = self.positions[(self.indices[x * 3] * 3 + 1) as usize];
            tri.v1.z = self.positions[(self.indices[x * 3] * 3 + 2) as usize];
            tri.v2.x = self.positions[(self.indices[x * 3 + 1] * 3) as usize];
            tri.v2.y = self.positions[(self.indices[x * 3 + 1] * 3 + 1) as usize];
            tri.v2.z = self.positions[(self.indices[x * 3 + 1] * 3 + 2) as usize];
            tri.v3.x = self.positions[(self.indices[x * 3 + 2] * 3) as usize];
            tri.v3.y = self.positions[(self.indices[x * 3 + 2] * 3 + 1) as usize];
            tri.v3.z = self.positions[(self.indices[x * 3 + 2] * 3 + 2) as usize];
        }
        SimpleMesh {
            triangles,
        }
    }
}