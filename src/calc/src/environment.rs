use druid::{theme, Color, Env, Key};

pub fn set_dark_envs(env: &mut Env) {
    env.set(theme::WINDOW_BACKGROUND_COLOR, Color::grey8(20));
    env.set(theme::TEXT_COLOR, Color::grey8(230));
    env.set(theme::DISABLED_TEXT_COLOR, Color::grey8(80));
    env.set(
        Key::<Color>::new("calc.active_textcolor"),
        Color::grey8(100),
    );
}

pub fn set_light_envs(env: &mut Env) {
    env.set(theme::WINDOW_BACKGROUND_COLOR, Color::grey8(230));
    env.set(theme::TEXT_COLOR, Color::grey8(40));
    env.set(theme::DISABLED_TEXT_COLOR, Color::grey8(170));
    env.set(
        Key::<Color>::new("calc.active_textcolor"),
        Color::grey8(200),
    );
}

pub fn set_digit_btn_envs(env: &mut Env) {
    env.set(
        Key::<Color>::new("calc.Dark.digit_btn.background"),
        Color::grey8(50),
    );
    env.set(
        Key::<Color>::new("calc.Light.digit_btn.background"),
        Color::grey8(220),
    );

    env.set(
        Key::<Color>::new("calc.Dark.digit_btn.hover"),
        Color::grey8(70),
    );
    env.set(
        Key::<Color>::new("calc.Light.digit_btn.hover"),
        Color::grey8(200),
    );

    env.set(
        Key::<Color>::new("calc.Dark.digit_btn.active"),
        Color::grey8(100),
    );
    env.set(
        Key::<Color>::new("calc.Light.digit_btn.active"),
        Color::grey8(180),
    );
}

pub fn set_func_btn_envs(env: &mut Env) {
    env.set(
        Key::<Color>::new("calc.Dark.func_btn.background"),
        Color::grey8(30),
    );
    env.set(
        Key::<Color>::new("calc.Light.func_btn.background"),
        Color::grey8(200),
    );

    env.set(
        Key::<Color>::new("calc.Dark.func_btn.hover"),
        Color::grey8(50),
    );
    env.set(
        Key::<Color>::new("calc.Light.func_btn.hover"),
        Color::grey8(180),
    );

    env.set(
        Key::<Color>::new("calc.Dark.func_btn.active"),
        Color::grey8(80),
    );
    env.set(
        Key::<Color>::new("calc.Light.func_btn.active"),
        Color::grey8(160),
    );
}

pub fn set_operation_btn_envs(env: &mut Env) {
    env.set(
        Key::<Color>::new("calc.Dark.operation_btn.background"),
        Color::grey8(35),
    );
    env.set(
        Key::<Color>::new("calc.Light.operation_btn.background"),
        Color::grey8(210),
    );

    env.set(
        Key::<Color>::new("calc.Dark.operation_btn.hover"),
        Color::grey8(60),
    );
    env.set(
        Key::<Color>::new("calc.Light.operation_btn.hover"),
        Color::grey8(190),
    );

    env.set(
        Key::<Color>::new("calc.Dark.operation_btn.active"),
        Color::grey8(90),
    );
    env.set(
        Key::<Color>::new("calc.Light.operation_btn.active"),
        Color::grey8(170),
    );
}
