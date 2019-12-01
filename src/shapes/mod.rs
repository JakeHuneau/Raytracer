pub mod hitable;
pub mod sphere;

#[macro_export]
macro_rules! make_sphere {
    ( $m:ident, $x:expr, $y:expr, $r:expr ) => {{
        Box::new(Sphere::new($y, $r, Box::new($m::new($x))))
    }};
}
