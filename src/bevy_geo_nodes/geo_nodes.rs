use super::boolean::BooleanOperations;
use bevy::prelude::*;

pub trait Node {}

#[derive(Debug, Clone)]
pub struct GeoNode {
    pub mesh: Mesh,
    pub material: StandardMaterial,
    pub scale: Vec3,
}

impl Default for GeoNode {
    fn default() -> Self {
        Self {
            mesh: Mesh::from(shape::Cube::new(0.40)),
            material: StandardMaterial::default(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Node for GeoNode {}

impl GeoNode {
    pub fn get_mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn get_material(&self) -> &StandardMaterial {
        &self.material
    }

    pub fn set_scale(&mut self, scale: Vec3) {}

    pub fn get_pbr_bundle(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> PbrBundle {
        PbrBundle {
            mesh: meshes.add(self.mesh.clone()),
            material: materials.add(self.material.clone()),
            transform: Transform::from_scale(self.scale),
            ..Default::default()
        }
    }

    pub fn from_mesh(mesh: Mesh) -> Self {
        Self {
            mesh,
            ..Default::default()
        }
    }

    pub fn combine(&mut self, other: GeoNode) {
        self.union(&other);
    }

    pub fn merge(&mut self, other: GeoNode) {
        // ... existing merge code ...
    }
}

#[derive(Debug)]
pub enum GeoNodeError {
    MeshError,
    MaterialError,
    DataframeFromMeshError,
    FailedToGetColumnError(String),
}

impl std::fmt::Display for GeoNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeoNodeError::MeshError => write!(f, "Failed to get mesh"),
            GeoNodeError::MaterialError => write!(f, "Failed to get material"),
            GeoNodeError::DataframeFromMeshError => {
                write!(f, "Failed to get dataframe from mesh")
            }
            GeoNodeError::FailedToGetColumnError(column_name) => {
                write!(f, "Failed to get column {}", column_name)
            }
        }
    }
}

impl std::error::Error for GeoNodeError {}
