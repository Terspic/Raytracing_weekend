use super::{is_campled, random_u32, vec3, Point3, Ray, Scatter, Vec3, PI};
use std::{cmp::Ordering, fmt::Debug, sync::Arc};

pub trait Hit: Send + Sync + Debug {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t1: f64, t2: f64) -> Option<AABB>;
}

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Scatter>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    min: Point3,
    max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, r: &Ray, mut tmin: f64, mut tmax: f64) -> bool {
        let inv_d = 1.0 / r.dir;
        let t1 = (self.min - r.origin) * inv_d;
        let t2 = (self.max - r.origin) * inv_d;

        for i in 0..3 {
            tmin = t1[i].min(t2[i]).max(tmin);
            tmax = t1[i].max(t2[i]).min(tmax);
        }

        tmin <= tmax
    }

    pub fn surrounding_box(b0: &Self, b1: &Self) -> Self {
        let min = vec3(
            b0.min.x.min(b1.min.x),
            b0.min.y.min(b1.min.y),
            b0.min.z.min(b1.min.z),
        );
        let max = vec3(
            b0.max.x.max(b1.max.x),
            b0.max.y.max(b1.max.y),
            b0.max.z.max(b1.max.z),
        );

        Self::new(min, max)
    }
}

type NodeID = usize;

#[derive(Debug, Clone)]
struct BVNode {
    left: Option<NodeID>,
    right: Option<NodeID>,
    bbox: AABB,
    hittable: Option<Arc<dyn Hit>>,
}

#[derive(Debug, Clone)]
pub struct BVTree {
    nodes: Vec<BVNode>,
    root_id: NodeID,
    pub objects_count: usize,
}

impl BVTree {
    pub fn new(mut l: HittableList) -> Self {
        let mut tree = Self {
            nodes: Vec::with_capacity(2 * l.len() - 1),
            root_id: 0,
            objects_count: l.len(),
        };

        tree.root_id = tree.build(&mut l);
        tree
    }

    fn build(&mut self, l: &mut [Arc<dyn Hit>]) -> NodeID {
        let (left, right): (NodeID, NodeID);
        if l.len() == 1 {
            return self.new_leaf(&l[0]);
        } else if l.len() == 2 {
            left = self.new_leaf(&l[0]);
            right = self.new_leaf(&l[1]);
        } else {
            let axis = random_u32(0..3);
            l.sort_by(|a, b| {
                Self::compare_boxes(
                    &a.bounding_box(0.0, 0.0).unwrap(),
                    &b.bounding_box(0.0, 0.0).unwrap(),
                    axis as usize,
                )
            });
            let (left_hits, right_hits) = l.split_at_mut(l.len() / 2);
            left = self.build(left_hits);
            right = self.build(right_hits);
        }

        self.new_node(
            left,
            right,
            AABB::surrounding_box(&self.nodes[left].bbox, &self.nodes[right].bbox),
        )
    }

    fn new_leaf(&mut self, hittable: &Arc<dyn Hit>) -> NodeID {
        self.nodes.push(BVNode {
            left: None,
            right: None,
            bbox: hittable.bounding_box(0.0, 0.0).unwrap(),
            hittable: Some(hittable.clone()),
        });

        self.nodes.len() - 1
    }

    fn new_node(&mut self, left: NodeID, right: NodeID, bbox: AABB) -> NodeID {
        self.nodes.push(BVNode {
            left: Some(left),
            right: Some(right),
            bbox,
            hittable: None,
        });

        self.nodes.len() - 1
    }

    fn compare_boxes(a: &AABB, b: &AABB, axis: usize) -> Ordering {
        a.min[axis].partial_cmp(&b.min[axis]).unwrap()
    }

    fn hit_node(&self, id: NodeID, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let node = self.nodes[id].clone();
        if !node.bbox.hit(&r, t_min, t_max) {
            return None;
        }

        // check if node is a leaf
        if node.left.is_none() && node.right.is_none() {
            return node.hittable.unwrap().hit(r, t_min, t_max);
        }

        let hit_left = match node.left {
            Some(id) => self.hit_node(id, &r, t_min, t_max),
            None => None,
        };
        let hit_right = match node.right {
            Some(id) => self.hit_node(id, &r, t_min, t_max),
            None => None,
        };

        match (hit_left, hit_right) {
            (None, None) => return None,
            (Some(left_rec), None) => return Some(left_rec),
            (None, Some(right_rec)) => return Some(right_rec),
            (Some(left_rec), Some(right_rec)) => {
                if left_rec.t < right_rec.t {
                    return Some(left_rec);
                } else {
                    return Some(right_rec);
                }
            }
        }
    }
}

impl Hit for BVTree {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // ray doesn't hit bbox of the tree
        if !self.nodes[self.root_id].bbox.hit(&r, t_min, t_max) {
            return None;
        }

        // ray hit bbox
        self.hit_node(self.root_id, r, t_min, t_max)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.nodes[self.root_id].bbox)
    }
}

