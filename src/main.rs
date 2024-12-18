fn main() {
    println!("Hello, world!");
}

//===============================
// TYPES
//===============================

// a struct with a lifetime annotation
struct Car<'a> {
    model: &'a str
}

//===============================
// FUNCS
//===============================

// in this example, the compiler will error is you uncomment the print
fn dangling_reference() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    // println!("{}", r); 💥 Dangling reference!
}

// in this example, we have a function which returns a reference with annotated lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// in this example, we use a lifetime within a trait bound
fn print_with_lifetime<'a, T>(item: &'a T)
where
    T: std::fmt::Display + 'a,
{
    println!("{}", item);
}


