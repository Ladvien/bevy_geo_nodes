use std::os::raw;

use bevy::render::{mesh::VertexAttributeValues, render_resource::PrimitiveTopology};
use three_d_asset::{Indices, Positions, TriMesh, Vector2, Vector3, Vector4};

pub trait MeshConverter {
    fn to_trimesh(&self) -> tri_mesh::Mesh;
    fn to_bevy_mesh(&self) -> bevy::render::mesh::Mesh;
}

impl MeshConverter for tri_mesh::Mesh {
    fn to_bevy_mesh(&self) -> bevy::render::mesh::Mesh {
        let mut mesh = bevy::render::mesh::Mesh::new(PrimitiveTopology::TriangleList);

        let raw_mesh = &self.export();

        // println!("Raw mesh: {:?}", raw_mesh.uvs);

        let mut bevy_positions = Vec::new();
        match &raw_mesh.positions {
            Positions::F32(positions) => positions.iter().for_each(|x| {
                bevy_positions.push([x.x, x.y, x.z]);
            }),
            Positions::F64(positions) => positions.iter().for_each(|position| {
                bevy_positions.push([position.x as f32, position.y as f32, position.z as f32]);
            }),
            _ => {
                panic!("Positions are not Float3");
            }
        };

        mesh.insert_attribute(
            bevy::render::mesh::Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(bevy_positions),
        );

        let mut bevy_normals = Vec::new();
        match &raw_mesh.normals {
            Some(normals) => normals.iter().for_each(|x| {
                bevy_normals.push([x.x, x.y, x.z]);
            }),
            None => todo!(),
        };

        mesh.insert_attribute(
            bevy::render::mesh::Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(bevy_normals),
        );

        // let mut bevy_uvs = Vec::new();

        // match &raw_mesh.uvs {
        //     Some(uvs) => uvs.iter().for_each(|x| {
        //         bevy_uvs.push([x.x, x.y]);
        //     }),
        //     None => println!("No UVs found"),
        // };

        // println!("UVs: {:?}", bevy_uvs);

        // mesh.insert_attribute(
        //     bevy::render::mesh::Mesh::ATTRIBUTE_UV_0,
        //     VertexAttributeValues::Float32x2(bevy_uvs),
        // );

        mesh
    }

    fn to_trimesh(&self) -> tri_mesh::Mesh {
        self.clone()
    }
}

impl MeshConverter for bevy::render::mesh::Mesh {
    fn to_trimesh(&self) -> tri_mesh::Mesh {
        let positions = self
            .attribute(bevy::render::mesh::Mesh::ATTRIBUTE_POSITION)
            .unwrap();

        let normals = self
            .attribute(bevy::render::mesh::Mesh::ATTRIBUTE_NORMAL)
            .unwrap();
        let uv = self
            .attribute(bevy::render::mesh::Mesh::ATTRIBUTE_UV_0)
            .unwrap();

        // let uv1 = self
        //     .attribute(bevy::render::mesh::Mesh::ATTRIBUTE_UV_1)
        //     .unwrap();
        let default_color = VertexAttributeValues::Float32x4(vec![[
            1.0 as f32, 1.0 as f32, 1.0 as f32, 1.0 as f32,
        ]]);

        let color = self
            .attribute(bevy::render::mesh::Mesh::ATTRIBUTE_COLOR)
            .or(Some(&default_color));
        let positions = match positions {
            VertexAttributeValues::Float32x3(positions) => positions
                .iter()
                .map(|x| Vector3::new(x[0], x[1], x[2]))
                .collect::<Vec<Vector3<f32>>>(),
            _ => panic!("Positions are not Float3"),
        };

        // Convert normals from Bevy to TriMesh normals
        let normals = match normals {
            VertexAttributeValues::Float32x3(normals) => normals
                .iter()
                .map(|x| Vector3::new(x[0], x[1], x[2]))
                .collect::<Vec<Vector3<f32>>>(),
            _ => panic!("Normals are not Float3"),
        };

        let uv = match uv {
            VertexAttributeValues::Float32x2(uv) => uv
                .iter()
                .map(|x| Vector2::new(x[0], x[1]))
                .collect::<Vec<Vector2<f32>>>(),
            _ => panic!("UVs are not Float2"),
        };

        // TODO: Does uv1 need to be added to the TriMesh UV vector?
        // let uv1 = match uv1 {
        //     VertexAttributeValues::Float32x2(uv1) => uv1
        //         .iter()
        //         .map(|x| Vector2::new(x[0], x[1]))
        //         .collect::<Vec<Vector2<f32>>>(),
        //     _ => panic!("UVs are not Float2"),
        // };

        // let color = match color {
        //     VertexAttributeValues::Unorm8x4(color) => color
        //         .iter()
        //         .map(|x| three_d_asset::Color::new(x[0], x[1], x[2], x[3]))
        //         .collect::<Vec<three_d_asset::Color>>(),
        //     _ => panic!("Colors are not Unorm8x4"),
        // };

        let indices = self.indices().unwrap();

        let raw_mesh = TriMesh {
            indices: Indices::U32(indices.clone().iter().map(|x| x as u32).collect()),
            normals: Some(normals),
            uvs: Some(uv),
            name: "Test".to_owned(),
            // material_name: todo!(),
            positions: Positions::F32(positions.clone()),
            // tangents: todo!(),
            colors: color.map(|x| match x {
                VertexAttributeValues::Float32x3(color) => color
                    .iter()
                    .map(|x| {
                        three_d_asset::Color::new(x[0] as u8, x[1] as u8, x[2] as u8, 1.0 as u8)
                    })
                    .collect::<Vec<three_d_asset::Color>>(),
                VertexAttributeValues::Float32x4(color) => color
                    .iter()
                    .map(|x| {
                        three_d_asset::Color::new(x[0] as u8, x[1] as u8, x[2] as u8, x[3] as u8)
                    })
                    .collect::<Vec<three_d_asset::Color>>(),
                _ => panic!("Colors are not Float32x3 or Float32x4"),
            }),
            ..Default::default()
        };

        let return_mesh = tri_mesh::Mesh::new(&raw_mesh);

        return_mesh
    }

    fn to_bevy_mesh(&self) -> bevy::render::mesh::Mesh {
        self.clone()
    }
}
