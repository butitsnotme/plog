use assets;

#[get("/")]
fn index() -> &'static str {
    assets::html::INDEX
}

#[get("/css/main.css")]
fn get_css_main() -> &'static str {
    assets::css::MAIN
}

#[get("/css/reset.css")]
fn get_css_reset() -> &'static str {
    assets::css::RESET
}

#[get("/css/style.css")]
fn get_css_style() -> &'static str {
    assets::css::STYLE
}

#[get("/js/Headroom.js")]
fn get_js_headroom() -> &'static str {
    assets::js::HEADROOM
}

#[get("/js/main.js")]
fn get_js_main() -> &'static str {
    assets::js::MAIN
}

#[get("/js/modernizr.js")]
fn get_js_modernizr() -> &'static str {
    assets::js::MODERNIZR
}
