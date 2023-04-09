//! Define internal tests for the calculator state.

use super::CalcState;

// Wrapper for testing the get_eval_str() method.
fn convert(display_text: String) -> String {
    let mut calc_state = CalcState::new(&["en"]);
    calc_state.displayed_text = display_text;
    calc_state.get_eval_str()
}

// Generic tests for binary operations
fn bin_opt_template(display_opt: &str, eval_opt: &str) {
    // convert("2*3root8") == "2*root(3,8)"
    assert_eq!(
        convert(format!("2*3{}8", display_opt)),
        format!("2*{}(3,8)", eval_opt)
    );

    // convert("4+24root6") == "4+root(24,6)"
    assert_eq!(
        convert(format!("4+24{}6", display_opt)),
        format!("4+{}(24,6)", eval_opt)
    );

    // convert("24root62") == "root(24,62)"
    assert_eq!(
        convert(format!("24{}62", display_opt)),
        format!("{}(24,62)", eval_opt)
    );

    // convert("-1+(10)root(-2*3)") == "-1+root(10,-2*3)"
    assert_eq!(
        convert(format!("-1+(10){}(-2*3)", display_opt)),
        format!("-1+{}(10,-2*3)", eval_opt)
    );

    // convert("3root8*4root16") == "root(3,8)*root(4,16)"
    assert_eq!(
        convert(format!("3{}8*4{}16", display_opt, display_opt)),
        format!("{}(3,8)*{}(4,16)", eval_opt, eval_opt)
    );

    // convert("3root(4root16)") == "root(3,root(4,16))"
    assert_eq!(
        convert(format!("3{}(4{}16)", display_opt, display_opt)),
        format!("{}(3,{}(4,16))", eval_opt, eval_opt)
    );
}

// Generic tests for unary operations
fn unary_opt_template(display_opt: &str, eval_opt: &str) {
    // convert("sqrt8") == "sqrt(8)"
    assert_eq!(
        convert(format!("{}8", display_opt)),
        format!("{}(8)", eval_opt)
    );

    // convert("sqrt16+2") == "sqrt(16)+2"
    assert_eq!(
        convert(format!("{}16+2", display_opt)),
        format!("{}(16)+2", eval_opt)
    );

    // convert("4+8sqrt6") == "4+8sqrt(6)"
    assert_eq!(
        convert(format!("4+8{}6", display_opt)),
        format!("4+8{}(6)", eval_opt)
    );

    // convert("7/21sqrt(15)") == "7/21sqrt(15)"
    assert_eq!(
        convert(format!("7/21{}(15)", display_opt)),
        format!("7/21{}(15)", eval_opt)
    );

    // convert("sqrt4*sqrt9") == "sqrt(4)*sqrt(9)"
    assert_eq!(
        convert(format!("{}4*{}9", display_opt, display_opt)),
        format!("{}(4)*{}(9)", eval_opt, eval_opt)
    );

    // convert("sqrt4sqrt9") == "sqrt(4)sqrt(9)"
    assert_eq!(
        convert(format!("{}4{}9", display_opt, display_opt)),
        format!("{}(4){}(9)", eval_opt, eval_opt)
    );

    // convert("sqrt(sqrt16)") == "sqrt(sqrt(16))"
    assert_eq!(
        convert(format!("{}({}16)", display_opt, display_opt)),
        format!("{}({}(16))", eval_opt, eval_opt)
    );
}

#[test]
fn convert_decimal() {
    assert_eq!(convert("1.2".to_owned()), "1.2");
    assert_eq!(convert("-0.1".to_owned()), "-0.1");
}

#[test]
fn convert_add() {
    assert_eq!(convert("1+2".to_owned()), "1+2");
    assert_eq!(convert("0.1+1.2".to_owned()), "0.1+1.2");
    assert_eq!(convert("1+2+3".to_owned()), "1+2+3");
}

