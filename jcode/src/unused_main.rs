use std::fmt;

/*****************************/
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
fn try_point() {
    let point = Point2D { x: 3.3, y: 7.2 };
    println!("{point}");
    println!("{point:#?}");
    println!("{point:?}");
}

/*****************************/
struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{count}:{v}")?;
        }
        write!(f, "]")
    }
}
fn try_list() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
    let x = [1, 2, 3, 4, 5];
    for i in 0..x.len() + 1 {
        match x.get(i) {
            Some(val) => println!("{i}: {val}"),
            None => println!("Slow down! {i} is too far!"),
        }
    }
}

/*****************************/
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}
fn try_inspect() {
    let pressed = WebEvent::KeyPress('x');
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

/*****************************/

fn main() {
    try_point();
    try_list();
    try_inspect();
}
