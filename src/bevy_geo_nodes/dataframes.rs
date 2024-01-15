use crate::bevy_geo_nodes::geo_nodes::GeoNodeError;

use bevy::prelude::*;
use polars::prelude::*;

pub trait ToDataframe {
    fn to_dataframe(&self) -> Result<DataFrame, GeoNodeError>;
}

fn add_columns_to_dataframe<I: IntoIterator<Item = [f32; 3]>>(
    df: &mut DataFrame,
    column_prefix: &str,
    values: I,
) -> Result<(), GeoNodeError> {
    let values: Vec<[f32; 3]> = values.into_iter().collect();
    for (i, suffix) in ["x", "y", "z"].iter().enumerate() {
        df.with_column(Series::new(
            &format!("{}_{}", column_prefix, suffix),
            values.iter().map(|x| x[i] as f64).collect::<Vec<_>>(),
        ))
        .or(Err(GeoNodeError::DataframeFromMeshError))?;
    }

    Ok(())
}

fn add_column_to_dataframe<I: IntoIterator<Item = f64>>(
    df: &mut DataFrame,
    column_name: &str,
    values: I,
) -> Result<(), GeoNodeError> {
    df.with_column(Series::new(
        column_name,
        values.into_iter().collect::<Vec<_>>(),
    ))
    .or(Err(GeoNodeError::FailedToGetColumnError(
        column_name.to_string(),
    )))?;

    Ok(())
}

impl ToDataframe for Mesh {
    fn to_dataframe(&self) -> Result<DataFrame, GeoNodeError> {
        let mut df = DataFrame::default();
        let positions = self.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        let normals = self.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap();
        let uvs = self.attribute(Mesh::ATTRIBUTE_UV_0).unwrap();

        let positions_iter: Vec<[f32; 3]> = positions.as_float3().unwrap().to_vec();
        let normals_iter: Vec<[f32; 3]> = normals.as_float3().unwrap().to_vec();
        let uvs_iter = uvs
            .as_float3()
            .ok_or_else(|| GeoNodeError::FailedToGetColumnError("uv_x".to_string()))?
            .to_vec();

        add_columns_to_dataframe(&mut df, "position", positions_iter)?;
        add_columns_to_dataframe(&mut df, "normal", normals_iter)?;
        add_column_to_dataframe(&mut df, "uv_x", uvs_iter.iter().map(|x| x[0] as f64))?;

        println!("{:?}", df);

        Ok(df)
    }
}
