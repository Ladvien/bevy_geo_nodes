use bevy::{math::Vec3, render::mesh::Mesh};

pub struct VoxelGrid {
    pub data: Vec<f32>,
    pub resolution: Vec<usize>,
}

impl VoxelGrid {
    pub fn new(&self, data: Vec<f32>, shape: Vec<usize>) -> Self {
        let mut voxel_grid: Vec<f32> = Vec::with_capacity(shape[0] * shape[1] * shape[2]);

        for x in 0..shape[0] {
            for y in 0..shape[1] {
                for z in 0..shape[2] {
                    voxel_grid.push(self.scaler_field(
                        x as f32 / shape[0] as f32,
                        y as f32 / shape[1] as f32,
                        z as f32 / shape[2] as f32,
                        // TODO: replace with noise
                        0.8,
                    ));
                }
            }
        }

        Self {
            data,
            resolution: shape,
        }
    }

    pub fn read(&self, x: usize, y: usize, z: usize) -> f32 {
        self.data[x + y * self.resolution[0] + z * self.resolution[0] * self.resolution[1]]
    }

    pub fn push(&mut self, value: f32) {
        self.data.push(value);
    }

    fn scaler_field(&self, x: f32, y: f32, z: f32, noise: f32) -> f32 {
        x + y * self.resolution[0] as f32
            + z * self.resolution[0] as f32 * self.resolution[1] as f32 * noise
    }
}
pub struct MarchingCubes {
    pub voxel_grid: VoxelGrid,
    pub isolevel: f32,
}

impl MarchingCubes {
    pub fn new(mesh: &Mesh, voxel_grid: VoxelGrid, isolevel: f32) -> Self {
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .unwrap();

        let voxel_grid = 

        Self {
            voxel_grid,
            isolevel,
        }
    }
}

impl MarchingCubes {
    pub fn march(&self, cube_index: usize) {}
}
