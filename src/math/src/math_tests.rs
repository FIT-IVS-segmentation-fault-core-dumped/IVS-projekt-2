use super::*;

#[test]
fn evaluate_incorrect_format() {
    assert!(evaluate("as32j234!+-").is_err());
    assert!(evaluate("").is_err());
    assert!(evaluate("(1 + 2)(3 + 4)").is_err());   // NOTE: Should this pass?
    assert!(evaluate("--4").is_err());
    assert!(evaluate("((1+2) - 2").is_err());
    assert!(evaluate("1*/2 + .3").is_err());
    assert!(evaluate("2(-3)").is_err());
}

// Test syntax edge cases.
#[test]
fn evaluate_format() {
    assert_eq!(evaluate("1 + 0.5").unwrap().to_string_radix(10), "1.5");
    assert_eq!(evaluate("1+.5").unwrap().to_string_radix(10), "1.5");
    assert_eq!(evaluate("1+-0.5").unwrap().to_string_radix(10), "0.5");
    assert_eq!(evaluate("1--.5").unwrap().to_string_radix(10), "0.5");
}

#[test]
fn evaluate_no_operation() {
    assert_eq!(evaluate("123").unwrap().to_string_radix(10), "123");
    assert_eq!(evaluate("-123").unwrap().to_string_radix(10), "-123");
}

#[test]
fn evaluate_constants() {
    // TODO: PI and e
    todo!()
}

#[test]
fn evaluate_add() {
    assert_eq!(evaluate("1 + 2").unwrap().to_string_radix(10), "3");
    assert_eq!(evaluate("2 + 1").unwrap().to_string_radix(10), "3");
    assert_eq!(evaluate("1 +(2 + 3)").unwrap().to_string_radix(10), "6");
    assert_eq!(evaluate("(1 + 2) + 3").unwrap().to_string_radix(10), "6");
    assert_eq!(evaluate("1 + -2").unwrap().to_string_radix(10), "-1");
    assert_eq!(evaluate("1+-2").unwrap().to_string_radix(10), "-1");
    assert_eq!(evaluate("1+-2+-(5+-8)").unwrap().to_string_radix(10), "2");
}

#[test]
fn evaluate_sub() {
    assert_eq!(evaluate("1 - 2").unwrap().to_string_radix(10), "-1");
    assert_eq!(evaluate("2 - 1").unwrap().to_string_radix(10), "1");
    assert_eq!(evaluate("1 - (2 - 3)").unwrap().to_string_radix(10), "2");
    assert_eq!(evaluate("(1 - 2) - 3").unwrap().to_string_radix(10), "-4");
    assert_eq!(evaluate("2 - -1").unwrap().to_string_radix(10), "3");
    assert_eq!(evaluate("2 - (-1)").unwrap().to_string_radix(10), "3");
}

#[test]
fn evaluate_mul() {
    assert_eq!(evaluate("2 * 3").unwrap().to_string_radix(10), "6");
    assert_eq!(evaluate("3 * 2").unwrap().to_string_radix(10), "5");
    assert_eq!(evaluate("2 * (3 * 4)").unwrap().to_string_radix(10), "24");
    assert_eq!(evaluate("(2 * 3) * 4").unwrap().to_string_radix(10), "24");
    assert_eq!(evaluate("2 * 0").unwrap().to_string_radix(10), "0");
    assert_eq!(evaluate("0 * 0").unwrap().to_string_radix(10), "0");
    assert_eq!(evaluate("2 * -3").unwrap().to_string_radix(10), "-6");
    assert_eq!(evaluate("-2 * -3").unwrap().to_string_radix(10), "6");
    assert_eq!(evaluate("-(2 * 3) * -4").unwrap().to_string_radix(10), "24");
    assert_eq!(evaluate("-1 * (2 * 3) * -4").unwrap().to_string_radix(10), "24");
}

