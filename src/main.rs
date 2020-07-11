mod matrix;
use matrix::*;

fn cmp_fast_slow_sqrt(num: f32) {
	let fast = matrix::fast_inverse_sqrt(num);
	let slow = 1.0 / (num).sqrt();
	println!("1 / sqrt({}) = fast: {}, slow(but accurate): {}", num, fast, slow);
	println!("Difference: {}", fast - slow);
}

fn test_normalize(vec: Vec2d) {
	let normalized = vec.normalized();
	println!("Vector: {}, normalized: {}", vec, normalized);
	println!("Length: {}", normalized.mag());
}

fn main() {
    println!("Comparing fast and slow inverse square root");
	cmp_fast_slow_sqrt(0.1);
	cmp_fast_slow_sqrt(6.0);
	cmp_fast_slow_sqrt(3.0);
	cmp_fast_slow_sqrt(30.0);

	println!();
	println!("Normalizing vectors");
	test_normalize(Vec2d { x: 100.0, y: 20.0 });
	test_normalize(Vec2d { x: 20.0, y: 40.0 });
	test_normalize(Vec2d { x: 0.5, y: 0.1 });
	test_normalize(Vec2d { x: 0.0, y: 0.0 });
}
