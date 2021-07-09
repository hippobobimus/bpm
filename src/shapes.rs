use nalgebra::{
    vector,
    Vector2,
};

#[derive(Debug)]
pub enum Shape {
    Aabb(Aabb),
    Circle(Circle),
    Plane(Plane),
}

/// Shapes must be able to determine the closest point on their area to a given target and the
/// absolute distance between those points.
pub trait CollidableShape: std::fmt::Debug {
    /// Returns the closest point on the shape to the given target point.
    fn closest_point_to(&self, centre: &Vector2<f64>, target: &Vector2<f64>) -> Vector2<f64>;

    /// Returns the distance from the closest point on the shape to the given target point.
    fn shortest_dist_to(&self, centre: &Vector2<f64>, point: &Vector2<f64>) -> f64;
}

/// An axis-aligned bounding box. It stores the half-width extents of the bounding box in the x and
/// y direction, but does not directly store the box's position within the coordinate system.
#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    extents: Vector2<f64>, // half-width extent in x and y axes.
}

impl Aabb {
    /// Creates a new axis-aligned bounding box with the given extents (half-widths) in the x and y
    /// directions.
    pub fn new(extent_x: f64, extent_y: f64) -> Self {
        Self {
            extents: vector![extent_x, extent_y],
        }
    }

    /// Returns the extents (half widths) of the aabb as an x, y vector.
    pub fn extents(&self) -> &Vector2<f64> {
        &self.extents
    }

    /// Calculates and returns the minimum vertex of the aabb.
    pub fn min(&self, centre: &Vector2<f64>) -> Vector2<f64> {
        centre - self.extents
    }

    /// Calculates and returns the maximum vertex of the aabb.
    pub fn max(&self, centre: &Vector2<f64>) -> Vector2<f64> {
        centre + self.extents
    }
}

impl CollidableShape for Aabb {
    /// Calculates and returns the closest point on the aabb centred at the given position to the
    /// given target point. The calculation is made by clamping the target to the min and max
    /// vertices of the aabb.
    fn closest_point_to(&self, centre: &Vector2<f64>, target: &Vector2<f64>) -> Vector2<f64> {
        let min = self.min(centre);
        let max = self.max(centre);

        vector![
            nalgebra::clamp(target.x, min.x, max.x),
            nalgebra::clamp(target.y, min.y, max.y)
        ]
    }

    /// Calculates and returns the shortest distance between the aabb, centred at the given
    /// position, and the target point.
    fn shortest_dist_to(&self, centre: &Vector2<f64>, target: &Vector2<f64>) -> f64 {
        (target - self.closest_point_to(centre, target)).magnitude()
    }
}

/// A circle described by its radius only. The centre position is not stored directly.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    radius: f64,
}

impl Circle {
    /// Creates a new circle with the given radius.
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }

    /// Returns the radius of the circle.
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl CollidableShape for Circle {
    /// Calculates and returns the closest point on the circle centred at the given position to the
    /// given target point. The calculation is made by taking the normalised vector between the
    /// circle's centre and the target point and scaling it by the circle's radius.
    fn closest_point_to(&self, centre: &Vector2<f64>, target: &Vector2<f64>) -> Vector2<f64> {
        (target - centre).normalize() * self.radius()
    }

    /// Calculates and returns the shortest distance between the circle, centred at the given
    /// position, and the target point.
    fn shortest_dist_to(&self, centre: &Vector2<f64>, target: &Vector2<f64>) -> f64 {
        (target - centre).magnitude() - self.radius()
    }
}

/// A straight 2D line with a start and end point.
#[derive(Debug)]
pub struct Line {
    start: Vector2<f64>,
    end: Vector2<f64>,
}

impl Line {
    /// Creates a new line with the given start and end points, translated such that its centre
    /// point is at (0, 0).
    pub fn new(start: Vector2<f64>, end: Vector2<f64>) -> Self {
        let centre = (end - start ) / 2.0;
        Self {
            start: start - centre,
            end: end - centre,
        }
    }

    /// Returns a reference to the start point of the line.
    pub fn start(&self) -> &Vector2<f64> {
        &self.start
    }

    /// Returns a reference to the end point of the line.
    pub fn end(&self) -> &Vector2<f64> {
        &self.end
    }
}

