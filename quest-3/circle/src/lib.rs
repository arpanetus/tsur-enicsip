#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;

        (x * x + y * y).sqrt()
    }
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        return Circle {
            center: Point { x: x, y: y },
            radius: radius,
        };
    }

    pub fn diameter(&self) -> f64 {
        self.radius * self.radius
    }

    pub fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }

    pub fn intersect(&self, other: &Circle) -> bool {
        self.center.distance(&other.center) - self.radius - other.radius <= 0.0
    }
}

#[allow(dead_code)]
fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

#[test]
fn test_new_circle() {
    let circle = Circle::new(500.0, 400.0, 150.0);
    assert!(approx_eq(circle.radius, 150.0));
    assert!(approx_eq(circle.center.x, 500.0));
    assert!(approx_eq(circle.center.y, 400.0));
}

#[test]
fn test_distance() {
    let a = Point { x: 0.0, y: 1.0 };
    let b = Point { x: 0.0, y: 0.0 };
    assert!(approx_eq(a.distance(&b), 1.0));
    let a = Point { x: 1.0, y: 0.0 };
    let b = Point { x: 0.0, y: 0.0 };
    assert!(approx_eq(a.distance(&b), 1.0));
    let a = Point { x: 1.0, y: 1.0 };
    let b = Point { x: 0.0, y: 0.0 };
    assert!(approx_eq(a.distance(&b), f64::sqrt(2.0)));
}

#[test]
fn test_area() {
    let circle = Circle::new(500.0, 400.0, 150.0);
    assert!(approx_eq(circle.area(), 70685.83470577035));
}

#[test]
fn test_intersection() {
    let circle = Circle::new(500.0, 500.0, 150.0);
    let circle1 = Circle::new(80.0, 115.0, 30.0);
    assert!(!circle.intersect(&circle1));
    let circle = Circle::new(100.0, 300.0, 150.0);
    let circle1 = Circle::new(80.0, 115.0, 100.0);
    assert!(circle.intersect(&circle1));
}