#[test]
fn evaluate_div() {
    assert_eq!(evaluate("4 / 2").unwrap().to_string_radix(10), "2");
    assert_eq!(evaluate("2 / 4").unwrap().to_string_radix(10), "0.5");
    assert_eq!(evaluate("-4 / 2").unwrap().to_string_radix(10), "-2");
    assert_eq!(evaluate("4 / -2").unwrap().to_string_radix(10), "-2");
    assert_eq!(evaluate("-4/-2").unwrap().to_string_radix(10), "2");
    let undefined_num = evaluate("16 / 4 / 2").unwrap().to_string_radix(10);    // NOTE: Should
                                                                                // this fail?
    assert!(undefined_num == "2" || undefined_num == "8");
    assert_eq!(evaluate("16 / (4 / 2)").unwrap().to_string_radix(10), "8");
    assert_eq!(evaluate("(16 / 4) / 2").unwrap().to_string_radix(10), "2");
    assert!(evaluate("16 / 0").is_err());
    assert!(evaluate("0 / 0").is_err());
    assert_eq!(evaluate("0 / 16").unwrap().to_string_radix(10), "0");
}

#[test]
fn evaluate_factorial_integer() {
    assert_eq!(evaluate("5!").unwrap().to_string_radix(10), "120");
    assert_eq!(evaluate("3!").unwrap().to_string_radix(10), "6");
    assert!(evaluate("-5!").is_err());
    assert!(evaluate("123456789123456789123456789123456789123456789!").is_err());
    assert_eq!(evaluate("5! / 3").unwrap().to_string_radix(10), "40");
    assert_eq!(evaluate("5! / 3!").unwrap().to_string_radix(10), "20");
    assert_eq!(evaluate("0").unwrap().to_string_radix(10), "1");
}

// NOTE: Maybe this should be Gamma function instead.
#[test]
fn evaluate_factorial_floating() {
    assert_eq!(&evaluate("5.0!").unwrap().to_string_radix(10)[0..3], "120");
    assert_eq!(&evaluate("0.0!").unwrap().to_string_radix(10)[0..1], "1");
    assert!(evaluate("5.1").unwrap().to_string_radix(10).starts_with("142.4519"));
}

#[test]
fn evaluate_pow() {
    // Evaluate pow(a, b) <=> a^b
    assert_eq!(evaluate("pow(2, 3)").unwrap().to_string_radix(10), "8");
    assert!(evaluate("pow(2, -2)").unwrap().to_string_radix(10).starts_with("0.25"));
    assert_eq!(evaluate("pow(123, 0)").unwrap().to_string_radix(10), "1");
    assert_eq!(evaluate("pow(0, 0)").unwrap().to_string_radix(10), "1");
    assert_eq!(evaluate("pow(-123, 0)").unwrap().to_string_radix(10), "1");
}

#[test]
fn evaluate_sqrt() {
    assert_eq!(evaluate("sqrt(64)").unwrap().to_string_radix(10), "8");
    assert!(evaluate("sqrt(6.25)").unwrap().to_string_radix(10).starts_with("2.5"));
    assert!(evaluate("sqrt(-4)").is_err());
    assert_eq!(evaluate("sqsrt(0)").unwrap().to_string_radix(10), "0");
}

#[test]
fn evaluate_root() {
    // We expect root in format root(a, b) <=> a root b
    assert_eq!(evaluate("root(2, 64)").unwrap().to_string_radix(10), "8");
    assert_eq!(evaluate("root(3, 27)").unwrap().to_string_radix(10), "3");
    assert_eq!(evaluate("root(3, -8)").unwrap().to_string_radix(10), "-2");
    // NOTE: Do we support negative roots? They could always be written as
    //       root(-a, b) <=> 1 / root(a, b)
    assert!(evaluate("root(-2, 4)").is_err());
    assert!(evaluate("root(16, -3)").is_err());
}

