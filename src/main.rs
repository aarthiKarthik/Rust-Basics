#![allow(dead_code)]
#![allow(unused_variable)]

use std::mem;
mod sh;

// No address for this variable, value inlined
const A_CONSTANT:u8 = 12;

// static denotes global scope, with a fixed address
// since it is global, with fixed addr, many threads 
// could modify it simultaneously, so its usage must 
// be enclosed in unsafe block
static mut A_STATIC_VAR:u8 = 20;

fn data_types() {
	// u8
	let a:u8 = 123;
	println!("a = {}", a);
	
	// mut i8
	let mut b:i8 = 0;
	println!("b = {}", b);
	b = 30;
	println!("b = {}", b);

	// mut i32
	let mut c =  123456;
	println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
	c = -76;
	println!("c = {}", c);

	// isize
	let z:isize = 123;
	println!("z = {}, sizeof z is {} bytes", z, mem::size_of_val(&z));

	// char - takes 4 bytes
	let d = 'a';  
	println!("d = {}, sizeof d is {} bytes", d, mem::size_of_val(&d));

	// dbl precision - 8 bytes (default)
	let e:f64 = 2.5;
	// single precision
	let f:f32 = 1.0;
	println!("e = {}, sizeof e is {} bytes", e, mem::size_of_val(&e));
	println!("f = {}, sizeof f is {} bytes", f, mem::size_of_val(&f));

	// boolean
	let g = false;
	println!("g = {}, sizeof g is {} bytes", g, mem::size_of_val(&g));
}

fn operators() {
	// arithmetic
	let mut a = 2 + 3 * 4;
	println!("a (bef)= {}", a);
	a += 1; 
	println!("a (aft)= {}", a);
	println!("remainder of {} / {} = {}", a, 2, a%2);

	a = 3;
	a /= 2;
	println!("a / 2 = {}", a); // int division

	// no pow of operator; to use built-in func instead
	let a3 = i32::pow(a, 3);
	println!("{} cubed is {}", a, a3);

	let b = 2.5;
	let b2 = f64::powi(b, 2); // denotes integer power
	// uses series expansion to eval
	let b_to_root2 = f64::powf(b, std::f64::consts::SQRT_2); // note usage of built-in constant
	println!("{} squared is {}, power of sqrt(2) is {}", b, b2, b_to_root2);

	// bitwise
	let c = 1 | 2; // | - OR, & - AND, ^ - XOR, ! - NOR
	println!("1 | 2 = {}", c);
	println!("2^10 = {}", 1 << 10); // l-shifting

	// logical
	let _cond1 = std::f64::consts::PI < 4.0;
	let x = 5;
	let _cond2 = x == 5; // equality check

}

fn scope_and_shadowing() {
	let a = 123;

	// Shadowing, using let allows to change an immutable variable's value
	let a = 777;	// this is allowed, a would now take value of 777
	{
		let b = 456;
		
		println!("{}", b);
		
		// With let, inner 'a' shadows outer 'a'
		// Without let, 'a' refers to outer 'a'
		println!("Inside, global {}", a);
		let a = 789;		
		println!("Inside, local {}", a);
	}
	println!("Outside {}", a);
}

fn static_var_print() {
	// since it is global, with fixed addr, many threads 
	// could modify it simultaneously, so its usage must 
	// be enclosed in unsafe block
	unsafe {
		println!("Unsafe usage of static var: {}", A_STATIC_VAR);
	}
}

fn if_stmt() {
	let age = 20;

	if age > 18 {
		println!("Major");
	}
	else if age == 18 {
		println!("Wait awhile");
	}
	else {
		println!("Minor", );
	}
}

fn main() {
	data_types();	
	operators();
	scope_and_shadowing();
	static_var_print();
	sh::heap_usage();
	if_stmt();
}