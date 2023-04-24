use math::evaluate;
use math::number::Radix;

fn eval_dec(s: &str, precision: u8) -> math::Result<String> {
    Ok(evaluate(s)?.to_string(Radix::Dec, precision))
}

#[test]
fn evaluate_add() -> math::Result<()> {
    assert_eq!(eval_dec("1 + 2", 3)?, "3");
    assert_eq!(eval_dec("2 + 1", 3)?, "3");
    assert_eq!(eval_dec("1 + (2 + 3)", 3)?, "6");
    assert_eq!(eval_dec("(1 + 2) + 3", 3)?, "6");
    assert_eq!(eval_dec("1 + -1", 3)?, "0");
    assert_eq!(eval_dec("-1000 + 100", 3)?, "-900");
    assert_eq!(eval_dec("1.123 + 2.456", 3)?, "3.579");
    Ok(())
}
#[test]
fn evaluate_sub() -> math::Result<()> {
    assert_eq!(eval_dec("1 - 1", 3)?, "0");
    assert_eq!(eval_dec("1 - 2", 3)?, "-1");
    assert_eq!(eval_dec("1 --1", 3)?, "2");
    assert_eq!(eval_dec("-1--1000", 3)?, "999");
    assert_eq!(eval_dec("1 - (2 - 3)", 3)?, "2");
    Ok(())
}
#[test]
fn evaluate_mul() -> math::Result<()> {
    assert_eq!(eval_dec("2 * 3", 3)?, "6");
    assert_eq!(eval_dec("3 * 2", 3)?, "6");
    assert_eq!(eval_dec("-2 * -3", 3)?, "6");
    assert_eq!(eval_dec("2 * (3 * 4)", 3)?, "24");
    assert_eq!(eval_dec("(2 * 3) * 4", 3)?, "24");
    assert!(evaluate("2**3").is_err());
    Ok(())
}
#[test]
fn evaluate_div() -> math::Result<()> {
    assert_eq!(eval_dec("6 / 3", 3)?, "2");
    assert_eq!(eval_dec("6 / -3", 3)?, "-2");
    assert_eq!(eval_dec("3 / 6", 3)?, "0.5");
    assert_eq!(eval_dec("1 / 3", 6)?, "0.333333");
    assert_eq!(eval_dec("2 * 10 / 20", 3)?, "1");
    Ok(())
}

#[test]
fn evaluate_fact() -> math::Result<()> {
    assert_eq!(eval_dec("0!", 0)?, "1");
    assert_eq!(eval_dec("5!", 0)?, "120");
    assert_eq!(eval_dec("3.2!", 5)?, "7.75669");
    Ok(())
}
#[test]
fn evaluate_pow() -> math::Result<()> {
    assert_eq!(eval_dec("pow(-123, 0)", 0)?, "1");
    assert_eq!(eval_dec("pow(123, 0)", 0)?, "1");
    assert_eq!(eval_dec("pow(2, 3) * pow(2, 5)", 0)?, "256");
    assert_eq!(eval_dec("pow(pow(2, 3), 3)", 0)?, "512");
    assert_eq!(eval_dec("pow(2, -2)", 2)?, "0.25");
    assert!(eval_dec("pow(pow(0, 0), 0)", 3).is_ok());
    Ok(())
}
#[test]
fn evaluate_pow_infix_notation() -> math::Result<()> {
    assert_eq!(eval_dec("-123^0", 0)?, "1");
    assert_eq!(eval_dec("123 ^ 0", 0)?, "1");
    assert_eq!(eval_dec("2^3 * 2^5", 0)?, "256");
    assert_eq!(eval_dec("2^3^3", 0)?, "512");
    assert_eq!(eval_dec("2^-2", 2)?, "0.25");
    assert!(eval_dec("0^0^0", 3).is_ok());
    Ok(())
}
#[test]
fn evaluate_mod() -> math::Result<()> {
    assert_eq!(eval_dec("7 mod 3", 0)?, "1");
    assert_eq!(eval_dec("7 mod-3", 0)?, "-2");
    assert_eq!(eval_dec("-7 mod 3", 0)?, "2");
    assert_eq!(eval_dec("-7 mod -3", 0)?, "-1");
    Ok(())
}
#[test]
fn evaluate_root() -> math::Result<()> {
    assert_eq!(eval_dec("root(2, 64)", 0)?, "8");
    assert_eq!(eval_dec("root(3, -64)", 0)?, "-4");
    assert!(eval_dec("root(2, -64)", 3).is_err());
    assert!(eval_dec("root(122, -64)", 3).is_err());

    assert_eq!(eval_dec("sqrt(2)", 6)?, "1.414214");
    assert!(eval_dec("sqrt(-2)", 3).is_err());
    assert_eq!(eval_dec("sqrt(9)", 3)?, "3");
    Ok(())
}
#[test]
fn evaluate_log() -> math::Result<()> {
    assert_eq!(eval_dec("log(13, 169)", 0)?, "2");
    assert_eq!(eval_dec("log(3, 27)", 0)?, "3");
    assert_eq!(
        eval_dec("log(50, 123 * 456)", 5)?,
        eval_dec("log(50, 123) + log(50, 456)", 5)?
    );
    assert_eq!(
        eval_dec("log(50, 123 / 456)", 5)?,
        eval_dec("log(50, 123) - log(50, 456)", 5)?
    );

    assert!(eval_dec("log(0, 123)", 3).is_err());
    assert!(eval_dec("log(123, 0)", 3).is_err());
    assert!(eval_dec("log(-1, 123)", 3).is_err());
    assert!(eval_dec("log(1, -123)", 3).is_err());
    assert!(eval_dec("log(-1, -123)", 3).is_err());

    assert_eq!(eval_dec("log(3, 123)", 2)?, eval_dec("ln(123) / ln(3)", 2)?);
    assert_eq!(eval_dec("log(e(), 10)", 5)?, eval_dec("ln(10)", 5)?);
    assert_eq!(eval_dec("pow(123, log(123, 10))", 2)?, "10");
    Ok(())
}

