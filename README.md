# Simple code to derivate and integrate
I hope that this simple code will make you see the calculus operations of derivation and integration as something simple. 

## Description
This is a really simple code and it's purpose is to demystify what is considered a big mathematical monster for several people, the calculus. <br>
The code will be written in the Rust language programming language, but it's easy to understand. <br>
There is a function to calculate the derivative and there is a function where you put the mathematical function that you wish to derivate. <br>
There is also be a function to calculate the integral and there is a function where you put your mathematical function to integrate. <br>
Those methods are called numerical approximations or solutions to a derivative or a integration and there are many more methods that more advanced. But this ones are simple and easy to understand. The normal way that calculus is taught is the analysis way not the numerical way, and requires that you memorize several tables of derivatives and integrals and requires that you learn several different methods of integration. One doesn't substitute the other, they complement each other. It depends on the context of the application.


## What is the calculus of a derivative of a function at an point A?
It is the calculus of the value of the slop of a line that is tangent to the function graph at that point A in the XX axis. In source code, in the following, you calculate in very small increment or delta. This will approximate the value of the derivative. As you make the delta smaller, the approximation will be closer. This uses the definition of the derivative based on the limit when delta tends to zero.

```rust

const DELTA_X_DERIVATE:  f64 = 0.000_001_f64;  // dx

fn derivate(func: fn(f64) -> f64, a: f64) -> f64 {
    return (func(a + DELTA_X_DERIVATE) - func(a)) / DELTA_X_DERIVATE;
}

fn func_to_derivate( x: f64) -> f64 {
    // Function
    // y = e^x - e^(x + 4)
    return x.exp() - (x + 4.0).exp();
}

fn main() {
    // Derivate the function at the point A.
    let mut point_A: f64 = 7.0_f64; 
    let derivation_value: f64 = derivate(func_to_derivate, point_A);
    println!("\nDerivative of the function y = e^a - e^(a + 4) at point A={:.3} is {:.3}.", point_A, derivation_value);
}

```

```

Result:
Derivative of the function y = e^a - e^(a + 4) at point A=7.000 is -58777.538.

```


## What is the calculus of a integral of a function between two points A and B?
It is the calculus of the area below the function graph between two points A and B in the XX axis. In code it is the following and it is calculated in very small increments or deltas, and will give you the sum of the area os the small streps bellow the graph. This will approximate the value of the integral. As you make the delta smaller, the approximation will be closer.


```rust

const DELTA_X_INTEGRATE: f64 = 0.000_001_f64;  // dx

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
  
    // Integrate the function between points A and B.
    let point_A: f64 = 0.0_f64;
    let point_B: f64 = 5.0_f64;
    let integration_value: f64 = integrate(func_to_integrate, point_A, point_B);
    println!("\nIntegration of the function y = e^a - e^(a + 4) between point A= {:.3} and point B= {:.3} is {:.3}.\n", point_A, point_B, integration_value);
}

```


```

Result:
Integration of the function y = e^a - e^(a + 4) between point A= 0.000 and point B= 5.000 is -7901.069.

```


## References

* Programação em Python <br>
  by Ernesto Costa (in Portuguese)

* Numerical Methods for Engineers <br>
  by Steven Chapra and Raymond Canale


## License
MIT Open License. 


## Have fun!
Best regards, <br>
Joao Nuno Carvalho 