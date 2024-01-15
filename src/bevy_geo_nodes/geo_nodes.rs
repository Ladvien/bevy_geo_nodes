use super::boolean::BooleanOperations;
use bevy::{
    prelude::*,
    render::{
        mesh::{self},
        render_resource::PrimitiveTopology,
    },
};
use std::fmt;

pub trait Node {
    // fn from_mesh(&self, mesh: Mesh) -> Self;
    // fn get_mesh(&self) -> &Mesh;
    // fn get_material(&self) -> &StandardMaterial;
    // fn get_pbr_bundle(&mut self) -> PbrBundle;
}

#[derive(Debug, Clone)]
pub struct GeoNode {
    pub mesh: Mesh,
    pub material: StandardMaterial,
    pub scale: Vec3,
}

impl Default for GeoNode {
    fn default() -> GeoNode {
        GeoNode {
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
        GeoNode {
            mesh: mesh,
            ..Default::default()
        }
    }

    pub fn combine(&mut self, other: GeoNode) {
        &self.union(&other);
    }

    pub fn merge(&mut self, other: GeoNode) {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            self.mesh
                .attribute(Mesh::ATTRIBUTE_POSITION)
                .unwrap()
                .clone(),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            self.mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap().clone(),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            self.mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap().clone(),
        );
        mesh.set_indices(Some(mesh::Indices::U32(
            self.mesh
                .indices()
                .unwrap()
                .clone()
                .iter()
                .map(|x| x as u32)
                .collect::<Vec<u32>>(),
        )));

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            other
                .mesh
                .attribute(Mesh::ATTRIBUTE_POSITION)
                .unwrap()
                .clone(),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            other
                .mesh
                .attribute(Mesh::ATTRIBUTE_NORMAL)
                .unwrap()
                .clone(),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            other.mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap().clone(),
        );
        mesh.set_indices(Some(mesh::Indices::U32(
            other
                .mesh
                .indices()
                .unwrap()
                .clone()
                .iter()
                .map(|x| x as u32)
                .collect::<Vec<u32>>(),
        )));
        // println!("{:?}", mesh);
        self.mesh = mesh;
    }
}

pub struct GeoNodeError;
impl fmt::Display for GeoNodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GeoNodeError")
    }
}

impl fmt::Debug for GeoNodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GeoNodeError")
    }
}