/// An infinite plane in a 2D coordinate system, described by the plane normal vector. Its
/// position, described by any point on the plane, is not stored directly.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Plane {
    normal: Vector2<f64>,
}

impl Plane {
    /// Creates a new infinite plane with the given vector normal.
    pub fn new(normal: Vector2<f64>) -> Self {
        Self { normal }
    }

    /// Returns a reference to the vector normal of the plane.
    pub fn normal(&self) -> &Vector2<f64> {
        &self.normal
    }
}

impl CollidableShape for Plane {
    /// Calculates and returns the closest point on the plane, located by the given point on the
    /// plane, to the given target point.
    fn closest_point_to(&self, plane_position: &Vector2<f64>, target: &Vector2<f64>) -> Vector2<f64> {
        // Use plane equation n.(X - P) = 0 where P is the location of the plane and X is any point
        // on the plane with R = Q - tn where Q is the target and R is the closest point on the
        // plane. Substitute R for X.
        let t = self.normal.dot(&(target - plane_position));  // assume unit normal so n.n = 1.

        target - t * self.normal
    }

    /// Calculates and returns the shortest distance between the plane, located by the given
    /// position, and the target point.
    fn shortest_dist_to(&self, plane_position: &Vector2<f64>, target: &Vector2<f64>) -> f64 {
        self.normal.dot(&(target - plane_position))
    }
}

#[derive(Debug)]
pub struct Polygon {
    vertices: Vec<Vector2<f64>>,
    aabb: Aabb,
}

impl Polygon {
    /// Takes a series of vertices describing the perimeter of a concave polygon, either in
    /// clockwise or anti-clockwise order. The vertices are translated such that the bounding box
    /// enclosing the polygon is centred at (0, 0).
    pub fn new(vertices: &Vec<Vector2<f64>>) -> Self {
        if vertices.len() < 3 {
            panic!("Tried to instantiate a polygon with fewer than 3 vertices!");
        }

        // Find bounding box dimensions.
        // Ok to unwrap here as we have already checked vertices vec is not empty.
        let min_x = vertices.iter()
            .min_by(|a, b| a.x.partial_cmp(&b.x).expect("Float comparison impossible")).unwrap().x;
        let max_x = vertices.iter()
            .max_by(|a, b| a.x.partial_cmp(&b.x).expect("Float comparison impossible")).unwrap().x;
        let min_y = vertices.iter()
            .min_by(|a, b| a.y.partial_cmp(&b.y).expect("Float comparison impossible")).unwrap().y;
        let max_y = vertices.iter()
            .max_by(|a, b| a.y.partial_cmp(&b.y).expect("Float comparison impossible")).unwrap().y;

        let extents = vector![(max_x - min_x) / 2.0, (max_y - min_y) / 2.0];

        let aabb = Aabb::new(extents.x, extents.y);

        // Calculate the current centre of the bounding box and use this to translate the polygon
        // vertices such that their bounding box is centred at (0, 0).
        let centre = vector![max_x - min_x, max_y - min_y];

        let vertices = vertices.iter().map(|v| v - centre).collect();

        Self { vertices, aabb }
    }

    pub fn vertices(&self) -> &Vec<Vector2<f64>> {
        &self.vertices
    }
}

// Intersection tests.

/// Evaluates whether two axis-aligned bounding boxes centred at the given positions are
/// intersecting.
pub fn aabbs_are_intersecting(a: &Aabb, apos: &Vector2<f64>,
                              b: &Aabb, bpos: &Vector2<f64>) -> bool {
    // Distance between centre points of boxes.
    let d = (apos - bpos).abs();

    // Combined extents of boxes.
    let e = a.extents() + b.extents();

    // Evaluate intersection in x and y axes.
    if d.x > e.x || d.y > e.y {
        return false;
    }

    true
}

/// Evaluates whether two circles centred at the given positions are intersecting.
pub fn circles_are_intersecting(c1: &Circle, c1_pos: &Vector2<f64>, c2: &Circle, c2_pos: &Vector2<f64>) -> Option<(Vector2<f64>, Vector2<f64>)> {
    // circles are deemed to be in collision with each other if the distance between their centre
    // points is less than or equal to the sum of their radii.
    if (c2_pos - c1_pos).magnitude() <= c1.radius() + c2.radius() {
        let contact_point = c1.closest_point_to(c1_pos, c2_pos);
        let normal = (c2_pos - c1_pos).normalize();

        return Some((contact_point, normal));
    }
    None
}

