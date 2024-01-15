use crate::bevy_geo_nodes::geo_nodes::GeoNodeError;

use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use polars::prelude::*;

pub trait ToDataframe {
    fn to_dataframe(&self) -> Result<DataFrame, GeoNodeError>;
}

pub trait ToSeries {
    fn as_series1(&self, name: &str) -> Result<Vec<Series>, GeoNodeError>;
    fn as_series2(&self, name: &str) -> Result<Vec<Series>, GeoNodeError>;
    fn as_series3(&self, name: &str) -> Result<Vec<Series>, GeoNodeError>;
    fn as_series4(&self, name: &str) -> Result<Vec<Series>, GeoNodeError>;
}

macro_rules! define_as_series1 {
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

macro_rules! series_nth {
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

impl ToSeries for VertexAttributeValues {
    fn as_series1(&self, name: &str) -> Result<Vec<Series>, GeoNodeError> {
        match self {
            VertexAttributeValues::Float32(values) => define_as_series1!(Float32, 1, name, values),
            VertexAttributeValues::Sint32(values) => define_as_series1!(Sint32, 1, name, values),
            VertexAttributeValues::Uint32(values) => define_as_series1!(Uint32, 1, name, values),
            _ => Err(GeoNodeError::DataframeFromMeshError),
        }
    }

    fn as_series2(&self, name: &str) -> Result<Vec<Series>, GeoNodeError> {
        match self {
            VertexAttributeValues::Float32x2(values) => {
                series_nth!(Float32x2, 2, name, values)
            }
            VertexAttributeValues::Sint32x2(values) => {
                series_nth!(Sint32x2, 2, name, values)
            }
            VertexAttributeValues::Uint32x2(values) => {
                series_nth!(Uint32x2, 2, name, values)
            }
            VertexAttributeValues::Sint8x2(values) => {
                series_nth!(Sint8x2, 2, name, values)
            }
            VertexAttributeValues::Snorm8x2(values) => {
                series_nth!(Snorm8x2, 2, name, values)
            }
            VertexAttributeValues::Uint8x2(values) => {
                series_nth!(Uint8x2, 2, name, values)
            }
            VertexAttributeValues::Unorm8x2(values) => {
                series_nth!(Unorm8x2, 2, name, values)
            }
            VertexAttributeValues::Sint16x2(values) => {
                series_nth!(Sint16x2, 2, name, values)
            }
            VertexAttributeValues::Snorm16x2(values) => {
                series_nth!(Snorm16x2, 2, name, values)
            }
            VertexAttributeValues::Uint16x2(values) => series_nth!(Uint16x2, 2, name, values),
            VertexAttributeValues::Unorm16x2(values) => {
                series_nth!(Unorm16x2, 2, name, values)
            }
            _ => Err(GeoNodeError::DataframeFromMeshError),
        }
    }

    fn as_series3(&self, name: &str) -> Result<Vec<Series>, GeoNodeError> {
        match self {
            VertexAttributeValues::Float32x3(values) => series_nth!(Float32x3, 3, name, values),
            VertexAttributeValues::Sint32x3(values) => series_nth!(Sint32x3, 3, name, values),
            VertexAttributeValues::Uint32x3(values) => series_nth!(Uint32x3, 3, name, values),
            _ => Err(GeoNodeError::DataframeFromMeshError),
        }
    }

    fn as_series4(&self, name: &str) -> Result<Vec<Series>, GeoNodeError> {
        match self {
            VertexAttributeValues::Float32x4(values) => series_nth!(Float32x4, 4, name, values),
            VertexAttributeValues::Sint32x4(values) => series_nth!(Sint32x4, 4, name, values),
            VertexAttributeValues::Uint32x4(values) => series_nth!(Uint32x4, 4, name, values),
            VertexAttributeValues::Sint16x4(values) => series_nth!(Sint16x4, 4, name, values),
            VertexAttributeValues::Snorm16x4(values) => series_nth!(Snorm16x4, 4, name, values),
            VertexAttributeValues::Uint16x4(values) => series_nth!(Uint16x4, 4, name, values),
            VertexAttributeValues::Unorm16x4(values) => series_nth!(Unorm16x4, 4, name, values),
            VertexAttributeValues::Sint8x4(values) => series_nth!(Sint8x4, 4, name, values),
            VertexAttributeValues::Snorm8x4(values) => series_nth!(Snorm8x4, 4, name, values),
            VertexAttributeValues::Uint8x4(values) => series_nth!(Uint8x4, 4, name, values),
            VertexAttributeValues::Unorm8x4(values) => series_nth!(Unorm8x4, 4, name, values),
            _ => Err(GeoNodeError::DataframeFromMeshError),
        }
    }
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
