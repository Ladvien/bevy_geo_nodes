use crate::bevy_geo_nodes::geo_nodes::GeoNodeError;

use bevy::{
    asset::io::memory::Data,
    prelude::*,
    render::{mesh::VertexAttributeValues, render_resource::PrimitiveTopology},
};
use polars::{frame::row::Row, prelude::*, time::series::AsSeries};

macro_rules! to_series {
    ($type:ty, $len:expr, $name:expr, $values:expr) => {
        Ok({
            (0..$len)
                .map(|i| {
                    Series::new(
                        &format!("{}_{}", $name, i)[..],
                        $values.iter().map(|x| *x as f64).collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>()
        })
    };
}

macro_rules! to_series_nth {
    ($type:ty, $len:expr, $name:expr, $values:expr) => {
        Ok({
            (0..$len)
                .map(|i| {
                    Series::new(
                        &format!("{}_{}", $name, i)[..],
                        $values.iter().map(|x| x[i] as f64).collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>()
        })
    };
}

pub trait ToSeries {
    fn as_series1(&self, name: &str) -> Result<Vec<Series>, GeoNodeError>;
    fn as_series2(&self, name: &str) -> Result<Vec<Series>, GeoNodeError>;
    fn as_series3(&self, name: &str) -> Result<Vec<Series>, GeoNodeError>;
    fn as_series4(&self, name: &str) -> Result<Vec<Series>, GeoNodeError>;
}

impl ToSeries for VertexAttributeValues {
    fn as_series1(&self, name: &str) -> Result<Vec<Series>, GeoNodeError> {
        match self {
            VertexAttributeValues::Float32(values) => to_series!(Float32, 1, name, values),
            VertexAttributeValues::Sint32(values) => to_series!(Sint32, 1, name, values),
            VertexAttributeValues::Uint32(values) => to_series!(Uint32, 1, name, values),
            _ => Err(GeoNodeError::DataframeFromMeshError),
        }
    }

    fn as_series2(&self, name: &str) -> Result<Vec<Series>, GeoNodeError> {
        match self {
            VertexAttributeValues::Float32x2(values) => {
                to_series_nth!(Float32x2, 2, name, values)
            }
            VertexAttributeValues::Sint32x2(values) => {
                to_series_nth!(Sint32x2, 2, name, values)
            }
            VertexAttributeValues::Uint32x2(values) => {
                to_series_nth!(Uint32x2, 2, name, values)
            }
            VertexAttributeValues::Sint8x2(values) => {
                to_series_nth!(Sint8x2, 2, name, values)
            }
            VertexAttributeValues::Snorm8x2(values) => {
                to_series_nth!(Snorm8x2, 2, name, values)
            }
            VertexAttributeValues::Uint8x2(values) => {
                to_series_nth!(Uint8x2, 2, name, values)
            }
            VertexAttributeValues::Unorm8x2(values) => {
                to_series_nth!(Unorm8x2, 2, name, values)
            }
            VertexAttributeValues::Sint16x2(values) => {
                to_series_nth!(Sint16x2, 2, name, values)
            }
            VertexAttributeValues::Snorm16x2(values) => {
                to_series_nth!(Snorm16x2, 2, name, values)
            }
            VertexAttributeValues::Uint16x2(values) => to_series_nth!(Uint16x2, 2, name, values),
            VertexAttributeValues::Unorm16x2(values) => {
                to_series_nth!(Unorm16x2, 2, name, values)
            }
            _ => Err(GeoNodeError::DataframeFromMeshError),
        }
    }

    fn as_series3(&self, name: &str) -> Result<Vec<Series>, GeoNodeError> {
        match self {
            VertexAttributeValues::Float32x3(values) => to_series_nth!(Float32x3, 3, name, values),
            VertexAttributeValues::Sint32x3(values) => to_series_nth!(Sint32x3, 3, name, values),
            VertexAttributeValues::Uint32x3(values) => to_series_nth!(Uint32x3, 3, name, values),
            _ => Err(GeoNodeError::DataframeFromMeshError),
        }
    }

    fn as_series4(&self, name: &str) -> Result<Vec<Series>, GeoNodeError> {
        match self {
            VertexAttributeValues::Float32x4(values) => to_series_nth!(Float32x4, 4, name, values),
            VertexAttributeValues::Sint32x4(values) => to_series_nth!(Sint32x4, 4, name, values),
            VertexAttributeValues::Uint32x4(values) => to_series_nth!(Uint32x4, 4, name, values),
            VertexAttributeValues::Sint16x4(values) => to_series_nth!(Sint16x4, 4, name, values),
            VertexAttributeValues::Snorm16x4(values) => to_series_nth!(Snorm16x4, 4, name, values),
            VertexAttributeValues::Uint16x4(values) => to_series_nth!(Uint16x4, 4, name, values),
            VertexAttributeValues::Unorm16x4(values) => to_series_nth!(Unorm16x4, 4, name, values),
            VertexAttributeValues::Sint8x4(values) => to_series_nth!(Sint8x4, 4, name, values),
            VertexAttributeValues::Snorm8x4(values) => to_series_nth!(Snorm8x4, 4, name, values),
            VertexAttributeValues::Uint8x4(values) => to_series_nth!(Uint8x4, 4, name, values),
            VertexAttributeValues::Unorm8x4(values) => to_series_nth!(Unorm8x4, 4, name, values),
            _ => Err(GeoNodeError::DataframeFromMeshError),
        }
    }
}

pub trait ToDataframe {
    fn to_dataframe(&self) -> Result<DataFrame, GeoNodeError>;
}

impl ToDataframe for Mesh {
    fn to_dataframe(&self) -> Result<DataFrame, GeoNodeError> {
        let mut df = DataFrame::default();
        let positions = self.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        let normals = self.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap();
        let uvs = self
            .attribute(Mesh::ATTRIBUTE_UV_0)
            .ok_or(GeoNodeError::FailedToGetColumnError("uv".to_string()))?
            .as_series2("uv")?;

        for series in positions.as_series3("position")? {
            df.with_column(series)
                .or(Err(GeoNodeError::DataframeFromMeshError))?;
        }

        for series in normals.as_series3("normal")? {
            df.with_column(series)
                .or(Err(GeoNodeError::DataframeFromMeshError))?;
        }

        for series in uvs {
            df.with_column(series)
                .or(Err(GeoNodeError::DataframeFromMeshError))?;
        }

        Ok(df)
    }
}

pub trait ToMesh {
    fn to_mesh(&self) -> Result<Mesh, GeoNodeError>;
}

impl ToMesh for DataFrame {
    fn to_mesh(&self) -> Result<Mesh, GeoNodeError> {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        // let mut positions = Vec::new();
        // let mut normals = Vec::new();
        // let mut uvs = Vec::new();

        // I need a way to convert three polars Series into a Vec<Vec3>>
        // let position = &self.to_vec_of_vec3("position_0", "position_1", "position_2")?;
        // let normal = &self.to_vec_of_vec3("normal_0", "normal_1", "normal_2")?;
        // let uv = &self.to_vec_of_vec3("uv_0", "uv_1", "uv_2")?;

        // let t = VertexAttributeValues::Float32x3(position.clone());

        // let new_mesh = Mesh::from(&[(
        //     ,
        //     VertexAttributeValues::Float32x3(normal.clone()),
        //     VertexAttributeValues::Float32x2(uv.clone()),
        // )]);

        Ok(mesh)
    }
}

pub trait ToVecOfVec3 {
    fn to_vec_of_vec3(
        &self,
        col_x_name: &str,
        col_y_name: &str,
        col_z_name: &str,
    ) -> Result<Vec<[f32; 3]>, GeoNodeError>;
}

// impl ToVecOfVec3 for DataFrame {
//     fn to_vec_of_vec3(
//         &self,
//         col_x_name: &str,
//         col_y_name: &str,
//         col_z_name: &str,
//     ) -> Result<Vec<[f32; 3]>, GeoNodeError> {
//         // println!("{:?}", self.get_columns());

//         let result = self.select(&[col_x_name, col_y_name, col_z_name]).unwrap();
//         let mut new_vec: Vec<[f32; 3]> = Vec::new();

//         for row_i in 0..result.height() {
//             let row = result.get_row(row_i).unwrap();

//             let x_value: f32 = row.get_nth(0).unwrap().unwrap_or(0.0) as f32;
//             let y_value: f32 = row.get_nth(1).unwrap().unwrap_or(0.0) as f32;
//             let z_value: f32 = row.get_nth(2).unwrap().unwrap_or(0.0) as f32;

//             new_vec.push([x_value, y_value, z_value]);
//         }

//         Ok(new_vec)
//     }
// }
