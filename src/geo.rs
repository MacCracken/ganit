//! Geometric primitives and intersection tests.
//!
//! Provides rays, planes, axis-aligned bounding boxes, spheres, and
//! ray-intersection routines.

use glam::Vec3;
use serde::{Deserialize, Serialize};

/// A ray defined by an origin and a direction.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Vec3,
    /// Should be normalized for correct distance results.
    pub direction: Vec3,
}

impl Ray {
    /// Create a new ray. Direction is normalized automatically.
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Point along the ray at parameter `t`.
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

/// An infinite plane defined by a normal and a signed distance from the origin.
///
/// Points **on** the plane satisfy `dot(normal, point) - distance == 0`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}

impl Plane {
    /// Create a plane from a point on the plane and a normal.
    pub fn from_point_normal(point: Vec3, normal: Vec3) -> Self {
        let n = normal.normalize();
        Self {
            normal: n,
            distance: n.dot(point),
        }
    }

    /// Signed distance from a point to the plane.
    /// Positive = same side as normal, negative = opposite side.
    pub fn signed_distance(&self, point: Vec3) -> f32 {
        self.normal.dot(point) - self.distance
    }
}

/// An axis-aligned bounding box.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    /// Create a new AABB. Min/max are corrected if swapped.
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            min: a.min(b),
            max: a.max(b),
        }
    }

    /// Check whether a point is inside (or on the boundary of) this AABB.
    #[inline]
    pub fn contains(&self, point: Vec3) -> bool {
        point.cmpge(self.min).all() && point.cmple(self.max).all()
    }

    /// Center point of the AABB.
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    /// Size (extents) of the AABB.
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    /// Merge two AABBs into one that encloses both.
    pub fn merge(&self, other: &Aabb) -> Aabb {
        Aabb {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }
}

/// A sphere defined by a center and radius.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Check whether a point is inside (or on the surface of) this sphere.
    #[inline]
    pub fn contains_point(&self, point: Vec3) -> bool {
        (point - self.center).length_squared() <= self.radius * self.radius
    }
}

/// Ray-plane intersection. Returns the `t` parameter if the ray hits the plane
/// (only `t >= 0`, i.e. forward hits).
#[inline]
pub fn ray_plane(ray: &Ray, plane: &Plane) -> Option<f32> {
    let denom = plane.normal.dot(ray.direction);
    if denom.abs() < 1e-8 {
        return None; // Ray parallel to plane
    }
    let t = (plane.distance - plane.normal.dot(ray.origin)) / denom;
    if t >= 0.0 { Some(t) } else { None }
}

/// Ray-sphere intersection using the quadratic formula.
/// Returns the nearest `t >= 0` if the ray hits the sphere.
///
/// Assumes `ray.direction` is normalized (guaranteed by `Ray::new`),
/// so the quadratic coefficient `a = 1` and is eliminated.
#[inline]
pub fn ray_sphere(ray: &Ray, sphere: &Sphere) -> Option<f32> {
    let oc = ray.origin - sphere.center;
    // With normalized direction: a=1, so b=2*dot(oc,d), c=dot(oc,oc)-r²
    // Use half-b form: half_b = dot(oc, d), discriminant = half_b² - c
    let half_b = oc.dot(ray.direction);
    let c = oc.dot(oc) - sphere.radius * sphere.radius;
    let discriminant = half_b * half_b - c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let t1 = -half_b - sqrt_d;
    let t2 = -half_b + sqrt_d;

    if t1 >= 0.0 {
        Some(t1)
    } else if t2 >= 0.0 {
        Some(t2)
    } else {
        None
    }
}

/// Ray-AABB intersection using the slab method.
/// Returns the nearest `t >= 0` if the ray hits the AABB.
#[inline]
pub fn ray_aabb(ray: &Ray, aabb: &Aabb) -> Option<f32> {
    let origin = ray.origin.to_array();
    let dir = ray.direction.to_array();
    let bb_min = aabb.min.to_array();
    let bb_max = aabb.max.to_array();

    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;

    for i in 0..3 {
        if dir[i].abs() < 1e-8 {
            if origin[i] < bb_min[i] || origin[i] > bb_max[i] {
                return None;
            }
        } else {
            let inv_d = 1.0 / dir[i];
            let mut t1 = (bb_min[i] - origin[i]) * inv_d;
            let mut t2 = (bb_max[i] - origin[i]) * inv_d;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            t_min = t_min.max(t1);
            t_max = t_max.min(t2);
            if t_min > t_max {
                return None;
            }
        }
    }

    if t_min >= 0.0 {
        Some(t_min)
    } else if t_max >= 0.0 {
        Some(t_max)
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Primitives
// ---------------------------------------------------------------------------

/// A triangle defined by three vertices.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { vertices: [a, b, c] }
    }

    /// Face normal (not normalized). Returns the cross product of two edges.
    ///
    /// The magnitude equals twice the triangle's area. Use [`unit_normal`](Self::unit_normal)
    /// for a normalized version.
    #[inline]
    pub fn normal(&self) -> Vec3 {
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        edge1.cross(edge2)
    }

    /// Normalized face normal.
    #[inline]
    pub fn unit_normal(&self) -> Vec3 {
        self.normal().normalize()
    }

    /// Area of the triangle.
    #[inline]
    pub fn area(&self) -> f32 {
        self.normal().length() * 0.5
    }

    /// Centroid (average of the three vertices).
    #[inline]
    pub fn centroid(&self) -> Vec3 {
        (self.vertices[0] + self.vertices[1] + self.vertices[2]) / 3.0
    }
}

/// An infinite line defined by a point and a direction.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Line {
    pub origin: Vec3,
    /// Normalized direction.
    pub direction: Vec3,
}

impl Line {
    /// Create a new line. Direction is normalized automatically.
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Closest point on this infinite line to the given point.
    #[inline]
    pub fn closest_point(&self, point: Vec3) -> Vec3 {
        let v = point - self.origin;
        let t = v.dot(self.direction);
        self.origin + self.direction * t
    }

    /// Distance from a point to this line.
    #[inline]
    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        (point - self.closest_point(point)).length()
    }
}

/// A line segment defined by start and end points.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    pub start: Vec3,
    pub end: Vec3,
}

impl Segment {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        Self { start, end }
    }

    /// Length of the segment.
    #[inline]
    pub fn length(&self) -> f32 {
        (self.end - self.start).length()
    }

    /// Midpoint of the segment.
    #[inline]
    pub fn midpoint(&self) -> Vec3 {
        (self.start + self.end) * 0.5
    }

    /// Normalized direction from start to end.
    #[inline]
    pub fn direction(&self) -> Vec3 {
        (self.end - self.start).normalize()
    }

    /// Closest point on this segment to the given point.
    #[inline]
    pub fn closest_point(&self, point: Vec3) -> Vec3 {
        let ab = self.end - self.start;
        let len_sq = ab.dot(ab);
        if len_sq < 1e-12 {
            return self.start; // Degenerate segment
        }
        let t = ((point - self.start).dot(ab) / len_sq).clamp(0.0, 1.0);
        self.start + ab * t
    }

    /// Distance from a point to this segment.
    #[inline]
    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        (point - self.closest_point(point)).length()
    }
}

