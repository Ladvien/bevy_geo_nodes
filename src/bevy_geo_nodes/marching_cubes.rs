use bevy::{math::Vec3, render::mesh::Mesh};

use super::TRIANGULATIONS;

// pub struct VoxelResolution {
//     pub x: usize,
//     pub y: usize,
//     pub z: usize,
// }

// impl Default for VoxelResolution {
//     fn default() -> Self {
//         Self {
//             x: 32,
//             y: 32,
//             z: 32,
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct VoxelGrid {
    pub data: Vec<f32>,
    pub resolution: usize,
}
fn scaler_field(x: f32, y: f32, z: f32, resolution: usize, noise: f32) -> f32 {
    x + y * resolution as f32 + z * resolution as f32 * resolution as f32 * noise
}

impl VoxelGrid {
    pub fn new(data: &[[f32; 3]], resolution: usize) -> Self {
        let mut voxel_grid: Vec<f32> = Vec::with_capacity(resolution * resolution * resolution);

        for x in 0..resolution {
            for y in 0..resolution {
                for z in 0..resolution {
                    voxel_grid.push(scaler_field(x as f32, y as f32, z as f32, resolution, 0.1));
                }
            }
        }

        Self {
            data: voxel_grid,
            resolution,
        }
    }

    pub fn from_mesh(mesh: &Mesh, resolution: usize) -> Self {
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .unwrap();

        // Voxelize the mesh

        let mut voxel_grid = VoxelGrid::new(positions, resolution);

        voxel_grid
    }

    pub fn debug(&self, func: &mut dyn FnMut(f32, f32, f32, usize, f32)) {
        for x in 0..self.resolution {
            for y in 0..self.resolution {
                for z in 0..self.resolution {
                    func(x as f32, y as f32, z as f32, self.resolution, 0.1)
                }
            }
        }
    }

    // pub fn generate_from_mesh(&mut self, mesh: &Mesh) -> Self {
    //     let positions = mesh
    //         .attribute(Mesh::ATTRIBUTE_POSITION)
    //         .unwrap()
    //         .as_float3()
    //         .unwrap();

    //     for xyz in positions.iter() {
    //         self.data.push(xyz[0]);
    //         self.data.push(xyz[1]);
    //         self.data.push(xyz[2]);
    //     }

    //     self.clone()
    // }

    pub fn read(&self, x: usize, y: usize, z: usize) -> f32 {
        println!("{} {} {}", x, y, z);
        self.data[x + y * self.resolution + z * self.resolution * self.resolution]
    }

    pub fn read_all(&self) -> Vec<f32> {
        self.data.clone()
    }

    pub fn push(&mut self, value: f32) {
        self.data.push(value);
    }
}

#[derive(Debug, Clone)]
pub struct MarchingCubes {
    pub voxel_grid: VoxelGrid,
}

impl MarchingCubes {
    pub fn new(mesh: &Mesh, resolution: usize) -> Self {
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .unwrap();

        let mut voxel_grid = VoxelGrid::new(positions, resolution);

        Self { voxel_grid }
    }

    pub fn march_all(&self) -> Self {
        for x in 0..self.voxel_grid.resolution - 1 {
            for y in 0..self.voxel_grid.resolution - 1 {
                for z in 0..self.voxel_grid.resolution - 1 {
                    self.march((x, y, z));
                }
            }
        }

        self.clone()
    }

    pub fn march(&self, (x, y, z): (usize, usize, usize)) {
        let mut new_positions: Vec<[f32; 3]> = Vec::new();

        let triangulation = self.get_triangulation((x, y, z));
        println!("{:?}", triangulation);
    }

    #[rustfmt::skip]
    fn get_triangulation(&self, (x, y, z): (usize, usize, usize)) -> [i8; 15] {
        let mut config_idx = 0b00000000;

        config_idx |= (self.voxel_grid.read(x    , y    , z    ) > 0.0) as i8;
        config_idx |= (self.voxel_grid.read(x + 1, y    , z    ) > 0.0) as i8;
        config_idx |= (self.voxel_grid.read(x + 1, y    , z + 1) > 0.0) as i8;
        config_idx |= (self.voxel_grid.read(x    , y    , z + 1) > 0.0) as i8;
        config_idx |= (self.voxel_grid.read(x    , y + 1, z    ) > 0.0) as i8;
        config_idx |= (self.voxel_grid.read(x + 1, y + 1, z    ) > 0.0) as i8;
        config_idx |= (self.voxel_grid.read(x + 1, y + 1, z + 1) > 0.0) as i8;
        config_idx |= (self.voxel_grid.read(x    , y + 1, z + 1) > 0.0) as i8;

        return TRIANGULATIONS[config_idx as usize];
    }
}
