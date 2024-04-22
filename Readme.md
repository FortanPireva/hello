# Macros
## Macros are expanded into Rust code during compilation, and can take a variable number of arguments. They are distinguished by a ! at the end. The Rust standard library includes an assortment of useful macros.

``` println!(format, ..)``` prints a line to standard output, applying formatting described in std::fmt.
```format!(format, ..)``` works just like println! but returns the result as a string.
```dbg!(expression)``` logs the value of the expression and returns it.
```todo!()``` marks a bit of code as not-yet-implemented. If executed, it will panic.
```unreachable!()``` marks a bit of code as unreachable. If executed, it will panic.

## Break and Contiue with Labels
```
fn main() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    print!("elements searched: {elements_searched}");
}
```


## Arrays and Tuples
```
fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");
}
```

### Pattern Matching
Patterns and Destructuring
When working with tuples and other structured values it’s common to want to extract the inner values into local variables. This can be done manually by directly accessing the inner values:

```
fn print_tuple(tuple: (i32, i32)) {
    let left = tuple.0;
    let right = tuple.1;
    println!("left: {left}, right: {right}");
}
```

However, Rust also supports using pattern matching to destructure a larger value into its constituent parts:
```
fn print_tuple(tuple: (i32, i32)) {
    let (left, right) = tuple;
    println!("left: {left}, right: {right}");
}
```


## Shared References
A reference provides a way to access another value without taking responsibility for the value, and is also called “borrowing”. Shared references are read-only, and the referenced data cannot change.

```
fn main() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
}
```


## Shared References
A reference provides a way to access another value without taking responsibility for the value,
"borrowing"

```rust
fn main() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
}
```
A shared reference to a type T has type &T. A reference value is made with the & operator. The * operator “dereferences” a reference, yielding its value.

### Exclusive References
Exclusive references, also known as mutable references, allow changing the value they refer to. They have typ `&mut T`
```rust
fn main() {
    let mut point = (1,2);
    let x_cord = &mut point.0;
    *x_cord = 20;
    println!("point: {point:?}");
}
```


### Slices
A slice gives you a view into a larger collection:
```rust

fn main() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];

    println!("s: {s:?}");
}
```


### Strings
We can now understand the two string types in Rust:
- &str is a slice of UTF-8 encoded bytes, similar to &[u8];
- String is an owned, heap-allocated buffer to UTF-8 bytes.
```rust
fn main() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[s2.len() - s1.len()..];
    println!("s3: {s3}");
}
```