/// A view frustum defined by six planes (near, far, left, right, top, bottom).
///
/// The planes' normals point **inward** — a point is inside the frustum if it is
/// on the positive side of all six planes.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    /// Extract frustum planes from a view-projection matrix.
    ///
    /// Uses the Gribb/Hartmann method. Planes are normalized.
    pub fn from_view_projection(vp: glam::Mat4) -> Self {
        let r = vp.to_cols_array_2d();
        // Row-based extraction (transposed column-major)
        let row = |i: usize| -> [f32; 4] {
            [r[0][i], r[1][i], r[2][i], r[3][i]]
        };
        let r0 = row(0);
        let r1 = row(1);
        let r2 = row(2);
        let r3 = row(3);

        let make_plane = |a: f32, b: f32, c: f32, d: f32| -> Plane {
            let len = (a * a + b * b + c * c).sqrt();
            Plane {
                normal: Vec3::new(a / len, b / len, c / len),
                distance: -d / len,
            }
        };

        let planes = [
            // Near:   row3 + row2
            make_plane(r3[0] + r2[0], r3[1] + r2[1], r3[2] + r2[2], r3[3] + r2[3]),
            // Far:    row3 - row2
            make_plane(r3[0] - r2[0], r3[1] - r2[1], r3[2] - r2[2], r3[3] - r2[3]),
            // Left:   row3 + row0
            make_plane(r3[0] + r0[0], r3[1] + r0[1], r3[2] + r0[2], r3[3] + r0[3]),
            // Right:  row3 - row0
            make_plane(r3[0] - r0[0], r3[1] - r0[1], r3[2] - r0[2], r3[3] - r0[3]),
            // Top:    row3 - row1
            make_plane(r3[0] - r1[0], r3[1] - r1[1], r3[2] - r1[2], r3[3] - r1[3]),
            // Bottom: row3 + row1
            make_plane(r3[0] + r1[0], r3[1] + r1[1], r3[2] + r1[2], r3[3] + r1[3]),
        ];

        Self { planes }
    }

    /// Check whether a point is inside the frustum.
    #[inline]
    pub fn contains_point(&self, point: Vec3) -> bool {
        self.planes.iter().all(|p| p.signed_distance(point) >= 0.0)
    }

    /// Conservative check whether an AABB intersects the frustum.
    ///
    /// Returns `false` only if the AABB is fully outside at least one plane.
    #[inline]
    pub fn contains_aabb(&self, aabb: &Aabb) -> bool {
        for plane in &self.planes {
            // Find the corner most aligned with the plane normal (P-vertex)
            let p = Vec3::new(
                if plane.normal.x >= 0.0 { aabb.max.x } else { aabb.min.x },
                if plane.normal.y >= 0.0 { aabb.max.y } else { aabb.min.y },
                if plane.normal.z >= 0.0 { aabb.max.z } else { aabb.min.z },
            );
            if plane.signed_distance(p) < 0.0 {
                return false;
            }
        }
        true
    }
}

// ---------------------------------------------------------------------------
// Intersection / overlap functions
// ---------------------------------------------------------------------------

/// Ray-triangle intersection using the Möller–Trumbore algorithm.
///
/// Returns the `t` parameter if the ray hits the triangle (only `t >= 0`).
#[inline]
pub fn ray_triangle(ray: &Ray, tri: &Triangle) -> Option<f32> {
    let edge1 = tri.vertices[1] - tri.vertices[0];
    let edge2 = tri.vertices[2] - tri.vertices[0];
    let h = ray.direction.cross(edge2);
    let a = edge1.dot(h);

    if a.abs() < 1e-8 {
        return None; // Ray parallel to triangle
    }

    let f = 1.0 / a;
    let s = ray.origin - tri.vertices[0];
    let u = f * s.dot(h);

    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let q = s.cross(edge1);
    let v = f * ray.direction.dot(q);

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = f * edge2.dot(q);
    if t >= 0.0 { Some(t) } else { None }
}

/// Check whether two AABBs overlap.
#[inline]
pub fn aabb_aabb(a: &Aabb, b: &Aabb) -> bool {
    a.min.cmple(b.max).all() && b.min.cmple(a.max).all()
}

/// Check whether two spheres overlap.
#[inline]
pub fn sphere_sphere(a: &Sphere, b: &Sphere) -> bool {
    let r = a.radius + b.radius;
    (a.center - b.center).length_squared() <= r * r
}

/// Intersection of two planes. Returns the line of intersection, or `None` if parallel.
pub fn plane_plane(a: &Plane, b: &Plane) -> Option<Line> {
    let dir = a.normal.cross(b.normal);
    let len_sq = dir.dot(dir);
    if len_sq < 1e-12 {
        return None; // Planes are parallel
    }
    // Find a point on the intersection line
    let point = (dir.cross(b.normal) * a.distance + a.normal.cross(dir) * b.distance) / len_sq;
    Some(Line {
        origin: point,
        direction: dir.normalize(),
    })
}

// ---------------------------------------------------------------------------
// Closest-point functions
// ---------------------------------------------------------------------------

/// Closest point on a ray to a given point (clamped to `t >= 0`).
#[inline]
pub fn closest_point_on_ray(ray: &Ray, point: Vec3) -> Vec3 {
    let t = (point - ray.origin).dot(ray.direction).max(0.0);
    ray.origin + ray.direction * t
}

/// Closest point on a plane to a given point.
#[inline]
pub fn closest_point_on_plane(plane: &Plane, point: Vec3) -> Vec3 {
    point - plane.normal * plane.signed_distance(point)
}

/// Closest point on a sphere's surface to a given point.
///
/// If the point is at the sphere's center, returns the point offset by the radius along +X.
#[inline]
pub fn closest_point_on_sphere(sphere: &Sphere, point: Vec3) -> Vec3 {
    let dir = point - sphere.center;
    let len = dir.length();
    if len < 1e-8 {
        return sphere.center + Vec3::new(sphere.radius, 0.0, 0.0);
    }
    sphere.center + dir * (sphere.radius / len)
}

/// Closest point on an AABB's surface or interior to a given point.
#[inline]
pub fn closest_point_on_aabb(aabb: &Aabb, point: Vec3) -> Vec3 {
    point.clamp(aabb.min, aabb.max)
}

// ---------------------------------------------------------------------------
// BVH (Bounding Volume Hierarchy)
// ---------------------------------------------------------------------------

