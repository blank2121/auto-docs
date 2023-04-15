pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn quadratic_roots(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        None // Imaginary roots
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let root1 = (-b + sqrt_discriminant) / (2.0 * a);
        let root2 = (-b - sqrt_discriminant) / (2.0 * a);
        if discriminant == 0.0 {
            Some((root1, root1)) // Single real root
        } else {
            Some((root1, root2)) // Two real roots
        }
    }
}

