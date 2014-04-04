use common::vec::Vec2;


#[test]
fn it_should_add_two_vectors() {
	let a = Vec2 { x: 1.0, y: 2.0, z: 3.0 };
	let b = Vec2 { x: 3.0, y: 2.0, z: 1.0 };

	assert!(a + b == Vec2 { x: 4.0, y: 4.0, z: 4.0 });
}

#[test]
fn it_should_subtract_a_vector_from_another() {
	let a = Vec2 { x: 2.0, y: 4.0, z: 6.0 };
	let b = Vec2 { x: 1.0, y: 2.0, z: 3.0 };

	assert!(a - b == Vec2 { x: 1.0, y: 2.0, z: 3.0 });
}

#[test]
fn it_should_scale_a_vector() {
	let v = Vec2 { x: 1.0, y: 2.0, z: 3.0 };
	let s = 2.0;

	assert!(v * s == Vec2 { x: 2.0, y: 4.0, z: 6.0 });
}

#[test]
fn it_should_compute_a_vectors_magnitude() {
	let v = Vec2 { x: 4.0, y: 4.0, z: 2.0 };

	assert!(v.magnitude() == 6.0);
}

#[test]
fn it_should_normalize_a_vector() {
	let a = Vec2 { x: 3.0, y: 0.0, z: 0.0 };
	let b = Vec2 { x: 0.0, y: 4.0, z: 0.0 };
	let c = Vec2 { x: 0.0, y: 0.0, z: 5.0 };

	assert!(a.normalize() == Vec2 { x: 1.0, y: 0.0, z: 0.0 });
	assert!(b.normalize() == Vec2 { x: 0.0, y: 1.0, z: 0.0 });
	assert!(c.normalize() == Vec2 { x: 0.0, y: 0.0, z: 1.0 });
}
