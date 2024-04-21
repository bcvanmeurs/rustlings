// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        // https://doc.rust-lang.org/std/keyword.ref.html
        // ref annotates pattern bindings to make them borrow rather than move.
        // It is not a part of the pattern as far as matching is concerned: it does not affect
        // whether a value is matched, only how it is matched.
        // By default, match statements consume all they can, which can sometimes be a problem,
        // when you don’t really need the value to be moved and owned.
        // Using the ref keyword, the value is only borrowed, not moved, making it available for
        // use after the match statement
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.

    // - & denotes that your pattern expects a reference to an object.
    // Hence & is a part of said pattern: &Foo matches different objects than Foo does.
    // ref indicates that you want a reference to an unpacked value.
    // It is not matched against: Foo(ref foo) matches the same objects as Foo(foo)
}