#[test]
fn evaluate_sin() -> math::Result<()> {
    assert_eq!(eval_dec("sin(123)", 7)?, "-0.4599035");
    assert_eq!(eval_dec("sin(-97)", 7)?, eval_dec("-sin(97)", 7)?);
    assert_eq!(eval_dec("sin(0)", 3)?, "0");
    assert_eq!(eval_dec("sin(pi())", 0)?, "0");
    assert_eq!(eval_dec("sin(pi() / 2)", 0)?, "1");

    assert_eq!(eval_dec("sin(3.141592 / 2)", 0)?, "1");
    assert_eq!(eval_dec("sin(3.141592)", 0)?, "0");
    assert_eq!(eval_dec("sin(3.141592 * 3 / 2)", 0)?, "-1");
    assert_eq!(eval_dec("sin(3.141592 * 2)", 0)?, "0");
    Ok(())
}
#[test]
fn evaluate_cos() -> math::Result<()> {
    assert_eq!(eval_dec("cos(123)", 6)?, "-0.887969");
    assert_eq!(eval_dec("cos(-97)", 7)?, eval_dec("cos(97)", 7)?);
    assert_eq!(eval_dec("cos(0)", 0)?, "1");
    assert_eq!(eval_dec("cos(pi())", 0)?, "-1");
    assert_eq!(eval_dec("cos(pi() / 2)", 0)?, "0");

    assert_eq!(eval_dec("cos(3.141592 / 2)", 0)?, "0");
    assert_eq!(eval_dec("cos(3.141592)", 0)?, "-1");
    assert_eq!(eval_dec("cos(3.141592 * 3 / 2)", 0)?, "0");
    assert_eq!(eval_dec("cos(3.141592 * 2)", 0)?, "1");
    Ok(())
}
#[test]
fn evaluate_tg() -> math::Result<()> {
    assert_eq!(eval_dec("tg(123)", 6)?, "0.517927");
    assert_eq!(eval_dec("tg(-97)", 7)?, eval_dec("-tg(97)", 7)?);
    assert_eq!(eval_dec("tg(0)", 0)?, "0");
    assert_eq!(eval_dec("tg(3.141592 / 4)", 0)?, "1");
    assert_eq!(eval_dec("tg(3.141592 * 3 / 4)", 0)?, "-1");
    assert_eq!(eval_dec("tg(3.141592 * 5 / 4)", 0)?, "1");
    assert_eq!(eval_dec("tg(-3.141592 / 4)", 0)?, "-1");
    assert!(eval_dec("tg(pi() / 2)", 0).is_err());
    assert_eq!(eval_dec("tg(15)", 3)?, eval_dec("sin(15) / cos(15)", 3)?);
    Ok(())
}
#[test]
fn evaluate_cotg() -> math::Result<()> {
    assert_eq!(eval_dec("cotg(123)", 5)?, "1.93077");
    assert_eq!(eval_dec("cotg(50)", 5)?, eval_dec("1 / tg(50)", 5)?);
    assert_eq!(eval_dec("cotg(-97)", 7)?, eval_dec("-cotg(97)", 7)?);
    assert_eq!(eval_dec("cotg(3.141592 / 4)", 0)?, "1");
    assert_eq!(eval_dec("cotg(3.141592 * 3 / 4)", 0)?, "-1");
    assert_eq!(eval_dec("cotg(3.141592 * 5 / 4)", 0)?, "1");
    assert_eq!(eval_dec("cotg(-3.141592 / 4)", 0)?, "-1");
    assert_eq!(eval_dec("cotg(pi() / 2)", 0)?, "0");
    assert!(eval_dec("cotg(0)", 0).is_err());
    assert_eq!(eval_dec("cotg(15)", 3)?, eval_dec("cos(15) / sin(15)", 3)?);
    Ok(())
}

