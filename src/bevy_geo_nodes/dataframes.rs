use super::geo_nodes::GeoNodeError;
use bevy::prelude::*;
use polars::prelude::*;

pub trait ToDataframe {
    fn to_dataframe(&self) -> Result<DataFrame, GeoNodeError>;
}

impl ToDataframe for Mesh {
    fn to_dataframe(&self) -> Result<DataFrame, GeoNodeError> {
        // TODO:
        // 1. Move to separate file
        // 2. Add support for other attributes
        // 3. DRY up code

        let mut df = DataFrame::default();
        let positions = self.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        let normals = self.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap();
        let uvs = self.attribute(Mesh::ATTRIBUTE_UV_0).unwrap();
        let indices = self.indices().unwrap();

        let positions_iter: Vec<&[f32; 3]> = positions.as_float3().unwrap().iter().collect();

        // Add columns
        df.with_column(Series::new(
            "position_x",
            positions_iter
                .iter()
                .map(|x| x[0] as f64)
                .collect::<Vec<f64>>(),
        ))
        .or(Err(GeoNodeError))?;

        df.with_column(Series::new(
            "position_y",
            positions_iter
                .iter()
                .map(|x| x[1] as f64)
                .collect::<Vec<f64>>(),
        ))
        .or(Err(GeoNodeError))?;

        df.with_column(Series::new(
            "position_z",
            positions_iter
                .iter()
                .map(|x| x[2] as f64)
                .collect::<Vec<f64>>(),
        ))
        .or(Err(GeoNodeError))?;

        let normals_iter: Vec<&[f32; 3]> = normals.as_float3().unwrap().iter().collect();

        df.with_column(Series::new(
            "normal_x",
            normals_iter
                .iter()
                .map(|x| x[0] as f64)
                .collect::<Vec<f64>>(),
        ))
        .or(Err(GeoNodeError))?;

        df.with_column(Series::new(
            "normal_y",
            normals_iter
                .iter()
                .map(|x| x[1] as f64)
                .collect::<Vec<f64>>(),
        ))
        .or(Err(GeoNodeError))?;

        df.with_column(Series::new(
            "normal_z",
            normals_iter
                .iter()
                .map(|x| x[2] as f64)
                .collect::<Vec<f64>>(),
        ))
        .or(Err(GeoNodeError))?;

        // let uvs_iter = uvs.as_float3().unwrap().iter().collect::<Vec<&[f32; 3]>>();

        // df.with_column(Series::new(
        //     "uv_x",
        //     uvs_iter.iter().map(|x| x[0] as f64).collect::<Vec<f64>>(),
        // ))
        // .unwrap();

        println!("{:?}", df);

        Ok(df)
    }
}
