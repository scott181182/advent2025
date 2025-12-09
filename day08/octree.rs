use std::cmp::Reverse;

use common::space::vector::Vector3i;
use priority_queue::PriorityQueue;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct BoundingBox {
    pub min: Vector3i,
    pub max: Vector3i,
}
impl BoundingBox {
    pub fn new(min: Vector3i, max: Vector3i) -> Self {
        Self { min, max }
    }

    pub fn midpoint(&self) -> Vector3i {
        (&self.min + &self.max) / 2
    }

    pub fn dist2(&self, point: &Vector3i) -> usize {
        let dx = if point.x < self.min.x {
            self.min.x - point.x
        } else if point.x > self.max.x {
            point.x - self.max.x
        } else {
            0
        };
        let dy = if point.y < self.min.y {
            self.min.y - point.y
        } else if point.y > self.max.y {
            point.y - self.max.y
        } else {
            0
        };
        let dz = if point.z < self.min.z {
            self.min.z - point.z
        } else if point.z > self.max.z {
            point.z - self.max.z
        } else {
            0
        };
        (dx * dx + dy * dy + dz * dz) as usize
    }
}
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum OctreeNode {
    Empty,
    Leaf(Vector3i),
    Internal(Box<[Octree; 8]>),
}
impl OctreeNode {
    pub fn new_internal(bbox: &BoundingBox) -> Self {
        let mid = bbox.midpoint();
        OctreeNode::Internal(Box::new([
            Octree::new(BoundingBox::new(bbox.min.clone(), mid.clone())),
            Octree::new(BoundingBox::new(
                Vector3i::new(bbox.min.x, bbox.min.y, mid.z),
                Vector3i::new(mid.x, mid.y, bbox.max.z),
            )),
            Octree::new(BoundingBox::new(
                Vector3i::new(bbox.min.x, mid.y, bbox.min.z),
                Vector3i::new(mid.x, bbox.max.y, mid.z),
            )),
            Octree::new(BoundingBox::new(
                Vector3i::new(bbox.min.x, mid.y, mid.z),
                Vector3i::new(mid.x, bbox.max.y, bbox.max.z),
            )),
            Octree::new(BoundingBox::new(
                Vector3i::new(mid.x, bbox.min.y, bbox.min.z),
                Vector3i::new(bbox.max.x, mid.y, mid.z),
            )),
            Octree::new(BoundingBox::new(
                Vector3i::new(mid.x, bbox.min.y, mid.z),
                Vector3i::new(bbox.max.x, mid.y, bbox.max.z),
            )),
            Octree::new(BoundingBox::new(
                Vector3i::new(mid.x, mid.y, bbox.min.z),
                Vector3i::new(bbox.max.x, bbox.max.y, mid.z),
            )),
            Octree::new(BoundingBox::new(mid, bbox.max.clone())),
        ]))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Octree {
    bbox: BoundingBox,
    node: OctreeNode,
}
impl Octree {
    pub fn new(bbox: BoundingBox) -> Self {
        Self {
            bbox,
            node: OctreeNode::Empty,
        }
    }
    pub fn from_vec(points: Vec<Vector3i>) -> Self {
        let (min, max) = points.iter().fold(
            (
                Vector3i::new(i64::MAX, i64::MAX, i64::MAX),
                Vector3i::new(i64::MIN, i64::MIN, i64::MIN),
            ),
            |(min_acc, max_acc), p| {
                (
                    Vector3i::new(min_acc.x.min(p.x), min_acc.y.min(p.y), min_acc.z.min(p.z)),
                    Vector3i::new(max_acc.x.max(p.x), max_acc.y.max(p.y), max_acc.z.max(p.z)),
                )
            },
        );

        let mut octree = Octree::new(BoundingBox::new(min, max));
        for point in points {
            octree.insert(point);
        }
        octree
    }

    pub fn insert(&mut self, point: Vector3i) {
        let mid = self.bbox.midpoint();
        match &mut self.node {
            OctreeNode::Internal(children) => {
                let index = ((point.x >= mid.x) as usize) << 2
                    | ((point.y >= mid.y) as usize) << 1
                    | ((point.z >= mid.z) as usize);
                children[index].insert(point);
            }
            OctreeNode::Empty => {
                self.node = OctreeNode::Leaf(point);
            }
            OctreeNode::Leaf(child) => {
                let c = child.clone();
                self.node = OctreeNode::new_internal(&self.bbox);
                self.insert(c);
                self.insert(point);
            }
        }
    }

    pub fn closest_points<'a>(&'a self, point: Vector3i) -> ClosestPoints<'a> {
        ClosestPoints::new(self, point)
    }
}
impl std::hash::Hash for Octree {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bbox.hash(state);
    }
}