#[test]
fn convert_sub() {
    assert_eq!(convert("1-2".to_owned()), "1-2");
    assert_eq!(convert("-0.1-1.2".to_owned()), "-0.1-1.2");
    assert_eq!(convert("4--8".to_owned()), "4--8");
    assert_eq!(convert("4--8-2".to_owned()), "4--8-2");
    assert_eq!(convert("-(-1-(2))".to_owned()), "-(-1-(2))");
}

#[test]
fn convert_mul() {
    assert_eq!(convert("1*2".to_owned()), "1*2");
    assert_eq!(convert("-0.1*1.2".to_owned()), "-0.1*1.2");
    assert_eq!(convert("1*2*3".to_owned()), "1*2*3");
}

#[test]
fn convert_div() {
    assert_eq!(convert("1/2".to_owned()), "1/2");
    assert_eq!(convert("-0.1/1.2".to_owned()), "-0.1/1.2");
    assert_eq!(convert("1/2/3".to_owned()), "1/2/3");
}

#[test]
fn convert_fact() {
    assert_eq!(convert("2fact".to_owned()), "fact(2)");
    assert_eq!(convert("4+8fact6".to_owned()), "4+fact(8)*6");
    assert_eq!(convert("(-2*3)fact".to_owned()), "fact(-2*3)");
    assert_eq!(convert("4+16fact*4".to_owned()), "4+fact(16)*4");
    assert_eq!(convert("7/21fact(15)".to_owned()), "7/fact(21)(15)");
    assert_eq!(convert("4fact*9fact".to_owned()), "fact(4)*fact(9)");
    assert_eq!(convert("4fact9fact".to_owned()), "fact(4)fact(9)");
}

#[test]
fn convert_pi() {
    assert_eq!(convert("$pi$".to_owned()), "pi()");
    assert_eq!(convert("4-$pi$-4".to_owned()), "4-pi()-4");
    assert_eq!(convert("$pi$root$pi$".to_owned()), "root(pi(),pi())");
}

#[test]
fn convert_e() {
    assert_eq!(convert("$e$".to_owned()), "e()");
    assert_eq!(convert("4-$e$-4".to_owned()), "4-e()-4");
    assert_eq!(convert("$e$root$e$".to_owned()), "root(e(),e())");
}

#[test]
fn convert_expression() {
    assert_eq!(
        convert("sqrt4fact+2root(cos0.707)".to_owned()),
        "sqrt(fact(4))+root(2,cos(0.707))"
    );
    assert_eq!(
        convert("2pow($e$+5comb3)".to_owned()),
        "pow(2,e()+comb(5,3))"
    );
}

#[test]
fn convert_root() {
    bin_opt_template("root", "root")
}

#[test]
fn convert_sqrt() {
    unary_opt_template("sqrt", "sqrt")
}

#[test]
fn convert_pow() {
    bin_opt_template("pow", "pow")
}

#[test]
fn convert_generic_log() {
    bin_opt_template("logN", "log")
}

#[test]
fn convert_log10() {
    unary_opt_template("log", "log10")
}

#[test]
fn convert_ln() {
    unary_opt_template("ln", "ln")
}

#[test]
fn convert_sin() {
    unary_opt_template("sin", "sin")
}

#[test]
fn convert_cos() {
    unary_opt_template("cos", "cos")
}

#[test]
fn convert_tg() {
    unary_opt_template("tg", "tg")
}

#[test]
fn convert_cotg() {
    unary_opt_template("cotg", "cotg")
}

#[test]
fn convert_arcsin() {
    unary_opt_template("arcsin", "arcsin")
}

#[test]
fn convert_arccos() {
    unary_opt_template("arccos", "arccos")
}

#[test]
fn convert_arctg() {
    unary_opt_template("arctg", "arctg")
}

#[test]
fn convert_arccotg() {
    unary_opt_template("arccotg", "arccotg")
}

#[test]
fn convert_comb() {
    unary_opt_template("comb", "comb")
}