#[test]
fn evaluate_arcsin() -> math::Result<()> {
    assert_eq!(eval_dec("arcsin(0.3912)", 8)?, "0.40193515");
    assert_eq!(eval_dec("arcsin(sin(0.123))", 3)?, "0.123");
    assert_eq!(eval_dec("sin(arcsin(0.123))", 3)?, "0.123");
    assert!(eval_dec("arcsin(1.0001)", 3).is_err());
    assert!(eval_dec("arcsin(-1.0001)", 3).is_err());
    Ok(())
}
#[test]
fn evaluate_arccos() -> math::Result<()> {
    assert_eq!(eval_dec("arccos(0.3912)", 8)?, "1.16886118");
    assert_eq!(eval_dec("arccos(cos(0.123))", 3)?, "0.123");
    assert_eq!(eval_dec("cos(arccos(0.123))", 3)?, "0.123");
    assert!(eval_dec("arccos(1.0001)", 3).is_err());
    assert!(eval_dec("arccos(-1.0001)", 3).is_err());
    Ok(())
}
#[test]
fn evaluate_arctg() -> math::Result<()> {
    assert_eq!(eval_dec("arctg(0.123)", 5)?, "0.12239");
    assert_eq!(eval_dec("arctg(tg(0.123))", 3)?, "0.123");
    assert_eq!(eval_dec("tg(arctg(0.123))", 3)?, "0.123");
    assert_eq!(eval_dec("arctg(99999999)", 5)?, eval_dec("pi() / 2", 5)?);
    assert_eq!(eval_dec("arctg(-99999999)", 5)?, eval_dec("-pi() / 2", 5)?);
    Ok(())
}
#[test]
fn evaluate_arccotg() -> math::Result<()> {
    assert_eq!(eval_dec("arccotg(0.123)", 5)?, "1.44841");
    assert_eq!(eval_dec("arccotg(cotg(0.123))", 3)?, "0.123");
    assert_eq!(eval_dec("cotg(arccotg(0.123))", 3)?, "0.123");
    assert_eq!(eval_dec("arccotg(99999999)", 5)?, eval_dec("0", 5)?);
    assert_eq!(eval_dec("arccotg(-99999999)", 5)?, eval_dec("pi()", 5)?);
    Ok(())
}

#[test]
fn evaluate_abs() -> math::Result<()> {
    assert_eq!(eval_dec("abs(12)", 0)?, "12");
    assert_eq!(eval_dec("abs(-12)", 0)?, "12");
    assert_eq!(
        eval_dec("abs(-3 * 5)", 0)?,
        eval_dec("abs(-3) * abs(5)", 0)?
    );
    Ok(())
}
#[test]
fn evaluate_comb() -> math::Result<()> {
    assert!(eval_dec("comb(-1, 123)", 0).is_err());
    assert!(eval_dec("comb(123, -1)", 0).is_err());
    assert_eq!(eval_dec("comb(3, 4)", 0)?, "0");
    assert_eq!(eval_dec("comb(3, 3)", 0)?, "1");
    assert_eq!(eval_dec("comb(4, 2)", 0)?, "6");
    assert_eq!(eval_dec("comb(4, 1)", 0)?, "4");
    assert_eq!(
        eval_dec("comb(12, 6)", 0)?,
        eval_dec("12! / ((12 - 6)! * 6!)", 0)?
    );
    Ok(())
}

#[test]
fn evaluate_expr() -> math::Result<()> {
    let dec_eq = |s1, s2, p| -> math::Result<()> {
        assert_eq!(eval_dec(s1, p)?, eval_dec(s2, p)?);
        Ok(())
    };

    dec_eq("sin(123) * sin(123) + cos(123) * cos(123)", "1", 0)?;
    dec_eq("sin(56)", "cos(56 - pi() / 2)", 4)?;
    dec_eq("123", "123", 0)?;
    dec_eq("0.1", "0.2 - 0.1", 2)?;
    dec_eq("ln(pow(5, 13))", "13 * ln(5)", 4)?;
    dec_eq("root(13, pow(3, 13))", "3", 0)?;

    Ok(())
}

#[test]
fn evaluate_expr_hidden_multiply_sign() -> math::Result<()> {
    assert_eq!(eval_dec("3pi()", 5)?, "9.42478");
    assert_eq!(eval_dec("e()pi()", 5)?, "8.53973");
    assert_eq!(eval_dec("1 + 2(3 + 4!e())", 5)?, "137.47752");
    assert_eq!(eval_dec("pi()2 / 2", 5)?, "3.14159");
    Ok(())
}

#[test]
fn evaluate_constants() -> math::Result<()> {
    assert_eq!(eval_dec("pi()", 6)?, "3.141593");
    assert_eq!(eval_dec("e()", 6)?, "2.718282");
    Ok(())
}