/// Evaluates whether the circle centred at a given position and the axis-aligned bounding box
/// centred at another given position intersect.
/// If an intersection is found the contact point and contact normal are returned wrapped in an
/// option, otherwise None is returned.
pub fn circle_aabb_are_intersecting(c: &Circle, c_pos: &Vector2<f64>, a: &Aabb, a_pos: &Vector2<f64>) -> Option<Vector2<f64>> {
    // circle centre to Aabb.
    let dist = a.shortest_dist_to(a_pos, c_pos);  // known that circle has a centre.

    // closest point on Aabb to circle centre.
    let p = a.closest_point_to(a_pos, c_pos);

    if dist <= c.radius {
        return Some(p);
    }
    None
}

/// Evaluates whether the circle centred at the given position and the plane containing the given
/// point intersect.
pub fn circle_plane_are_intersecting(c: &Circle, c_pos: &Vector2<f64>, p: &Plane, p_pos: &Vector2<f64>) -> Option<Vector2<f64>> {
    // evaluate the plane equation for the circle centre.
    let dist = p.shortest_dist_to(p_pos, c_pos);

    // consider the negative half-space behind the plane to be solid.
    if dist <= c.radius {
        return Some(p.normal);
    }
    None
}

/// Evaluates whether the axis-aligned bounding box centred at the given position and the plane
/// containing the given point intersect.
pub fn aabb_plane_are_intersecting(a: &Aabb, a_pos: &Vector2<f64>, p: &Plane, p_pos: &Vector2<f64>) -> bool {
    // Test separating axis that intersects aabb centre and is parallel to plane normal;
    // L(t) = a.centre + t * p.normal.

    // Calculate projection radius of aabb onto L.
    let r = a.extents.x * p.normal().x.abs() + a.extents.y * p.normal().y.abs();

    // distance of aabb centre from plane.
    let dist = p.shortest_dist_to(p_pos, a_pos);

    // consider the negative half-space behind the plane to be solid.
    dist <= r
}

