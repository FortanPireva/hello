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