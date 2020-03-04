
// `xy` takes two references to `f32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `xy`.
pub fn xy<'a, 'b>(x: &'a f32, y: &'b f32) -> (&'a f32, &'b f32) {
    (x, y)
}

