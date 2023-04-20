//! Define internal tests for the calculator state.

use crate::Opt;

use super::{ExprManager, Btn, ToExpr};

// Wrapper for testing the get_eval_str() method.
fn convert(btn_stack: Vec<Btn>) -> String {
    let mut expr_man = ExprManager::new();
    expr_man.btn_stack = btn_stack;
    expr_man.get_eval_str()
}

// Convert given string to sequence of `PressedButton` enums.
// All `@` characters will be replaced with `rep`.
fn to_opt_seq(str: &str, rep: Vec<&Opt>) -> Vec<Btn> {
    let mut btns = Vec::new();
    let mut n_rep = 0;
    let mut expect_const = false;
    let mut constant = String::new();

    for c in str.chars() {
        if expect_const {
            if c == '$' {
                btns.push(Btn::Const(constant.clone()));
                constant.clear();
                expect_const = false;
            } else {
                constant += &c.to_string();
            }
            continue;
        }

        let num = c.to_digit(10).unwrap_or(10) as u8;
        if (0..9).contains(&num) {
            btns.push(Btn::Num(num));
            continue;
        }

        btns.push(match c {
            '+' => Btn::BinOpt(Opt::Add),
            '-' => Btn::BinOpt(Opt::Sub),
            '*' => Btn::BinOpt(Opt::Mul),
            '/' => Btn::BinOpt(Opt::Div),
            '!' => Btn::BinOpt(Opt::Fact),
            '(' => Btn::BracketLeft,
            ')' => Btn::BracketRight,
            '.' => Btn::Comma,
            '@' => {
                n_rep += 1;
                Btn::BinOpt((*rep.get(n_rep - 1).unwrap()).to_owned())
            },
            '$' => {
                expect_const = true;
                continue
            }
            _ => panic!("Invalid character '{}'", c),
        });
    }

    btns
}

// Generic tests for binary operations
fn bin_opt_template(display_opt: &Opt, eval_opt: &Opt) {
    let eval = eval_opt.to_expr().unwrap().eval;

    // convert("2*3root8") == "2*root(3,8)"
    assert_eq!(
        convert(to_opt_seq("2*3@8", Vec::from([display_opt]))),
        format!("2*{}(3,8)", eval)
    );

    // convert("4+24root6") == "4+root(24,6)"
    assert_eq!(
        convert(to_opt_seq("4+24@6", Vec::from([display_opt]))),
        format!("4+{}(24,6)", eval)
    );

    // convert("24root62") == "root(24,62)"
    assert_eq!(
        convert(to_opt_seq("24@62", Vec::from([display_opt]))),
        format!("{}(24,62)", eval)
    );

    // convert("-1+(10)root(-2*3)") == "-1+root(10,-2*3)"
    assert_eq!(
        convert(to_opt_seq("-1+(10)@(-2*3)", Vec::from([display_opt]))),
        format!("-1+{}(10,-2*3)", eval)
    );

    // convert("3root8*4root16") == "root(3,8)*root(4,16)"
    assert_eq!(
        convert(to_opt_seq("3@8*4@16", Vec::from([display_opt]))),
        format!("{}(3,8)*{}(4,16)", eval, eval)
    );

    // convert("3root(4root16)") == "root(3,root(4,16))"
    assert_eq!(
        convert(to_opt_seq("3@(4@16)", Vec::from([display_opt]))),
        format!("{}(3,{}(4,16))", eval, eval)
    );
}

// Generic tests for unary operations
fn unary_opt_template(display_opt: &Opt, eval_opt: &Opt) {
    let eval = eval_opt.to_expr().unwrap().eval;
    // convert("sqrt8") == "sqrt(8)"
    assert_eq!(
        convert(to_opt_seq("@8", Vec::from([display_opt]))),
        format!("{}(8)", eval)
    );

    // convert("sqrt16+2") == "sqrt(16)+2"
    assert_eq!(
        convert(to_opt_seq("@16+2", Vec::from([display_opt]))),
        format!("{}(16)+2", eval)
    );

    // convert("4+8sqrt6") == "4+8sqrt(6)"
    assert_eq!(
        convert(to_opt_seq("4+8@6", Vec::from([display_opt]))),
        format!("4+8{}(6)", eval)
    );

    // convert("7/21sqrt(15)") == "7/21sqrt(15)"
    assert_eq!(
        convert(to_opt_seq("7/21@(15)", Vec::from([display_opt]))),
        format!("7/21{}(15)", eval)
    );

    // convert("sqrt4*sqrt9") == "sqrt(4)*sqrt(9)"
    assert_eq!(
        convert(to_opt_seq("@4*@9", Vec::from([display_opt]))),
        format!("{}(4)*{}(9)", eval, eval)
    );

    // convert("sqrt4sqrt9") == "sqrt(4)sqrt(9)"
    assert_eq!(
        convert(to_opt_seq("@4@9", Vec::from([display_opt]))),
        format!("{}(4){}(9)", eval, eval)
    );

    // convert("sqrt(sqrt16)") == "sqrt(sqrt(16))"
    assert_eq!(
        convert(to_opt_seq("@(@16)", Vec::from([display_opt]))),
        format!("{}({}(16))", eval, eval)
    );
}

#[test]
fn convert_decimal() {
    assert_eq!(convert(to_opt_seq("1.2", Vec::new())), "1.2");
    assert_eq!(convert(to_opt_seq("-0.1", Vec::new())), "-0.1");
}

