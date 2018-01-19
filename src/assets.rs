pub mod css {
    pub const MAIN: &'static str = include_str!("../assets/css/main.css");
    pub const RESET: &'static str = include_str!("../assets/css/reset.css");
    pub const STYLE: &'static str = include_str!("../assets/css/style.css");
}

pub mod html {
    pub const INDEX: &'static str = include_str!("../assets/html/index.html");
}

pub mod js {
    pub const MAIN: &'static str = include_str!("../assets/js/main.js");
    pub const HEADROOM: &'static str = include_str!("../assets/js/Headroom.js");
    pub const MODERNIZR: &'static str = include_str!("../assets/js/modernizr.js");
}

pub mod sql {
    pub const BASE_001_20180118: &'static str = include_str!("../assets/sql/001_2018-01-18.sql");
}