impl std::fmt::Display for BVTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Root ID: {}\n", self.root_id)?;
        for (i, node) in self.nodes.iter().enumerate() {
            write!(
                f,
                "Node {0}: (left = {1:?}, right = {2:?})\n",
                i, node.left, node.right
            )?;
        }

        Ok(())
    }
}

pub type HittableList = Vec<Arc<dyn Hit>>;

impl Hit for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest: f64 = t_max;

        for object in self {
            if let Some(rec) = object.hit(&r, t_min, closest) {
                closest = rec.t;
                tmp_rec = Some(rec)
            }
        }

        tmp_rec
    }

    fn bounding_box(&self, t1: f64, t2: f64) -> Option<AABB> {
        if self.is_empty() {
            return None;
        }

        let mut bbox = AABB::new(Vec3::ZERO, Vec3::ZERO);
        for object in self {
            match object.bounding_box(t1, t2) {
                Some(b) => bbox = AABB::surrounding_box(&b, &bbox),
                None => return None,
            }
        }

        Some(bbox)
    }
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Scatter>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }

    pub fn get_uv(point: &Point3) -> (f64, f64) {
        let theta = -point.y.acos();
        let phi = -point.z.atan2(point.x);

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.dir.squared_norm();
        let half_b = oc.dot(r.dir);
        let c = oc.squared_norm() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !is_campled(root, t_min, t_max) {
            root = (-half_b - sqrtd) / a;
            if !is_campled(root, t_min, t_max) {
                return None;
            }
        }

        let outward_normal = (r.at(root) - self.center) / self.radius;
        let (u, v) = Self::get_uv(&outward_normal);
        let mut rec = HitRecord {
            point: r.at(root),
            normal: vec3(0.0, 0.0, 0.0),
            t: root,
            u,
            v,
            front_face: false,
            mat: self.mat.clone(),
        };

        rec.set_face_normal(&r, outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(AABB::new(
            self.center - self.radius * Vec3::ONE,
            self.center + self.radius * Vec3::ONE,
        ))
    }
}

#[derive(Clone, Debug)]
pub struct MovingSphere {
    pub radius: f64,
    pub mat: Arc<dyn Scatter>,
    centers: (Point3, Point3),
    t1: f64,
    t2: f64,
}

impl MovingSphere {
    pub fn new(
        center: (Point3, Point3),
        radius: f64,
        t1: f64,
        t2: f64,
        mat: Arc<dyn Scatter>,
    ) -> Self {
        Self {
            centers: center,
            radius,
            mat,
            t1,
            t2,
        }
    }

    pub fn center(&self, t: f64) -> Point3 {
        self.centers.0 + ((t - self.t1) / (self.t2 - self.t1)) * (self.centers.1 - self.centers.0)
    }

    pub fn get_uv(point: &Point3) -> (f64, f64) {
        let theta = -point.y.acos();
        let phi = -point.z.atan2(point.x);

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hit for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
        let a = r.dir.squared_norm();
        let half_b = oc.dot(r.dir);
        let c = oc.squared_norm() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !is_campled(root, t_min, t_max) {
            root = (-half_b - sqrtd) / a;
            if !is_campled(root, t_min, t_max) {
                return None;
            }
        }

        let outward_normal = (r.at(root) - self.center(r.time)) / self.radius;
        let (u, v) = Self::get_uv(&outward_normal);
        let mut rec = HitRecord {
            point: r.at(root),
            normal: vec3(0.0, 0.0, 0.0),
            t: root,
            u,
            v,
            front_face: false,
            mat: self.mat.clone(),
        };

        rec.set_face_normal(&r, outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, t1: f64, t2: f64) -> Option<AABB> {
        Some(AABB::surrounding_box(
            &AABB::new(
                self.center(t1) - self.radius * Vec3::ONE,
                self.center(t1) - self.radius * Vec3::ONE,
            ),
            &AABB::new(
                self.center(t2) - self.radius * Vec3::ONE,
                self.center(t2) - self.radius * Vec3::ONE,
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::sync::Arc;

    #[test]
    fn test_build_tree() {
        let mut world = HittableList::new();
        world.push(Arc::new(Sphere::new(
            vec3(-10.0, 0.0, 0.0),
            2.5,
            Arc::new(Lambertian::from_color(Color::new(255, 0, 0, 255))),
        )));
        world.push(Arc::new(Sphere::new(
            vec3(0.0, 0.0, 0.0),
            2.5,
            Arc::new(Lambertian::from_color(Color::new(0, 255, 0, 255))),
        )));
        world.push(Arc::new(Sphere::new(
            vec3(10.0, 0.0, 0.0),
            2.5,
            Arc::new(Lambertian::from_color(Color::new(0, 0, 255, 255))),
        )));

        let tree = BVTree::new(world);
        println!("{}", tree);

        let r = ray(vec3(0.0, -10.0, 0.0), vec3(0.0, 1.0, 0.0), 0.0);
        println!("{:?}", tree.hit(&r, 0.0001, f64::INFINITY));
    }
}