#[test]
fn convert_add() {
    assert_eq!(convert(to_opt_seq("1+2", Vec::new())), "1+2");
    assert_eq!(convert(to_opt_seq("0.1+1.2", Vec::new())), "0.1+1.2");
    assert_eq!(convert(to_opt_seq("1+2+3", Vec::new())), "1+2+3");
}

#[test]
fn convert_sub() {
    assert_eq!(convert(to_opt_seq("1-2", Vec::new())), "1-2");
    assert_eq!(convert(to_opt_seq("-0.1-1.2", Vec::new())), "-0.1-1.2");
    assert_eq!(convert(to_opt_seq("4--8", Vec::new())), "4--8");
    assert_eq!(convert(to_opt_seq("4--8-2", Vec::new())), "4--8-2");
    assert_eq!(convert(to_opt_seq("-(-1-(2))", Vec::new())), "-(-1-(2))");
}

#[test]
fn convert_mul() {
    assert_eq!(convert(to_opt_seq("1*2", Vec::new())), "1*2");
    assert_eq!(convert(to_opt_seq("-0.1*1.2", Vec::new())), "-0.1*1.2");
    assert_eq!(convert(to_opt_seq("1*2*3", Vec::new())), "1*2*3");
}

#[test]
fn convert_div() {
    assert_eq!(convert(to_opt_seq("1/2", Vec::new())), "1/2");
    assert_eq!(convert(to_opt_seq("-0.1/1.2", Vec::new())), "-0.1/1.2");
    assert_eq!(convert(to_opt_seq("1/2/3", Vec::new())), "1/2/3");
}

#[test]
fn convert_fact() {
    assert_eq!(convert(to_opt_seq("2!", Vec::new())), "2!");
    assert_eq!(convert(to_opt_seq("4+8!6", Vec::new())), "4+8!*6");
    assert_eq!(convert(to_opt_seq("(-2*3)!", Vec::new())), "(-2*3)!");
    assert_eq!(convert(to_opt_seq("4+16!*4", Vec::new())), "4+16!*4");
    assert_eq!(convert(to_opt_seq("4!*9!", Vec::new())), "4!*9!");
    assert_eq!(convert(to_opt_seq("4!9!", Vec::new())), "4!9!");
}

#[test]
fn convert_pi() {
    assert_eq!(convert(to_opt_seq("$pi$", Vec::new())), "pi()");
    assert_eq!(convert(to_opt_seq("4-$pi$-4", Vec::new())), "4-pi()-4");
    assert_eq!(convert(to_opt_seq("$pi$@$pi$", Vec::from([&Opt::Root]))), "root(pi(),pi())");
}

#[test]
fn convert_e() {
    assert_eq!(convert(to_opt_seq("$e$", Vec::new())), "e()");
    assert_eq!(convert(to_opt_seq("4-$e$-4", Vec::new())), "4-e()-4");
    assert_eq!(convert(to_opt_seq("$e$@$e$", Vec::from([&Opt::Root]))), "root(e(),e())");
}

#[test]
fn convert_expression() {
    assert_eq!(
        convert(to_opt_seq("@4@+2@(@0.707)", Vec::from([&Opt::Sqrt, &Opt::Fact, &Opt::Root, &Opt::Cos]))),
        "sqrt(fact(4))+root(2,cos(0.707))"
    );
    assert_eq!( 
        convert(to_opt_seq("2@($e$+5@3)", Vec::from([&Opt::Pow, &Opt::Comb]))),
        "pow(2,e()+comb(5,3))"
    );
}

#[test]
fn convert_root() {
    bin_opt_template(&Opt::Root, &Opt::Root);
}

#[test]
fn convert_sqrt() {
    unary_opt_template(&Opt::Sqrt, &Opt::Sqrt)
}

#[test]
fn convert_pow() {
    bin_opt_template(&Opt::Pow, &Opt::Pow)
}

#[test]
fn convert_generic_log() {
    bin_opt_template(&Opt::LogN, &Opt::LogN)
}

#[test]
fn convert_log10() {
    unary_opt_template(&Opt::Log, &Opt::Log)
}

#[test]
fn convert_ln() {
    unary_opt_template(&Opt::Ln, &Opt::Ln)
}

#[test]
fn convert_sin() {
    unary_opt_template(&Opt::Sin, &Opt::Sin)
}

#[test]
fn convert_cos() {
    unary_opt_template(&Opt::Cos, &Opt::Cos)
}

#[test]
fn convert_tg() {
    unary_opt_template(&Opt::Tg, &Opt::Tg)
}

#[test]
fn convert_cotg() {
    unary_opt_template(&Opt::Cotg, &Opt::Cotg)
}

#[test]
fn convert_arcsin() {
    unary_opt_template(&Opt::Arcsin, &Opt::Arcsin)
}

#[test]
fn convert_arccos() {
    unary_opt_template(&Opt::Arccos, &Opt::Arccos)
}

#[test]
fn convert_arctg() {
    unary_opt_template(&Opt::Arctg, &Opt::Arctg)
}

#[test]
fn convert_arccotg() {
    unary_opt_template(&Opt::Arccotg, &Opt::Arccotg)
}

#[test]
fn convert_comb() {
    unary_opt_template(&Opt::Comb, &Opt::Comb)
}
