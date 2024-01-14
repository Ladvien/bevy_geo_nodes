/* Interpolation is a curve fitting process, where a set of points is used
to fit the shape of a curve such that the curve closely passes through the
relevant set of points. In effect, it is a process of determining parameters
in a model; for CFD simulations, these curve parameters define the curvature
of surfaces along which a fluid will flow. Many interpolation methods exist
for different types of curves, all of which are intended to balance the
resolution and computational power required to converge to an accurate mesh.

https://resources.system-analysis.cadence.com/blog/msa2022-the-best-methods-for-mesh-interpolation
*/
fn interpolate(pL: Vec3, pR: Vec3, x: f32) -> Vec2 {
    let dxL = x - pL.x;
    let dxR = x - pR.x;

    #[cfg(MANIFOLD_DEBUG)]
    check_domain(dxL, dxR);

    let useL = f32::abs(dxL) < f32::abs(dxR);
    let lambda = if useL { dxL } else { dxR } / (pR.x - pL.x);

    if !lambda.is_finite() {
        return Vec2::new(pL.y, pL.z);
    }

    let mut yz = Vec2::default();
    yz[0] = if useL { pL.y } else { pR.y } + lambda * (pR.y - pL.y);
    yz[1] = if useL { pL.z } else { pR.z } + lambda * (pR.z - pL.z);

    yz
}

/* Intersect is a curve fitting process, where a set of points is used
to fit the shape of a curve such that the curve closely passes through the
relevant set of points. In effect, it is a process of determining parameters
in a model; for CFD simulations, these curve parameters define the curvature
of surfaces along which a fluid will flow. Many interpolation methods exist
for different types of curves, all of which are intended to balance the
resolution and computational power required to converge to an accurate mesh.

https://cp-algorithms.com/others/stern_brocot_tree_farey_sequences.html
*/
pub fn intersect(pL: Vec3, pR: Vec3, qL: Vec3, qR: Vec3) -> Vec4 {
    let dyL = qL.y - pL.y;
    let dyR = qR.y - pR.y;

    #[cfg(MANIFOLD_DEBUG)]
    check_intersection(dyL, dyR);

    let useL = f32::abs(dyL) < f32::abs(dyR);
    let dx = pR.x - pL.x;
    let lambda = if useL { dyL } else { dyR } / (dyL - dyR);

    if !lambda.is_finite() {
        return Vec4::new(pL.x, pL.y, pL.z, qL.z);
    }

    let mut xyzz = Vec4::default();
    xyzz.x = if useL { pL.x } else { pR.x } + lambda * dx;

    let pDy = pR.y - pL.y;
    let qDy = qR.y - qL.y;
    let useP = f32::abs(pDy) < f32::abs(qDy);

    xyzz.y = if useL {
        if useP {
            pL.y
        } else {
            qL.y
        }
    } else if useP {
        pR.y
    } else {
        qR.y
    } + lambda * if useP { pDy } else { qDy };

    xyzz.z = if useL { pL.z } else { pR.z } + lambda * (pR.z - pL.z);
    xyzz.w = if useL { qL.z } else { qR.z } + lambda * (qR.z - qL.z);

    xyzz
}

use bevy::math::{Vec2, Vec3, Vec4};

use super::GeoNode;

pub trait BooleanOperations {
    fn union(&self, other: &Self) -> Self;
    fn difference(&self, other: &Self) -> Self;
    fn intersection(&self, other: &Self) -> Self;
}

impl BooleanOperations for GeoNode {
    fn union(&self, other: &Self) -> Self {
        println!("{:?}", self);

        return self.clone();
    }

    fn difference(&self, other: &Self) -> Self {
        self.clone()
    }

    fn intersection(&self, other: &Self) -> Self {
        self.clone()
    }
}