/// A node in a BVH tree.
#[derive(Debug, Clone)]
enum BvhNode {
    Leaf {
        bounds: Aabb,
        index: usize,
    },
    Internal {
        bounds: Aabb,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

/// A Bounding Volume Hierarchy for fast ray-AABB broadphase queries.
///
/// Stores a tree of AABBs built from a list of primitives. Each primitive
/// is represented by its AABB and an index into the caller's primitive array.
#[derive(Debug, Clone)]
pub struct Bvh {
    root: Option<BvhNode>,
    len: usize,
}

impl Bvh {
    /// Build a BVH from a list of (aabb, index) pairs using midpoint splitting.
    pub fn build(items: &mut [(Aabb, usize)]) -> Self {
        let len = items.len();
        if items.is_empty() {
            return Self { root: None, len: 0 };
        }
        let root = Self::build_recursive(items);
        Self { root: Some(root), len }
    }

    fn build_recursive(items: &mut [(Aabb, usize)]) -> BvhNode {
        if items.len() == 1 {
            return BvhNode::Leaf {
                bounds: items[0].0,
                index: items[0].1,
            };
        }

        // Compute bounding box of all items
        let mut bounds = items[0].0;
        for item in items.iter().skip(1) {
            bounds = bounds.merge(&item.0);
        }

        // Split along the longest axis at the midpoint
        let size = bounds.size();
        let axis = if size.x >= size.y && size.x >= size.z {
            0
        } else if size.y >= size.z {
            1
        } else {
            2
        };

        let mid = match axis {
            0 => (bounds.min.x + bounds.max.x) * 0.5,
            1 => (bounds.min.y + bounds.max.y) * 0.5,
            _ => (bounds.min.z + bounds.max.z) * 0.5,
        };

        // Partition items around the midpoint
        let center_axis = |aabb: &Aabb| -> f32 {
            let c = aabb.center();
            match axis {
                0 => c.x,
                1 => c.y,
                _ => c.z,
            }
        };

        items.sort_unstable_by(|a, b| {
            center_axis(&a.0).partial_cmp(&center_axis(&b.0)).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Find split point: first item whose center is >= midpoint
        let mut split = items.partition_point(|item| center_axis(&item.0) < mid);
        // Ensure at least one item on each side
        if split == 0 {
            split = 1;
        } else if split == items.len() {
            split = items.len() - 1;
        }

        let (left_items, right_items) = items.split_at_mut(split);
        let left = Self::build_recursive(left_items);
        let right = Self::build_recursive(right_items);

        BvhNode::Internal {
            bounds,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Number of primitives in the BVH.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the BVH is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Query all primitive indices whose AABBs are hit by the ray.
    ///
    /// Returns indices in no particular order. Use the indices to test
    /// against the actual primitives for exact intersection.
    pub fn query_ray(&self, ray: &Ray) -> Vec<usize> {
        let mut results = Vec::new();
        if let Some(ref root) = self.root {
            Self::query_ray_recursive(root, ray, &mut results);
        }
        results
    }

    fn query_ray_recursive(node: &BvhNode, ray: &Ray, results: &mut Vec<usize>) {
        match node {
            BvhNode::Leaf { bounds, index } => {
                if ray_aabb(ray, bounds).is_some() {
                    results.push(*index);
                }
            }
            BvhNode::Internal { bounds, left, right } => {
                if ray_aabb(ray, bounds).is_some() {
                    Self::query_ray_recursive(left, ray, results);
                    Self::query_ray_recursive(right, ray, results);
                }
            }
        }
    }

    /// Query all primitive indices whose AABBs overlap the given AABB.
    pub fn query_aabb(&self, query: &Aabb) -> Vec<usize> {
        let mut results = Vec::new();
        if let Some(ref root) = self.root {
            Self::query_aabb_recursive(root, query, &mut results);
        }
        results
    }

    fn query_aabb_recursive(node: &BvhNode, query: &Aabb, results: &mut Vec<usize>) {
        match node {
            BvhNode::Leaf { bounds, index } => {
                if aabb_aabb(bounds, query) {
                    results.push(*index);
                }
            }
            BvhNode::Internal { bounds, left, right } => {
                if aabb_aabb(bounds, query) {
                    Self::query_aabb_recursive(left, query, results);
                    Self::query_aabb_recursive(right, query, results);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// K-d tree
// ---------------------------------------------------------------------------

/// A node in a k-d tree.
#[derive(Debug, Clone)]
enum KdNode {
    Leaf {
        point: Vec3,
        index: usize,
    },
    Split {
        axis: usize,
        split_val: f32,
        left: Box<KdNode>,
        right: Box<KdNode>,
    },
}

/// A 3D k-d tree for nearest-neighbor point queries.
#[derive(Debug, Clone)]
pub struct KdTree {
    root: Option<KdNode>,
    len: usize,
}

impl KdTree {
    /// Build a k-d tree from a list of (point, index) pairs.
    pub fn build(items: &mut [(Vec3, usize)]) -> Self {
        let len = items.len();
        if items.is_empty() {
            return Self { root: None, len: 0 };
        }
        let root = Self::build_recursive(items, 0);
        Self { root: Some(root), len }
    }

    fn build_recursive(items: &mut [(Vec3, usize)], depth: usize) -> KdNode {
        if items.len() == 1 {
            return KdNode::Leaf {
                point: items[0].0,
                index: items[0].1,
            };
        }

        let axis = depth % 3;
        items.sort_unstable_by(|a, b| {
            let va = [a.0.x, a.0.y, a.0.z][axis];
            let vb = [b.0.x, b.0.y, b.0.z][axis];
            va.partial_cmp(&vb).unwrap_or(std::cmp::Ordering::Equal)
        });

        let mid = items.len() / 2;
        let split_val = [items[mid].0.x, items[mid].0.y, items[mid].0.z][axis];

        let (left_items, right_items) = items.split_at_mut(mid);
        let left = Self::build_recursive(left_items, depth + 1);
        let right = Self::build_recursive(right_items, depth + 1);

        KdNode::Split {
            axis,
            split_val,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Number of points in the tree.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Find the nearest neighbor to `query`.
    ///
    /// Returns `(index, distance_squared)` of the closest point,
    /// or `None` if the tree is empty.
    pub fn nearest(&self, query: Vec3) -> Option<(usize, f32)> {
        let mut best_idx = 0;
        let mut best_dist_sq = f32::INFINITY;
        if let Some(ref root) = self.root {
            Self::nearest_recursive(root, query, &mut best_idx, &mut best_dist_sq);
        }
        if best_dist_sq.is_finite() {
            Some((best_idx, best_dist_sq))
        } else {
            None
        }
    }

    fn nearest_recursive(
        node: &KdNode,
        query: Vec3,
        best_idx: &mut usize,
        best_dist_sq: &mut f32,
    ) {
        match node {
            KdNode::Leaf { point, index } => {
                let dist_sq = (*point - query).length_squared();
                if dist_sq < *best_dist_sq {
                    *best_dist_sq = dist_sq;
                    *best_idx = *index;
                }
            }
            KdNode::Split { axis, split_val, left, right } => {
                let query_val = [query.x, query.y, query.z][*axis];
                let diff = query_val - split_val;

                // Visit the closer side first
                let (near, far) = if diff < 0.0 {
                    (left.as_ref(), right.as_ref())
                } else {
                    (right.as_ref(), left.as_ref())
                };

                Self::nearest_recursive(near, query, best_idx, best_dist_sq);

                // Only visit the far side if the splitting plane is closer than current best
                if diff * diff < *best_dist_sq {
                    Self::nearest_recursive(far, query, best_idx, best_dist_sq);
                }
            }
        }
    }

    /// Find all points within `radius` of `query`.
    ///
    /// Returns a list of `(index, distance_squared)`.
    pub fn within_radius(&self, query: Vec3, radius: f32) -> Vec<(usize, f32)> {
        let mut results = Vec::new();
        let radius_sq = radius * radius;
        if let Some(ref root) = self.root {
            Self::radius_recursive(root, query, radius_sq, &mut results);
        }
        results
    }

    fn radius_recursive(
        node: &KdNode,
        query: Vec3,
        radius_sq: f32,
        results: &mut Vec<(usize, f32)>,
    ) {
        match node {
            KdNode::Leaf { point, index } => {
                let dist_sq = (*point - query).length_squared();
                if dist_sq <= radius_sq {
                    results.push((*index, dist_sq));
                }
            }
            KdNode::Split { axis, split_val, left, right } => {
                let query_val = [query.x, query.y, query.z][*axis];
                let diff = query_val - split_val;

                let (near, far) = if diff < 0.0 {
                    (left.as_ref(), right.as_ref())
                } else {
                    (right.as_ref(), left.as_ref())
                };

                Self::radius_recursive(near, query, radius_sq, results);

                if diff * diff <= radius_sq {
                    Self::radius_recursive(far, query, radius_sq, results);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-4;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }

    fn vec3_approx_eq(a: Vec3, b: Vec3) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
    }

    #[test]
    fn ray_at_parameter() {
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        assert_eq!(r.at(0.0), Vec3::ZERO);
        assert!(approx_eq(r.at(5.0).x, 5.0));
    }

    #[test]
    fn plane_from_point_normal() {
        let p = Plane::from_point_normal(Vec3::new(0.0, 1.0, 0.0), Vec3::Y);
        assert!(approx_eq(p.distance, 1.0));
        assert!(approx_eq(p.signed_distance(Vec3::new(0.0, 2.0, 0.0)), 1.0));
        assert!(approx_eq(
            p.signed_distance(Vec3::new(0.0, 0.0, 0.0)),
            -1.0
        ));
    }

    #[test]
    fn aabb_contains() {
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(bb.contains(Vec3::splat(0.5)));
        assert!(!bb.contains(Vec3::splat(2.0)));
        assert!(bb.contains(Vec3::ZERO));
    }

    #[test]
    fn aabb_center_and_size() {
        let bb = Aabb::new(Vec3::new(-1.0, -2.0, -3.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(bb.center(), Vec3::ZERO);
        assert_eq!(bb.size(), Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn aabb_merge() {
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let b = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(0.5, 0.5, 0.5));
        let merged = a.merge(&b);
        assert_eq!(merged.min, Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(merged.max, Vec3::ONE);
    }

    #[test]
    fn sphere_contains() {
        let s = Sphere::new(Vec3::ZERO, 1.0);
        assert!(s.contains_point(Vec3::ZERO));
        assert!(s.contains_point(Vec3::new(1.0, 0.0, 0.0)));
        assert!(!s.contains_point(Vec3::new(1.1, 0.0, 0.0)));
    }

    #[test]
    fn ray_plane_intersection() {
        let r = Ray::new(Vec3::ZERO, Vec3::Y);
        let p = Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
        let t = ray_plane(&r, &p).unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn ray_plane_parallel_no_hit() {
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        let p = Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
        assert!(ray_plane(&r, &p).is_none());
    }

    #[test]
    fn ray_sphere_hit() {
        let r = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::Z);
        let s = Sphere::new(Vec3::ZERO, 1.0);
        let t = ray_sphere(&r, &s).unwrap();
        assert!(approx_eq(t, 4.0));
    }

    #[test]
    fn ray_sphere_miss() {
        let r = Ray::new(Vec3::new(0.0, 5.0, -5.0), Vec3::Z);
        let s = Sphere::new(Vec3::ZERO, 1.0);
        assert!(ray_sphere(&r, &s).is_none());
    }

    #[test]
    fn ray_aabb_hit() {
        let r = Ray::new(Vec3::new(0.5, 0.5, -5.0), Vec3::Z);
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let t = ray_aabb(&r, &bb).unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn ray_aabb_miss() {
        let r = Ray::new(Vec3::new(5.0, 5.0, -5.0), Vec3::Z);
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(ray_aabb(&r, &bb).is_none());
    }

    #[test]
    fn ray_inside_sphere() {
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        let s = Sphere::new(Vec3::ZERO, 10.0);
        let t = ray_sphere(&r, &s).unwrap();
        assert!(t > 0.0);
        assert!(approx_eq(t, 10.0));
    }

    #[test]
    fn ray_inside_aabb() {
        let r = Ray::new(Vec3::splat(0.5), Vec3::X);
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let t = ray_aabb(&r, &bb).unwrap();
        assert!(t >= 0.0);
    }

    #[test]
    fn aabb_auto_corrects_min_max() {
        let bb = Aabb::new(Vec3::ONE, Vec3::ZERO);
        assert_eq!(bb.min, Vec3::ZERO);
        assert_eq!(bb.max, Vec3::ONE);
    }

    #[test]
    fn ray_normalizes_direction() {
        let r = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, 10.0));
        let len = r.direction.length();
        assert!(approx_eq(len, 1.0));
        assert!(approx_eq(r.direction.z, 1.0));
    }

    #[test]
    fn ray_at_negative_parameter() {
        let r = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::X);
        let p = r.at(-2.0);
        assert!(approx_eq(p.x, -1.0));
    }

    #[test]
    fn plane_signed_distance_on_plane() {
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        assert!(approx_eq(p.signed_distance(Vec3::new(5.0, 0.0, -3.0)), 0.0));
    }

    #[test]
    fn plane_non_axis_normal() {
        let normal = Vec3::new(1.0, 1.0, 0.0);
        let p = Plane::from_point_normal(Vec3::ZERO, normal);
        assert!(approx_eq(p.normal.length(), 1.0));
        assert!(approx_eq(p.distance, 0.0));
    }

    #[test]
    fn aabb_contains_boundary_max() {
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(bb.contains(Vec3::ONE));
    }

    #[test]
    fn aabb_merge_identical() {
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let merged = bb.merge(&bb);
        assert_eq!(merged.min, Vec3::ZERO);
        assert_eq!(merged.max, Vec3::ONE);
    }

    #[test]
    fn aabb_merge_disjoint() {
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let b = Aabb::new(Vec3::splat(5.0), Vec3::splat(6.0));
        let merged = a.merge(&b);
        assert_eq!(merged.min, Vec3::ZERO);
        assert_eq!(merged.max, Vec3::splat(6.0));
    }

    #[test]
    fn sphere_surface_point() {
        let s = Sphere::new(Vec3::ZERO, 5.0);
        assert!(s.contains_point(Vec3::new(5.0, 0.0, 0.0)));
        assert!(s.contains_point(Vec3::new(0.0, -5.0, 0.0)));
    }

    #[test]
    fn sphere_offset_center() {
        let s = Sphere::new(Vec3::new(10.0, 0.0, 0.0), 1.0);
        assert!(s.contains_point(Vec3::new(10.5, 0.0, 0.0)));
        assert!(!s.contains_point(Vec3::ZERO));
    }

    #[test]
    fn ray_plane_behind_origin() {
        let r = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::Z);
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Z);
        assert!(ray_plane(&r, &p).is_none());
    }

    #[test]
    fn ray_sphere_tangent() {
        let s = Sphere::new(Vec3::ZERO, 1.0);
        let r = Ray::new(Vec3::new(0.0, 1.0, -5.0), Vec3::Z);
        let t = ray_sphere(&r, &s);
        assert!(t.is_some());
        assert!(approx_eq(t.unwrap(), 5.0));
    }

    #[test]
    fn ray_sphere_behind_ray() {
        let r = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::Z);
        let s = Sphere::new(Vec3::ZERO, 1.0);
        assert!(ray_sphere(&r, &s).is_none());
    }

    #[test]
    fn ray_aabb_axis_aligned_hit() {
        let r = Ray::new(Vec3::new(-5.0, 0.5, 0.5), Vec3::X);
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let t = ray_aabb(&r, &bb).unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn ray_aabb_parallel_to_slab_inside() {
        let r = Ray::new(Vec3::new(-5.0, 0.5, 0.5), Vec3::X);
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(ray_aabb(&r, &bb).is_some());
    }

    #[test]
    fn ray_aabb_parallel_to_slab_outside() {
        let r = Ray::new(Vec3::new(-5.0, 5.0, 0.5), Vec3::X);
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(ray_aabb(&r, &bb).is_none());
    }

    #[test]
    fn geo_error_display() {
        use crate::GanitError;
        let e = GanitError::Degenerate("zero-length edge".to_string());
        assert_eq!(e.to_string(), "degenerate geometry: zero-length edge");
    }

    #[test]
    fn ray_serde_roundtrip() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::Y);
        let json = serde_json::to_string(&r).unwrap();
        let r2: Ray = serde_json::from_str(&json).unwrap();
        assert_eq!(r, r2);
    }

    #[test]
    fn aabb_serde_roundtrip() {
        let bb = Aabb::new(Vec3::new(-1.0, -2.0, -3.0), Vec3::new(4.0, 5.0, 6.0));
        let json = serde_json::to_string(&bb).unwrap();
        let bb2: Aabb = serde_json::from_str(&json).unwrap();
        assert_eq!(bb, bb2);
    }

    #[test]
    fn sphere_serde_roundtrip() {
        let s = Sphere::new(Vec3::new(1.0, 2.0, 3.0), 5.0);
        let json = serde_json::to_string(&s).unwrap();
        let s2: Sphere = serde_json::from_str(&json).unwrap();
        assert_eq!(s, s2);
    }

    #[test]
    fn aabb_zero_size() {
        let bb = Aabb::new(Vec3::splat(3.0), Vec3::splat(3.0));
        assert_eq!(bb.size(), Vec3::ZERO);
        assert_eq!(bb.center(), Vec3::splat(3.0));
        assert!(bb.contains(Vec3::splat(3.0)));
    }

    #[test]
    fn ray_plane_intersection_at_angle() {
        let r = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, -1.0, 1.0));
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        let t = ray_plane(&r, &p).unwrap();
        let hit = r.at(t);
        assert!(approx_eq(hit.y, 0.0));
    }

    #[test]
    fn ray_sphere_optimized_matches_distance() {
        // Verify half-b optimization gives correct hit distance
        let r = Ray::new(Vec3::new(0.0, 0.0, -10.0), Vec3::Z);
        let s = Sphere::new(Vec3::ZERO, 2.0);
        let t = ray_sphere(&r, &s).unwrap();
        assert!(approx_eq(t, 8.0)); // 10 - 2
        let hit = r.at(t);
        assert!(approx_eq(hit.z, -2.0));
    }

    #[test]
    fn ray_sphere_large_radius() {
        let r = Ray::new(Vec3::new(0.0, 0.0, -1000.0), Vec3::Z);
        let s = Sphere::new(Vec3::ZERO, 500.0);
        let t = ray_sphere(&r, &s).unwrap();
        assert!(approx_eq(t, 500.0));
    }

    #[test]
    fn aabb_contains_just_outside() {
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(!bb.contains(Vec3::new(1.001, 0.5, 0.5)));
        assert!(!bb.contains(Vec3::new(0.5, -0.001, 0.5)));
        assert!(!bb.contains(Vec3::new(0.5, 0.5, 1.001)));
    }

    #[test]
    fn aabb_contains_all_corners() {
        let bb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        // All 8 corners should be contained
        for x in [-1.0, 1.0] {
            for y in [-1.0, 1.0] {
                for z in [-1.0, 1.0] {
                    assert!(bb.contains(Vec3::new(x, y, z)));
                }
            }
        }
    }

    #[test]
    fn ray_aabb_diagonal_ray() {
        // Diagonal ray through AABB
        let r = Ray::new(Vec3::new(-5.0, -5.0, -5.0), Vec3::new(1.0, 1.0, 1.0));
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        assert!(ray_aabb(&r, &bb).is_some());
    }

    #[test]
    fn plane_signed_distance_both_sides() {
        let p = Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
        assert!(p.signed_distance(Vec3::new(0.0, 10.0, 0.0)) > 0.0);
        assert!(p.signed_distance(Vec3::new(0.0, 0.0, 0.0)) < 0.0);
        assert!(approx_eq(p.signed_distance(Vec3::new(0.0, 5.0, 0.0)), 0.0));
    }

    // -----------------------------------------------------------------------
    // V0.2 tests
    // -----------------------------------------------------------------------

    // Triangle tests
    #[test]
    fn triangle_normal() {
        let tri = Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y);
        let n = tri.normal();
        assert!(approx_eq(n.z, 1.0)); // CCW in XY plane -> +Z
    }

    #[test]
    fn triangle_area() {
        let tri = Triangle::new(Vec3::ZERO, Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0));
        assert!(approx_eq(tri.area(), 2.0));
    }

    #[test]
    fn triangle_centroid() {
        let tri = Triangle::new(Vec3::ZERO, Vec3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 3.0, 0.0));
        assert!(vec3_approx_eq(tri.centroid(), Vec3::new(1.0, 1.0, 0.0)));
    }

    #[test]
    fn triangle_degenerate_area() {
        // Collinear points -> zero area
        let tri = Triangle::new(Vec3::ZERO, Vec3::X, Vec3::new(2.0, 0.0, 0.0));
        assert!(approx_eq(tri.area(), 0.0));
    }

    // Line tests
    #[test]
    fn line_closest_point() {
        let l = Line::new(Vec3::ZERO, Vec3::X);
        let p = Vec3::new(5.0, 3.0, 0.0);
        let cp = l.closest_point(p);
        assert!(vec3_approx_eq(cp, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn line_distance_to_point() {
        let l = Line::new(Vec3::ZERO, Vec3::X);
        let d = l.distance_to_point(Vec3::new(5.0, 3.0, 4.0));
        assert!(approx_eq(d, 5.0)); // sqrt(9+16) = 5
    }

    #[test]
    fn line_closest_point_behind_origin() {
        // Line is infinite — should work for negative t
        let l = Line::new(Vec3::ZERO, Vec3::X);
        let cp = l.closest_point(Vec3::new(-10.0, 1.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(-10.0, 0.0, 0.0)));
    }

    // Segment tests
    #[test]
    fn segment_length_and_midpoint() {
        let s = Segment::new(Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0));
        assert!(approx_eq(s.length(), 4.0));
        assert!(vec3_approx_eq(s.midpoint(), Vec3::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn segment_closest_point_clamped() {
        let s = Segment::new(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0));
        // Point past the end
        assert!(vec3_approx_eq(s.closest_point(Vec3::new(20.0, 0.0, 0.0)), Vec3::new(10.0, 0.0, 0.0)));
        // Point before the start
        assert!(vec3_approx_eq(s.closest_point(Vec3::new(-5.0, 0.0, 0.0)), Vec3::ZERO));
        // Point alongside
        assert!(vec3_approx_eq(s.closest_point(Vec3::new(5.0, 3.0, 0.0)), Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn segment_distance() {
        let s = Segment::new(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0));
        assert!(approx_eq(s.distance_to_point(Vec3::new(5.0, 3.0, 0.0)), 3.0));
    }

    #[test]
    fn segment_direction_normalized() {
        let s = Segment::new(Vec3::ZERO, Vec3::new(0.0, 0.0, 10.0));
        assert!(vec3_approx_eq(s.direction(), Vec3::Z));
    }

    // Ray-triangle tests
    #[test]
    fn ray_triangle_hit() {
        let tri = Triangle::new(
            Vec3::new(-1.0, -1.0, 5.0),
            Vec3::new(1.0, -1.0, 5.0),
            Vec3::new(0.0, 1.0, 5.0),
        );
        let r = Ray::new(Vec3::ZERO, Vec3::Z);
        let t = ray_triangle(&r, &tri).unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn ray_triangle_miss() {
        let tri = Triangle::new(
            Vec3::new(-1.0, -1.0, 5.0),
            Vec3::new(1.0, -1.0, 5.0),
            Vec3::new(0.0, 1.0, 5.0),
        );
        let r = Ray::new(Vec3::new(10.0, 10.0, 0.0), Vec3::Z);
        assert!(ray_triangle(&r, &tri).is_none());
    }

    #[test]
    fn ray_triangle_parallel() {
        // Ray parallel to triangle plane
        let tri = Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y);
        let r = Ray::new(Vec3::new(0.0, 0.0, 1.0), Vec3::X);
        assert!(ray_triangle(&r, &tri).is_none());
    }

    #[test]
    fn ray_triangle_behind() {
        let tri = Triangle::new(
            Vec3::new(-1.0, -1.0, -5.0),
            Vec3::new(1.0, -1.0, -5.0),
            Vec3::new(0.0, 1.0, -5.0),
        );
        let r = Ray::new(Vec3::ZERO, Vec3::Z); // Points away from triangle
        assert!(ray_triangle(&r, &tri).is_none());
    }

    // AABB-AABB overlap tests
    #[test]
    fn aabb_aabb_overlap() {
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let b = Aabb::new(Vec3::splat(0.5), Vec3::splat(1.5));
        assert!(aabb_aabb(&a, &b));
    }

    #[test]
    fn aabb_aabb_no_overlap() {
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let b = Aabb::new(Vec3::splat(2.0), Vec3::splat(3.0));
        assert!(!aabb_aabb(&a, &b));
    }

    #[test]
    fn aabb_aabb_touching() {
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let b = Aabb::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(2.0, 1.0, 1.0));
        assert!(aabb_aabb(&a, &b)); // Touching edge = overlap
    }

    #[test]
    fn aabb_aabb_contained() {
        let a = Aabb::new(Vec3::ZERO, Vec3::splat(10.0));
        let b = Aabb::new(Vec3::ONE, Vec3::splat(2.0));
        assert!(aabb_aabb(&a, &b));
    }

    // Sphere-sphere overlap tests
    #[test]
    fn sphere_sphere_overlap() {
        let a = Sphere::new(Vec3::ZERO, 1.0);
        let b = Sphere::new(Vec3::new(1.5, 0.0, 0.0), 1.0);
        assert!(sphere_sphere(&a, &b));
    }

    #[test]
    fn sphere_sphere_no_overlap() {
        let a = Sphere::new(Vec3::ZERO, 1.0);
        let b = Sphere::new(Vec3::new(3.0, 0.0, 0.0), 1.0);
        assert!(!sphere_sphere(&a, &b));
    }

    #[test]
    fn sphere_sphere_touching() {
        let a = Sphere::new(Vec3::ZERO, 1.0);
        let b = Sphere::new(Vec3::new(2.0, 0.0, 0.0), 1.0);
        assert!(sphere_sphere(&a, &b)); // Touching = overlap
    }

    // Plane-plane intersection tests
    #[test]
    fn plane_plane_intersection() {
        let a = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        let b = Plane::from_point_normal(Vec3::ZERO, Vec3::X);
        let line = plane_plane(&a, &b).unwrap();
        // Intersection should be along Z axis
        assert!(approx_eq(line.direction.z.abs(), 1.0));
    }

    #[test]
    fn plane_plane_parallel() {
        let a = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        let b = Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
        assert!(plane_plane(&a, &b).is_none());
    }

    // Closest-point tests
    #[test]
    fn closest_on_ray_forward() {
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        let cp = closest_point_on_ray(&r, Vec3::new(5.0, 3.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn closest_on_ray_clamped() {
        // Point behind the ray — should clamp to origin
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        let cp = closest_point_on_ray(&r, Vec3::new(-5.0, 3.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::ZERO));
    }

    #[test]
    fn closest_on_plane() {
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        let cp = closest_point_on_plane(&p, Vec3::new(3.0, 7.0, -2.0));
        assert!(vec3_approx_eq(cp, Vec3::new(3.0, 0.0, -2.0)));
    }

    #[test]
    fn closest_on_sphere_outside() {
        let s = Sphere::new(Vec3::ZERO, 1.0);
        let cp = closest_point_on_sphere(&s, Vec3::new(10.0, 0.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(1.0, 0.0, 0.0)));
    }

    #[test]
    fn closest_on_sphere_inside() {
        let s = Sphere::new(Vec3::ZERO, 10.0);
        let cp = closest_point_on_sphere(&s, Vec3::new(1.0, 0.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(10.0, 0.0, 0.0)));
    }

    #[test]
    fn closest_on_sphere_at_center() {
        let s = Sphere::new(Vec3::ZERO, 5.0);
        let cp = closest_point_on_sphere(&s, Vec3::ZERO);
        assert!(vec3_approx_eq(cp, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn closest_on_aabb_inside() {
        let bb = Aabb::new(Vec3::ZERO, Vec3::splat(10.0));
        let cp = closest_point_on_aabb(&bb, Vec3::new(5.0, 5.0, 5.0));
        assert!(vec3_approx_eq(cp, Vec3::new(5.0, 5.0, 5.0)));
    }

    #[test]
    fn closest_on_aabb_outside() {
        let bb = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let cp = closest_point_on_aabb(&bb, Vec3::new(5.0, 0.5, -3.0));
        assert!(vec3_approx_eq(cp, Vec3::new(1.0, 0.5, 0.0)));
    }

    // Frustum tests
    #[test]
    fn frustum_contains_origin() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        // Point at center of near plane should be inside
        assert!(frustum.contains_point(Vec3::new(0.0, 0.0, -1.0)));
    }

    #[test]
    fn frustum_rejects_behind_camera() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        // Point behind camera should be outside
        assert!(!frustum.contains_point(Vec3::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn frustum_rejects_far_point() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        assert!(!frustum.contains_point(Vec3::new(0.0, 0.0, -200.0)));
    }

    #[test]
    fn frustum_contains_aabb_inside() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        let bb = Aabb::new(Vec3::new(-0.1, -0.1, -2.0), Vec3::new(0.1, 0.1, -1.0));
        assert!(frustum.contains_aabb(&bb));
    }

    #[test]
    fn frustum_rejects_aabb_outside() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        let bb = Aabb::new(Vec3::splat(500.0), Vec3::splat(600.0));
        assert!(!frustum.contains_aabb(&bb));
    }

    // Serde roundtrip for new types
    #[test]
    fn triangle_serde_roundtrip() {
        let tri = Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y);
        let json = serde_json::to_string(&tri).unwrap();
        let tri2: Triangle = serde_json::from_str(&json).unwrap();
        assert_eq!(tri, tri2);
    }

    #[test]
    fn line_serde_roundtrip() {
        let l = Line::new(Vec3::ZERO, Vec3::X);
        let json = serde_json::to_string(&l).unwrap();
        let l2: Line = serde_json::from_str(&json).unwrap();
        assert_eq!(l, l2);
    }

    #[test]
    fn segment_serde_roundtrip() {
        let s = Segment::new(Vec3::ZERO, Vec3::ONE);
        let json = serde_json::to_string(&s).unwrap();
        let s2: Segment = serde_json::from_str(&json).unwrap();
        assert_eq!(s, s2);
    }

    // --- Audit tests ---

    #[test]
    fn triangle_unit_normal() {
        let tri = Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y);
        let n = tri.unit_normal();
        assert!(approx_eq(n.length(), 1.0));
        assert!(approx_eq(n.z, 1.0));
    }

    #[test]
    fn triangle_unit_normal_3d() {
        // Tilted triangle in 3D space
        let tri = Triangle::new(Vec3::ZERO, Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 1.0));
        let n = tri.unit_normal();
        assert!(approx_eq(n.length(), 1.0));
    }

    #[test]
    fn segment_degenerate_zero_length() {
        let s = Segment::new(Vec3::ONE, Vec3::ONE);
        assert!(approx_eq(s.length(), 0.0));
        // Closest point should return the segment point itself
        assert!(vec3_approx_eq(s.closest_point(Vec3::new(5.0, 5.0, 5.0)), Vec3::ONE));
    }

    #[test]
    fn plane_plane_intersection_point_on_both() {
        let a = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        let b = Plane::from_point_normal(Vec3::ZERO, Vec3::X);
        let line = plane_plane(&a, &b).unwrap();
        // The origin should be close to the intersection line
        let cp = line.closest_point(Vec3::ZERO);
        assert!(approx_eq(cp.length(), 0.0));
    }

    #[test]
    fn closest_on_sphere_direction_consistent() {
        let s = Sphere::new(Vec3::ZERO, 5.0);
        // Point along +Y axis -> closest should be along +Y
        let cp = closest_point_on_sphere(&s, Vec3::new(0.0, 100.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(0.0, 5.0, 0.0)));
        // Point along -Z axis -> closest should be along -Z
        let cp2 = closest_point_on_sphere(&s, Vec3::new(0.0, 0.0, -100.0));
        assert!(vec3_approx_eq(cp2, Vec3::new(0.0, 0.0, -5.0)));
    }

    #[test]
    fn frustum_serde_roundtrip() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let f = Frustum::from_view_projection(proj);
        let json = serde_json::to_string(&f).unwrap();
        let f2: Frustum = serde_json::from_str(&json).unwrap();
        assert_eq!(f, f2);
    }

    #[test]
    fn frustum_left_right_rejection() {
        use crate::transforms::projection_perspective;
        let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = Frustum::from_view_projection(proj);
        // Points far to the left/right should be outside
        assert!(!frustum.contains_point(Vec3::new(1000.0, 0.0, -10.0)));
        assert!(!frustum.contains_point(Vec3::new(-1000.0, 0.0, -10.0)));
    }

    #[test]
    fn ray_triangle_edge_hit() {
        // Ray hitting exactly on the edge of the triangle
        let tri = Triangle::new(
            Vec3::new(-1.0, 0.0, 5.0),
            Vec3::new(1.0, 0.0, 5.0),
            Vec3::new(0.0, 2.0, 5.0),
        );
        let r = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::Z);
        // Should hit at the bottom edge (y=0)
        let t = ray_triangle(&r, &tri).unwrap();
        assert!(approx_eq(t, 5.0));
    }

    #[test]
    fn aabb_aabb_single_axis_separation() {
        // Separated only on Z axis
        let a = Aabb::new(Vec3::ZERO, Vec3::ONE);
        let b = Aabb::new(Vec3::new(0.0, 0.0, 2.0), Vec3::new(1.0, 1.0, 3.0));
        assert!(!aabb_aabb(&a, &b));
    }

    #[test]
    fn line_distance_at_origin() {
        let l = Line::new(Vec3::new(0.0, 5.0, 0.0), Vec3::X);
        assert!(approx_eq(l.distance_to_point(Vec3::ZERO), 5.0));
    }

    #[test]
    fn closest_on_plane_already_on_plane() {
        let p = Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        let point = Vec3::new(3.0, 0.0, -7.0);
        let cp = closest_point_on_plane(&p, point);
        assert!(vec3_approx_eq(cp, point));
    }

    #[test]
    fn closest_on_ray_along_direction() {
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        let cp = closest_point_on_ray(&r, Vec3::new(5.0, 0.0, 0.0));
        assert!(vec3_approx_eq(cp, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn sphere_sphere_concentric() {
        let a = Sphere::new(Vec3::ZERO, 1.0);
        let b = Sphere::new(Vec3::ZERO, 0.5);
        assert!(sphere_sphere(&a, &b));
    }

    // --- V0.5a: BVH ---

    #[test]
    fn bvh_empty() {
        let bvh = Bvh::build(&mut []);
        assert!(bvh.is_empty());
        assert_eq!(bvh.len(), 0);
        let r = Ray::new(Vec3::ZERO, Vec3::X);
        assert!(bvh.query_ray(&r).is_empty());
    }

    #[test]
    fn bvh_single() {
        let bb = Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::ONE);
        let mut items = [(bb, 42)];
        let bvh = Bvh::build(&mut items);
        assert_eq!(bvh.len(), 1);

        let r = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::Z);
        let hits = bvh.query_ray(&r);
        assert_eq!(hits, vec![42]);
    }

    #[test]
    fn bvh_ray_query_hits_and_misses() {
        let mut items: Vec<(Aabb, usize)> = (0..10)
            .map(|i| {
                let x = i as f32 * 3.0;
                (Aabb::new(Vec3::new(x, 0.0, 0.0), Vec3::new(x + 1.0, 1.0, 1.0)), i)
            })
            .collect();
        let bvh = Bvh::build(&mut items);
        assert_eq!(bvh.len(), 10);

        // Ray hitting the first box
        let r = Ray::new(Vec3::new(0.5, 0.5, -5.0), Vec3::Z);
        let hits = bvh.query_ray(&r);
        assert!(hits.contains(&0));
        assert!(!hits.contains(&5));

        // Ray missing everything (way above)
        let r_miss = Ray::new(Vec3::new(0.5, 100.0, -5.0), Vec3::Z);
        assert!(bvh.query_ray(&r_miss).is_empty());
    }

    #[test]
    fn bvh_aabb_query() {
        let mut items: Vec<(Aabb, usize)> = (0..5)
            .map(|i| {
                let x = i as f32 * 2.0;
                (Aabb::new(Vec3::new(x, 0.0, 0.0), Vec3::new(x + 1.0, 1.0, 1.0)), i)
            })
            .collect();
        let bvh = Bvh::build(&mut items);

        // Query box overlapping first two items
        let query = Aabb::new(Vec3::new(-0.5, 0.0, 0.0), Vec3::new(2.5, 1.0, 1.0));
        let hits = bvh.query_aabb(&query);
        assert!(hits.contains(&0));
        assert!(hits.contains(&1));
    }

    #[test]
    fn bvh_many_items() {
        let mut items: Vec<(Aabb, usize)> = (0..100)
            .map(|i| {
                let x = (i % 10) as f32;
                let y = (i / 10) as f32;
                (Aabb::new(Vec3::new(x, y, 0.0), Vec3::new(x + 0.5, y + 0.5, 0.5)), i)
            })
            .collect();
        let bvh = Bvh::build(&mut items);
        assert_eq!(bvh.len(), 100);

        let r = Ray::new(Vec3::new(0.25, 0.25, -10.0), Vec3::Z);
        let hits = bvh.query_ray(&r);
        assert!(!hits.is_empty());
    }

    // --- V0.5a: K-d tree ---

    #[test]
    fn kdtree_empty() {
        let tree = KdTree::build(&mut []);
        assert!(tree.is_empty());
        assert!(tree.nearest(Vec3::ZERO).is_none());
    }

    #[test]
    fn kdtree_single() {
        let mut items = [(Vec3::new(5.0, 0.0, 0.0), 0)];
        let tree = KdTree::build(&mut items);
        let (idx, dist_sq) = tree.nearest(Vec3::ZERO).unwrap();
        assert_eq!(idx, 0);
        assert!(approx_eq(dist_sq, 25.0));
    }

    #[test]
    fn kdtree_nearest_basic() {
        let mut items: Vec<(Vec3, usize)> = vec![
            (Vec3::new(0.0, 0.0, 0.0), 0),
            (Vec3::new(10.0, 0.0, 0.0), 1),
            (Vec3::new(5.0, 5.0, 0.0), 2),
        ];
        let tree = KdTree::build(&mut items);

        let (idx, _) = tree.nearest(Vec3::new(0.1, 0.0, 0.0)).unwrap();
        assert_eq!(idx, 0);

        let (idx, _) = tree.nearest(Vec3::new(9.9, 0.0, 0.0)).unwrap();
        assert_eq!(idx, 1);

        let (idx, _) = tree.nearest(Vec3::new(5.0, 4.9, 0.0)).unwrap();
        assert_eq!(idx, 2);
    }

    #[test]
    fn kdtree_within_radius() {
        let mut items: Vec<(Vec3, usize)> = (0..10)
            .map(|i| (Vec3::new(i as f32, 0.0, 0.0), i))
            .collect();
        let tree = KdTree::build(&mut items);

        let results = tree.within_radius(Vec3::new(5.0, 0.0, 0.0), 1.5);
        let indices: Vec<usize> = results.iter().map(|&(idx, _)| idx).collect();
        assert!(indices.contains(&4));
        assert!(indices.contains(&5));
        assert!(indices.contains(&6));
        assert!(!indices.contains(&3));
        assert!(!indices.contains(&7));
    }

    #[test]
    fn kdtree_within_radius_empty() {
        let mut items = [(Vec3::new(100.0, 100.0, 100.0), 0)];
        let tree = KdTree::build(&mut items);
        let results = tree.within_radius(Vec3::ZERO, 1.0);
        assert!(results.is_empty());
    }

    #[test]
    fn kdtree_many_points() {
        let mut items: Vec<(Vec3, usize)> = (0..1000)
            .map(|i| {
                let x = (i % 10) as f32;
                let y = ((i / 10) % 10) as f32;
                let z = (i / 100) as f32;
                (Vec3::new(x, y, z), i)
            })
            .collect();
        let tree = KdTree::build(&mut items);
        assert_eq!(tree.len(), 1000);

        // The point (0,0,0) should have index 0 as nearest
        let (idx, dist_sq) = tree.nearest(Vec3::new(0.01, 0.01, 0.01)).unwrap();
        assert_eq!(idx, 0);
        assert!(dist_sq < 0.01);
    }

    #[test]
    fn kdtree_nearest_exact_match() {
        let mut items: Vec<(Vec3, usize)> = vec![
            (Vec3::new(1.0, 2.0, 3.0), 7),
            (Vec3::new(4.0, 5.0, 6.0), 8),
        ];
        let tree = KdTree::build(&mut items);
        let (idx, dist_sq) = tree.nearest(Vec3::new(1.0, 2.0, 3.0)).unwrap();
        assert_eq!(idx, 7);
        assert!(approx_eq(dist_sq, 0.0));
    }
}
