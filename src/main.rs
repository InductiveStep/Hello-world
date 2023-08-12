// My first Rust program


// The main function that runs first
fn main() {
	fun();
	fibs(20);
}

fn fun() {
	println!("My first Rust program!");
	println!("Here's the Fibonacci sequence:");	
}

// Nice notation for inputs and outputs. i32 is a 32 bit int
fn fib(n: i32) -> i32 {
	if n == 0 || n == 1 {
		n
	}
	else {
		fib(n - 1) + fib(n - 2)
	}
}

// Also nice, though x..y loops from x to y - 1, not y
fn fibs(max_n: i32) {
	for i in 0..max_n {
		println!("{}\t{}", i, fib(i));
	}
}

// The end.
