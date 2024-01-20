# bevy_geo_nodes
An experiment to see if "Geometry Nodes" like framework can be replicated in the Bevy engine


## Reads
https://github.com/hjwdzh/ManifoldPlus
https://www.wihlidal.com/blog/pipeline/2018-10-20-rust-mesh-optimizer/

## Reference Libraries
https://github.com/gwihlidal/meshopt-rs

## Performance
https://www.reddit.com/r/rust/comments/vl72g3/polars_dataframes_vs_vectors_of_structs/

## Geometry
https://github.com/swiftcoder/isosurface/tree/trunk
https://iquilezles.org/articles/distfunctions/

## Voxelization
https://www.gamedeveloper.com/programming/triangle-mesh-voxelization#close-modal
https://developer.nvidia.com/content/basics-gpu-voxelization
https://fileadmin.cs.lth.se/cs/Personal/Tomas_Akenine-Moller/code/tribox_tam.pdf

## Marching Cubes & Mesh Creation
https://www.youtube.com/watch?v=KvwVYJY_IZ4
https://dl.acm.org/doi/pdf/10.1145/280811.281026
[Marching Cubes Lvl. 2!](https://github.com/Twinklebear/webgpu-bcmc)
https://transvoxel.org/
https://towardsdatascience.com/how-to-voxelize-meshes-and-point-clouds-in-python-ca94d403f81d
https://bonsairobo.medium.com/smooth-voxel-mapping-a-technical-deep-dive-on-real-time-surface-nets-and-texturing-ef06d0f8ca14
https://core.ac.uk/download/pdf/295558237.pdf
https://github.com/IceSentry/bevy_marching_cube/blob/master/src/main.rs
https://sites.google.com/site/letsmakeavoxelengine/

## Definitions
* Voxel -> The three-dimensional analogue of a pixel; a volume element representing some numerical quantity, such as the colour, of a point in three-dimensional space, used in the visualisation and analysis of three-dimensional (especially scientific and medical) data.
* Normals -> A normal in 3D modeling is a depiction of the orientation of a polygon's surface. It's basically a perpendicular line jutting out from the plane. When you're dealing with a curve, you'll use the plane lying tangent to the point in question to find its normal.
* Scaler -> A scalar is an element of a field which is used to define a vector space. In linear algebra, real numbers or generally elements of a field are called scalars and relate to vectors in an associated vector space through the operation of scalar multiplication (defined in the vector space), in which a vector can be multiplied by a scalar in the defined way to produce another vector.[1][2][3] Generally speaking, a vector space may be defined by using any field instead of real numbers (such as complex numbers). Then scalars of that vector space will be elements of the associated field (such as complex numbers). 
* Vector -> In mathematics, physics, and engineering, a Euclidean vector or simply a vector (sometimes called a geometric vector[1] or spatial vector[2]) is a geometric object that has magnitude (or length) and direction.
* Vertex -> In geometry, a vertex (pl.: vertices or vertexes) is a point where two or more curves, lines, or edges meet or intersect. As a consequence of this definition, the point where two lines meet to form an angle and the corners of polygons and polyhedra are vertices.
* Tessellation -> A tessellation or tiling is the covering of a surface, often a plane, using one or more geometric shapes, called tiles, with no overlaps and no gaps. In mathematics, tessellation can be generalized to higher dimensions and a variety of geometries. 
* 
##