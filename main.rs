/******************************************************************************
* Author:      Joao Nuno Carvalho                                             *
* Date:        2020.10.7                                                      *
* Description: This is a really simple code and it's purpose is to demystify  *
*              what is considered a big mathematical monster by several       *
*              persons, the calculus.                                         *
*              The code will be written in the Rust language but it's easy to *
*              understand.                                                    *
*              There is a function to calculate the derivative, there is a    *
*              function to put the mathematical function that you wish to     *
*              derivate.                                                      *
*              There is also be a function to calculate the integral and a    *
*              function where you put your mathematical function to integrate.*
* License:     MIT Open License                                               *
* Note:        The returns are not necessary but in this case they aid in     *
*              code understanding.                                            *
******************************************************************************/

const DELTA_X_DERIVATE:  f64 = 0.000_001_f64;  // dx
const DELTA_X_INTEGRATE: f64 = 0.000_001_f64;  // dx

fn derivate(func: fn(f64) -> f64, a: f64) -> f64 {
    return (func(a + DELTA_X_DERIVATE) - func(a)) / DELTA_X_DERIVATE;
}

fn func_to_derivate( x: f64) -> f64 {
    // Function
    // y = e^x - e^(x + 4)
    return x.exp() - (x + 4.0).exp();
}

fn integrate(func: fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let mut value: f64 = 0.0_f64;
    let mut x = a;
    while x <= b {
        value = value + func(x) * DELTA_X_INTEGRATE;
        x = x + DELTA_X_INTEGRATE;
    }
    return value;
}

fn func_to_integrate( x: f64) -> f64 {
    // Function
    // y = e^x - e^(x + 4)
    return x.exp() - (x + 4.0).exp();
}

fn main() {
    println!("\nSimple examples of derivation and integration...");

    // Derivate the function at the point A.
    let mut point_A: f64 = 7.0_f64; 
    let derivation_value: f64 = derivate(func_to_derivate, point_A);
    println!("\nDerivative of the function y = e^a - e^(a + 4) at point A={:.3} is {:.3}.", point_A, derivation_value);

    // Integrate the function between points A and B.
    point_A = 0.0_f64;
    let point_B: f64 = 5.0_f64;
    let integration_value: f64 = integrate(func_to_integrate, point_A, point_B);
    println!("\nIntegration of the function y = e^a - e^(a + 4) between point A= {:.3} and point B= {:.3} is {:.3}.\n", point_A, point_B, integration_value);
}
