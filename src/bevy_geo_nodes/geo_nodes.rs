use bevy::{
    pbr::wireframe::WireframeMaterial,
    prelude::*,
    render::{
        mesh::{self, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};

use three_d_asset::{Indices, Positions, TriMesh, Vector2, Vector3};

use super::mesh_converter::MeshConverter;
// Load tri_mesh

pub trait Node {
    fn new() -> Self;
    fn default() -> GeoNode;
    // fn from_mesh(&self, mesh: Mesh) -> Self;
    // fn get_mesh(&self) -> &Mesh;
    // fn get_material(&self) -> &StandardMaterial;
    // fn get_pbr_bundle(&mut self) -> PbrBundle;
}

pub struct GeoNode {
    pub mesh: Mesh,
    pub material: StandardMaterial,
    pub scale: Vec3,
}

impl Node for GeoNode {
    fn default() -> GeoNode {
        GeoNode {
            mesh: Mesh::from(shape::Cube::new(0.40)),
            material: StandardMaterial::default(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    fn new() -> Self {
        GeoNode {
            mesh: Mesh::new(PrimitiveTopology::LineList),
            material: StandardMaterial::default(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

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
        scale: Option<Vec3>,
    ) -> PbrBundle {
        let scale = match scale {
            Some(scale) => scale,
            None => Vec3::new(1.0, 1.0, 1.0),
        };

        PbrBundle {
            mesh: meshes.add(self.mesh.clone()),
            material: materials.add(self.material.clone()),
            transform: Transform::from_scale(self.scale),
            ..Default::default()
        }
    }

    pub fn from_mesh(&self, mesh: Mesh) -> Self {
        GeoNode {
            mesh: mesh,
            material: self.material.clone(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    fn convert_normals_to_tri_mesh(&self) -> tri_mesh::Mesh {
        self.mesh.to_trimesh()
    }

    pub fn combine(&mut self, other: GeoNode) {
        let mut mesh1 = self.mesh.to_trimesh();
        let mesh2 = other.mesh.to_trimesh();

        // Convert the TriMesh back to Bevy mesh
        mesh1.merge_with(&mesh2);

        self.mesh = mesh1.to_bevy_mesh()
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
