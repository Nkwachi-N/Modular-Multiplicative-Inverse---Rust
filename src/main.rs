use std::cmp::Ordering;


///Function in Rust comes with the fn keyword. This function takes
/// i32 equals a 32 bit signed integer, meaning it can hold both positive and negative integers.
/// All variables in Rust are constant and the mut keyword denotes variables that can change values.
/// The & before the variable indicates it is passed by reference and not by value
/// 
/// 
/// The Euclidean Algorithm is a technique for quickly finding the GCD of two integers.
/// The Euclidean Algorithm for finding GCD(A,B) is as follows:
/// If A = 0 then GCD(A,B)=B, since the GCD(0,B)=B, and we can stop.
/// Write A in quotient remainder form (A = B⋅Q + R), where R is the Remainder and Q is the Quotient.
/// Thus GCD(A,B) = GCD(B,R) and we continue till we get to 0 (the base case).
/// E.g Find the GCD of 2 and 7 =  GCD(7,2).
/// GCD(7,2) = GCD(2,7%2) = GCD(2,1) = GCD(1, 2 % 1) = GCD(1,0) = 1.
/// GCD of 7 and 2 is 1.
/// 
/// 
///Extended Euclidean algorithm also finds integer coefficients x and y such that: ax + by = gcd(a, b) 

///ax + by = gcd(a, b)
///gcd(a, b) = gcd(b%a, a) using Euclidean Algorithm
///gcd(b%a, a) = (b%a)x1 + ay1  [From law ax + by = gcd(a,b)]
/// 
///ax + by = (b%a)x1 + ay1  -- Since gcd(a,b) = gcd(b%a,a) and gcd(a,b) = ax + by : gcd(b%a,a) = b%ax1 + ay1
/// 
/// 
///ax + by = (b – [b/a] * a)x1 + ay1
///ax + by = a(y1 – [b/a] * x1) + bx1

///Comparing LHS and RHS,
///x = y1 – ⌊b/a⌋ * x1
/// y = x1
   

fn gcd_extended(a: i32, b: i32,x:&mut i32, y: &mut i32) -> i32 {
    // Base Case
    if a == 0 {
        // * The keyword allows you to access the value of variables passed by reference.
        *x = 0;
        *y = 1;
        return b;
    }

    // To store results of recursive call
    let gcd = gcd_extended(b % a, a,x,y);
    let x1 = *x;
    let y1 = *y;

    // Update x and y using results of recursive call
    let tmp = b / a;
    *x = y1 - tmp * x1;
    *y = x1;

    return gcd;
}




fn mod_inverse(a: i32, m: i32) {
    let mut x = 0;
    let mut y = 0;

    //x and y variables are passed with a mutable reference.
    let g = gcd_extended(a, m,&mut x,&mut y);

    //The modular inverse only exists when gcd(a,b) = 1.
    match g.cmp(&1) {
        Ordering::Equal => {
            // m is added to handle negative x
            let res = (x % m + m) % m;
            println!("Modular multiplicative inverse is {}", res);
        }
        Ordering::Greater => println!("Inverse doesn't exist"),
        Ordering::Less => println!("Inverse doesn't exist"),
    }
}


///The main function is the entry point of Rust programs
fn main() {
    let a = 3;
    let m = 11;

    mod_inverse(a, m);
}
