#![allow(dead_code)]

use std::mem;

struct Point {
    x:f64,
    y:f64
}

fn origin() -> Point {
    Point{x:0.0, y:0.0}
}

pub fn heap_usage() {
    // y is a pointer which holds address to alloc'ed mem
    let y = Box::new(10); 
	println!("{}", *y); 

    let p1 = origin();
    let p2 = Box::new(origin()); // pointer

    println!("Stack var p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("Heap var (ptr) p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("p3.x = {}", p3.x); // moving heap value to stack

}