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
    assert_eq!(eval_dec("-2 * -3", 3)?, "-6");
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
    assert_eq!(eval_dec("1 / 3", 6)?, "0.333334");
    assert_eq!(eval_dec("2 * 10 / 20", 3)?, "1");
    Ok(())
}

#[test]
fn evaluate_fact() -> math::Result<()> {
    todo!();
}
#[test]
fn evaluate_pow() -> math::Result<()> {
    todo!();
}
#[test]
fn evaluate_root() -> math::Result<()> {
    todo!();
}
#[test]
fn evaluate_log() -> math::Result<()> {
    todo!();
}

#[test]
fn evaluate_sin() -> math::Result<()> {
    todo!();
}
#[test]
fn evaluate_cos() -> math::Result<()> {
    todo!();
}
#[test]
fn evaluate_tg() -> math::Result<()> {
    todo!();
}
#[test]
fn evaluate_cotg() -> math::Result<()> {
    todo!();
}

#[test]
fn evaluate_arcsin() -> math::Result<()> {
    todo!();
}
#[test]
fn evaluate_arccos() -> math::Result<()> {
    todo!();
}
#[test]
fn evaluate_arctg() -> math::Result<()> {
    todo!();
}
#[test]
fn evaluate_arccotg() -> math::Result<()> {
    todo!();
}

#[test]
fn evaluate_abs() -> math::Result<()> {
    todo!();
}
#[test]
fn evaluate_comb() -> math::Result<()> {
    todo!();
}

