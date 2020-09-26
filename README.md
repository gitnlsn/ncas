<html>
<h1 align="center">Rust Symbolic Algebraic</h1>
<p align="center" >

<img src="https://img.shields.io/badge/Language-Rust-red.svg" />

<img src="https://img.shields.io/github/license/nelsonatgithub/ncas" />

<img src="https://img.shields.io/github/issues/nelsonatgithub/ncas" />

<img src="https://img.shields.io/github/stars/nelsonatgithub/ncas" />

<img src="https://img.shields.io/github/forks/nelsonatgithub/ncas" />

</p>
<p align="center" >

<a href="https://travis-ci.org/github/nelsonatgithub/ncas">
  <img src="https://travis-ci.org/nelsonatgithub/ncas.svg?branch=dev" />
</a>

<a href="https://codecov.io/gh/nelsonatgithub/ncas">
  <img src="https://codecov.io/gh/nelsonatgithub/ncas/branch/dev/graph/badge.svg" />
</a>

</p>

</html>

# Description

This repository aims to implement a crate for manipulation of symbolic algebraic expressions.

The major objective is to provide an API that genericaly manipulates mathematical expressions in pure Rust. It should provide an environment for building more complex algebraic applications or applications that simply requires algebraic expressions abstraction.

The code below demonstrates the desired syntax. (work in progress)

``` rust
#[test]
fn sample() {
    use crate::symbols::variable::Variable;
    use crate::calculus::integration::integrate;
    use crate::exponential::neperian_logarithm::ln;
    use crate::trigonometrics::{sine::sin, cossine::cos};

    let x = &Variable::new(String::from("x"));

    let fx = (x ^ 2 + 1) / (x * (sin(x) ^ 2 + cos(x) ^ 2));
    let expected = ((x ^ 2) / 2) + ln(x);

    assert_eq!(integrate(fx, x), expected);
}
```

> The scientific references considered in this project are presented at the end.

# Features

It should handle expressions containing:

    [x] polynomials
    [ ] trigonometrics
    [ ] exponentials
    [ ] logaritmics
    [ ] rationals
    [ ] indefinite integrals (integral(f(x)) composing the expression)
    [ ] indefinite differentials (df/dx composing the expression)

It should provide façade methods and overloaded operations for:

    [x] basic arithmetics
    [x] numerical evaluation
    [x] comparison and ordenation
    [ ] normal equality
    [ ] substitution
    [ ] expansion
    [ ] simplification

Advanced topics based on more elaborated algorithms are not to be implemented.

* **Transformations** 
    - Riemann Series
    - Fourier Series
    - Laplace Series

* **Solvers** 
    - system of algebraic equations
    - differential equations
    - integral equations
    - numerical solvers

* Polynomial **factorization**:
    - Berlekamp's algorithm
    - Cantor-Zassenhaus algorithm
    - Trager's algorithm

# API

> In progress

```rust
#[test]
fn basic_api() {
    /* number value comparison */
    use crate::symbols::{number::Number, variable::Variable};
    let one = &Number::new(1.0);
    let two = &Number::new(2.0);
    assert_eq!(one + one, two);
    
    /* commutative */
    let a = &Variable::new(String::from("a"));
    let b = &Variable::new(String::from("b"));
    let c = &Variable::new(String::from("c"));
    assert_eq!(a + b + c, c + b + a);
}
```

# Testing and Contributions

This repository is being developed through Cargo with minimal configurations.

``` bash
# Testing is straightforward with cargo
> cargo test
```

# References

1. Cohen, Joel S. Computer Algebra and Symbolic Computation: Mathematical Methods. 2003 by A K Peters, Ltd.

2. Cohen, Joel S. Computer Algebra and Symbolic Computation: Elementary Algorithms. 2002 by A K Peters, Ltd.

3. Geddes, K.O., Czapor S.R., Labahn, G. Algorithms for Computer Algebra. 1992 by Kluwer Academic Publishers.

4. Carette, J. Understanding expression simplification. In: Proc. 2004 Int. Symp. on Symb. and Algebr. Comp. ISSAC ’04. ACM, New York, NY, USA, pp. 72–79.

5. D.H. Bailey, J.M. Borwein, A.D. Kaiser. Automated simplification of large symbolic expressions J. Symb. Anal., 60 (2013), pp. 120-136.