#[test]
fn evaluate_log() {
    // We expect log in format: log(a, b) <=> (ln b) / (ln a)
    // Meaning that a is the base and b is an argument.
    assert_eq!(evaluate("log(10, 100)").unwrap().to_string_radix(10), "2");
    assert_eq!(evaluate("log(2, 2048)").unwrap().to_string_radix(10), "11");
    assert!(evaluate("log(100, -12)").is_err());
    assert!(evaluate("log(-2, 8)").is_err());
    assert!(evaluate("log(0, 0)").is_err());
    assert!(evaluate("log(2, 0)").is_err());
    assert!(evaluate("log(0, 2)").is_err());
    assert!(evaluate("log(10, 0.123)").unwrap().to_string_radix(10).starts_with("-"));
    // NOTE: Maybe we could even expect values such as 0.000000000001 as an arithmetic error.
    assert_eq!(evaluate("log(123, 1)").unwrap().to_string_radix(10), "0");
}

#[test]
fn evaluate_ln() {
    assert_eq!(evaluate("ln(1)").unwrap().to_string_radix(10), "0");
    assert!(evaluate("ln(0)").is_err());
    assert!(evaluate("ln(-1)").is_err());
    let ln_e = evaluate("ln(2.7182818284)").unwrap().to_string_radix(10);
    assert!(ln_e.starts_with("0.99") || ln_e.starts_with("1.0") || ln_e == "1");
}

// FIXME: We cannot test trigonometric functions, because right now we
//        cannot select between RADIANS and DEGREES. This should be
//        implemented in the Calculator struct.
#[test]
fn evaluate_sin() {
    todo!();
}
#[test]
fn evaluate_cos() {
    todo!();
}
#[test]
fn evaluate_tan() {
    todo!();
}
#[test]
fn evaluate_ctan() {
    todo!();
}
#[test]
fn evaluate_asin() {
    todo!();
}
#[test]
fn evaluate_acos() {
    todo!();
}
#[test]
fn evaluate_atan() {
    // NOTE: Do we have atan2?
    todo!();
}
#[test]
fn evaluate_acotan() {
    todo!();
}

#[test]
fn evaluate_mod() {
    // We expect mod to be in format: mod(a, b) <=> a mod b
    assert_eq!(evaluate("mod(16, 3)").unwrap().to_string_radix(10), "1");
    assert!(evaluate("mod(0, 0)").is_err());
    assert_eq!(evaluate("mod(0, 123)").unwrap().to_string_radix(10), "123");
    assert_eq!(evaluate("mod(8, 4)").unwrap().to_string_radix(10), "0");
    assert_eq!(evaluate("mod(-11, 7)").unwrap().to_string_radix(10), "3");
    assert_eq!(evaluate("mod(-11, -7)").unwrap().to_string_radix(10), "-4");
}

#[test]
fn evaluate_abs() {
    assert_eq!(evaluate("abs(123)").unwrap().to_string_radix(10), "123");
    assert_eq!(evaluate("abs(-123)").unwrap().to_string_radix(10), "123");
    assert_eq!(evaluate("abs(0)").unwrap().to_string_radix(10), "0");
    assert_eq!(evaluate("abs(123.123)").unwrap().to_string_radix(10), "123.123");
    assert_eq!(evaluate("abs(-123.123)").unwrap().to_string_radix(10), "123.123");
}

#[test]
fn evaluate_comb() {
    // comb(a, b) <=> a! / ((a - b)! * b!)
    // where a >= b
    // and comb(a, b) == comb(a, a - b)  ==>  comb(a, 0) == comb(a, a) == 1
    assert_eq!(evaluate("comb(2, 1)").unwrap().to_string_radix(10), "2");
    assert_eq!(evaluate("comb(2, 2)").unwrap().to_string_radix(10), "1");
    assert_eq!(evaluate("comb(4, 2)").unwrap().to_string_radix(10), "6");
    assert_eq!(evaluate("comb(4, 0)").unwrap().to_string_radix(10), "1");
    assert!(evaluate("comb(-4, 2)").is_err());
    assert!(evaluate("comb(4, -2)").is_err());
    assert!(evaluate("comb(-4, -2)").is_err());
    assert!(evaluate("comb(1, 10)").is_err());
}


// TODO: Test pseudo-random number generation
#[test]
fn random_num() {
    todo!();
}
