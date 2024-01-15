use crate::bevy_geo_nodes::geo_nodes::GeoNodeError;

use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use polars::prelude::*;

pub trait ToDataframe {
    fn to_dataframe(&self) -> Result<DataFrame, GeoNodeError>;
}

// fn add_columns_to_dataframe<I: IntoIterator<Item = [f32; 3]>>(
//     df: &mut DataFrame,
//     column_prefix: &str,
//     values: I,
// ) -> Result<(), GeoNodeError> {
//     let values: Vec<[f32; 3]> = values.into_iter().collect();
//     for (i, suffix) in ["x", "y", "z"].iter().enumerate() {
//         df.with_column(Series::new(
//             &format!("{}_{}", column_prefix, suffix),
//             values.iter().map(|v| v[i] as f64).collect::<Vec<_>>(),
//         ))
//         .or(Err(GeoNodeError::DataframeFromMeshError))?;
//     }

//     Ok(())
// }

macro_rules! define_as_series {
    ($type:ident, $num:expr, $name:ident) => {
        VertexAttributeValues::$type(values) => {
            let mut series = Vec::new();
            for i in 0..$num {
                series.push(create_series(
                    &format!("{}_{}", $name, ['x', 'y', 'z', 'w'][i]),
                    &values.iter().map(|x| x[i] as f64).collect::<Vec<_>>(),
                ));
            }
            Ok(series)
        }
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
            VertexAttributeValues::Float32(values) => Ok(vec![Series::new(name, values.to_vec())]),
            VertexAttributeValues::Sint32(values) => Ok(vec![Series::new(name, values.to_vec())]),
            VertexAttributeValues::Uint32(values) => Ok(vec![Series::new(name, values.to_vec())]),
            _ => Err(GeoNodeError::DataframeFromMeshError),
        }
    }

    fn as_series2(&self, name: &str) -> Result<Vec<Series>, GeoNodeError> {
        match self {
            VertexAttributeValues::Float32x2(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Sint32x2(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Uint32x2(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Sint8x2(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Snorm8x2(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Uint8x2(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Unorm8x2(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Sint16x2(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Snorm16x2(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Uint16x2(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Unorm16x2(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
            ]),
            _ => Err(GeoNodeError::DataframeFromMeshError),
        }
    }

    fn as_series3(&self, name: &str) -> Result<Vec<Series>, GeoNodeError> {
        match self {
            VertexAttributeValues::Float32x3(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[2] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Sint32x3(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[2] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Uint32x3(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[2] as f64).collect::<Vec<_>>()),
            ]),
            _ => Err(GeoNodeError::DataframeFromMeshError),
        }
    }

    fn as_series4(&self, name: &str) -> Result<Vec<Series>, GeoNodeError> {
        match self {
            VertexAttributeValues::Float32x4(values) => Ok(vec![
                Series::new(
                    &format!("{}_x", name)[..],
                    values.iter().map(|x| x[0] as f64).collect::<Vec<_>>(),
                ),
                Series::new(
                    &format!("{}_y", name)[..],
                    values.iter().map(|x| x[1] as f64).collect::<Vec<_>>(),
                ),
                Series::new(
                    &format!("{}_z", name)[..],
                    values.iter().map(|x| x[2] as f64).collect::<Vec<_>>(),
                ),
                Series::new(
                    &format!("{}_w", name)[..],
                    values.iter().map(|x| x[3] as f64).collect::<Vec<_>>(),
                ),
            ]),
            VertexAttributeValues::Sint32x4(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[2] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[3] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Uint32x4(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[2] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[3] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Sint16x4(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[2] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[3] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Snorm16x4(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[2] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[3] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Uint16x4(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[2] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[3] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Unorm16x4(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[2] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[3] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Sint8x4(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[2] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[3] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Snorm8x4(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[2] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[3] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Uint8x4(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[2] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[3] as f64).collect::<Vec<_>>()),
            ]),
            VertexAttributeValues::Unorm8x4(values) => Ok(vec![
                Series::new(name, values.iter().map(|x| x[0] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[1] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[2] as f64).collect::<Vec<_>>()),
                Series::new(name, values.iter().map(|x| x[3] as f64).collect::<Vec<_>>()),
            ]),
            _ => Err(GeoNodeError::DataframeFromMeshError),
        }
    }
}

fn append_all_series_to_dataframe(
    df: &mut DataFrame,
    series: Vec<Series>,
) -> Result<(), GeoNodeError> {
    for series in series {
        df.with_column(series)
            .or(Err(GeoNodeError::DataframeFromMeshError))?;
    }

    Ok(())
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

        let positions_iter = positions.as_series3("position")?;
        let normals_iter = normals.as_series3("normal")?;

        for series in positions_iter {
            df.with_column(series)
                .or(Err(GeoNodeError::DataframeFromMeshError))?;
        }

        for series in normals_iter {
            df.with_column(series)
                .or(Err(GeoNodeError::DataframeFromMeshError))?;
        }

        Ok(df)
    }
}