///// TODO possible generic test?
//pub fn are_intersecting(a: &Shape, a_pos: &Vector2<f64>, b: &Shape, b_pos: &Vector2<f64>) -> Option<(Vector2<f64>, Vector2<f64>)> {
//    match (a, b) {
//        (Shape::Aabb(aabb_1), Shape::Aabb(aabb_2)) => {
//            aabbs_are_intersecting(aabb_1, a_pos, aabb_2, b_pos)
//        },
//        (Shape::Circle(c1), Shape::Circle(c2)) => {
//            circles_are_intersecting(c1, a_pos, c2, b_pos)
//        },
//        (Shape::Circle(c), Shape::Plane(p)) => {
//            circle_plane_are_intersecting(c, a_pos, p, b_pos)
//        },
//        (Shape::Plane(p), Shape::Circle(c)) => {
//            circle_plane_are_intersecting(c, b_pos, p, a_pos)
//        },
//        (Shape::Circle(c), Shape::Aabb(aabb)) => {
//            circle_aabb_are_intersecting(c, a_pos, aabb, b_pos)
//        },
//        (Shape::Aabb(aabb), Shape::Circle(c)) => {
//            circle_aabb_are_intersecting(c, b_pos, aabb, a_pos)
//        },
//        (Shape::Aabb(aabb), Shape::Plane(p)) => {
//            aabb_plane_are_intersecting(aabb, a_pos, p, b_pos)
//        },
//        (Shape::Plane(p), Shape::Aabb(aabb)) => {
//            aabb_plane_are_intersecting(aabb, b_pos, p, a_pos)
//        },
//        _ => panic!("This intersection test is not supported!"),
//    }
//}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_intersection_aabbs() {
        let aabb1 = Aabb::new(50.0, 40.0);
        let aabb2 = Aabb::new(9.0, 32.0);
        let aabb1_pos = vector![0.0, 0.0];
        let mut aabb2_pos = vector![58.0, -70.0];

        assert!(aabbs_are_intersecting(&aabb1, &aabb1_pos, &aabb2, &aabb2_pos));

        // no overlap in either dimension.
        aabb2_pos = vector![59.1, 72.01];
        assert!(!aabbs_are_intersecting(&aabb1, &aabb1_pos, &aabb2, &aabb2_pos));

        // overlap in x dimension only.
        aabb2_pos = vector![58.9, -72.01];
        assert!(!aabbs_are_intersecting(&aabb1, &aabb1_pos, &aabb2, &aabb2_pos));

        // overlap in y dimension only.
        aabb2_pos = vector![-59.1, 71.99];
        assert!(!aabbs_are_intersecting(&aabb1, &aabb1_pos, &aabb2, &aabb2_pos));
    }

    #[test]
    fn test_intersection_circles() {
        let r1 = 1.0;
        let r2 = 3.2;
        let c1 = Circle::new(r1);
        let c2 = Circle::new(r2);
        let c1_pos = vector![10.0, -1.2];

        // test clear overlap.
        let mut c2_pos = c1_pos;
        assert!(circles_are_intersecting(&c1, &c1_pos, &c2, &c2_pos).is_some());

        // test intersection where circumferences are just touching.
        let dy = r1 + r2 - 0.5;
        let dx = ((r1 + r2).powi(2) - dy.powi(2)).sqrt();
        c2_pos = vector![c1_pos.x + dx, c1_pos.y + dy];

        assert!(circles_are_intersecting(&c1, &c1_pos, &c2, &c2_pos).is_some());

        // test not intersecting.
        c2_pos = vector![c1_pos.x + dx + 0.1, c1_pos.y + dy];
        assert!(circles_are_intersecting(&c1, &c1_pos, &c2, &c2_pos).is_none());

        c2_pos = vector![c1_pos.x + dx, c1_pos.y + dy + 0.1];
        assert!(circles_are_intersecting(&c1, &c1_pos, &c2, &c2_pos).is_none());
    }

    #[test]
    fn test_intersection_circle_aabb() {
        let r = 10.0;
        let c = Circle::new(r);

        let aabb = Aabb::new(50.0, 40.0);
        let aabb_pos = vector![0.0, 0.0];

        // test clear overlap.
        let mut c_pos = vector![-10.0, -40.2];
        assert!(circle_aabb_are_intersecting(&c, &c_pos, &aabb, &aabb_pos).is_some());

        // test just touching.
        c_pos = vector![60.0, 20.3];
        assert!(circle_aabb_are_intersecting(&c, &c_pos, &aabb, &aabb_pos).is_some());

        // test not intersecting.
        c_pos = vector![60.001, 20.3];
        assert!(circle_aabb_are_intersecting(&c, &c_pos, &aabb, &aabb_pos).is_none());
    }

    #[test]
    fn test_intersection_circle_plane() {
        let r = 10.0;
        let c = Circle::new(r);

        let n = vector![0.0, 1.0];
        let p = Plane::new(n);
        let p_pos = vector![0.0, 0.0];

        // test clear overlap.
        let mut c_pos = vector![-20.0, 0.5];
        assert!(circle_plane_are_intersecting(&c, &c_pos, &p, &p_pos).is_some());

        // test just touching.
        c_pos = vector![3.2, 10.0];
        assert!(circle_plane_are_intersecting(&c, &c_pos, &p, &p_pos).is_some());

        // test not intersecting.
        c_pos = vector![3.2, 10.01];
        assert!(circle_plane_are_intersecting(&c, &c_pos, &p, &p_pos).is_none());
    }

    #[test]
    fn test_intersection_aabb_plane() {
        let a = Aabb::new(40.0, 50.0);
        let a_pos = vector![0.0, 0.0];

        let n = vector![0.0, 1.0];
        let p = Plane::new(n);

        // test clear overlap.
        let mut p_pos = vector![0.0, -30.0];
        assert!(aabb_plane_are_intersecting(&a, &a_pos, &p, &p_pos));

        // test just touching.
        p_pos = vector![0.0, -50.0];
        assert!(aabb_plane_are_intersecting(&a, &a_pos, &p, &p_pos));

        // test not intersecting.
        p_pos = vector![0.0, -50.01];
        assert!(!aabb_plane_are_intersecting(&a, &a_pos, &p, &p_pos));
    }
}