pub struct ClosestPoints<'a> {
    point: Vector3i,
    stack: PriorityQueue<&'a OctreeNode, Reverse<usize>>,
}
impl<'a> ClosestPoints<'a> {
    pub fn new(tree: &'a Octree, point: Vector3i) -> Self {
        let mut stack = PriorityQueue::new();
        stack.push(&tree.node, Reverse(tree.bbox.dist2(&point)));

        Self { point, stack }
    }
}
impl<'a> Iterator for ClosestPoints<'a> {
    type Item = (&'a Vector3i, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match node.0 {
                OctreeNode::Empty => continue,
                OctreeNode::Leaf(p) => {
                    if p == &self.point {
                        continue;
                    } else {
                        return Some((p, self.point.dist2(p) as usize));
                    }
                }
                OctreeNode::Internal(children) => {
                    for child in children.iter() {
                        match &child.node {
                            OctreeNode::Empty => continue,
                            OctreeNode::Leaf(p) => {
                                if p == &self.point {
                                    continue;
                                } else {
                                    self.stack
                                        .push(&child.node, Reverse(self.point.dist2(p) as usize));
                                }
                            }
                            n => {
                                self.stack.push(n, Reverse(child.bbox.dist2(&self.point)));
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octree_octant_create_ordering() {
        let node = OctreeNode::new_internal(&BoundingBox::new(
            Vector3i::new(0, 0, 0),
            Vector3i::new(8, 8, 8),
        ));

        if let OctreeNode::Internal(children) = node {
            assert_eq!(children[0].bbox.min, Vector3i::new(0, 0, 0));
            assert_eq!(children[0].bbox.max, Vector3i::new(4, 4, 4));
            assert_eq!(children[1].bbox.min, Vector3i::new(0, 0, 4));
            assert_eq!(children[1].bbox.max, Vector3i::new(4, 4, 8));
            assert_eq!(children[2].bbox.min, Vector3i::new(0, 4, 0));
            assert_eq!(children[2].bbox.max, Vector3i::new(4, 8, 4));
            assert_eq!(children[3].bbox.min, Vector3i::new(0, 4, 4));
            assert_eq!(children[3].bbox.max, Vector3i::new(4, 8, 8));
            assert_eq!(children[4].bbox.min, Vector3i::new(4, 0, 0));
            assert_eq!(children[4].bbox.max, Vector3i::new(8, 4, 4));
            assert_eq!(children[5].bbox.min, Vector3i::new(4, 0, 4));
            assert_eq!(children[5].bbox.max, Vector3i::new(8, 4, 8));
            assert_eq!(children[6].bbox.min, Vector3i::new(4, 4, 0));
            assert_eq!(children[6].bbox.max, Vector3i::new(8, 8, 4));
            assert_eq!(children[7].bbox.min, Vector3i::new(4, 4, 4));
            assert_eq!(children[7].bbox.max, Vector3i::new(8, 8, 8));
        } else {
            panic!("Expected Internal node");
        }
    }
    #[test]
    fn test_octree_octant_insert_ordering() {
        let mut node = Octree::new(BoundingBox::new(
            Vector3i::new(0, 0, 0),
            Vector3i::new(8, 8, 8),
        ));

        for i in 0..8 {
            let x = (i >> 2) * 4 + 2;
            let y = ((i >> 1) & 1) * 4 + 2;
            let z = (i & 1) * 4 + 2;
            node.insert(Vector3i::new(x, y, z));
        }

        let OctreeNode::Internal(children) = node.node else {
            panic!("Expected Internal node");
        };
        for (i, child) in children.into_iter().enumerate() {
            let OctreeNode::Leaf(p) = child.node else {
                panic!("Expected Leaf node");
            };

            let expected_x = (i >> 2) * 4 + 2;
            let expected_y = ((i >> 1) & 1) * 4 + 2;
            let expected_z = (i & 1) * 4 + 2;

            assert_eq!(
                p,
                Vector3i::new(expected_x as i64, expected_y as i64, expected_z as i64)
            );
        }
    }
}
