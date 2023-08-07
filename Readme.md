# Contains
The Contains crate has 2 traits Container and In. 

## Container
The Container trait can be used to abstract over
types that can contain items: `Vec<T>`, `&[T]`, `HashMap<T>`, `Option<T>`, ect.
```rust
use contains::Container;

let vec = vec![1, 2, 3, 4, 5];
let range = 0..5;
let option = Some(3);

let containers: &[&dyn Container<usize>] = &[&vec, &range, &option];
for container in containers {
  assert!(container.contains(&3));
}
```

## In
The In trait is the Inverse of the Container trait and represents a type that is in
a container. Mainly it reverse the call order by providing the `is_in` method.
```rust
use contains::{Container, In};

let range = 0..5;
assert!(range.contains(&3));    // using contains
assert!(3.is_in(&range));       // using in
```