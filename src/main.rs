fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}
// struct Matrix(f32, f32, f32,f32);

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x:i64, y:i64},
}

fn inspect(event: WebEvent ){
    match event {
        WebEvent::PageLoad => println!("page loaded!"),
        WebEvent::PageUnload => print!("page unloaded"),
        WebEvent::KeyPress(c) => print!("pressed '{}'.",c),
        WebEvent::Paste(s) => print!("pasted\"{}\".",s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}
fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    println!("1+2={}", 1 + 2);

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;;
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}