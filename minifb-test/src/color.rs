#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        (($r << 16) | ($g << 8) | $b) as u32
    };
}

#[macro_export]
macro_rules! rgba {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        (($r << 24) | ($g << 16) | ($b << 8) | $a) as u32
    };
}

#[macro_export]
macro_rules! rgb_red {
    ($color:expr) => {
        ($color >> 16) & 0xFF
    };
}

#[macro_export]
macro_rules! rgb_green {
    ($color:expr) => {
        ($color >> 8) & 0xFF
    };
}

#[macro_export]
macro_rules! rgb_blue {
    ($color:expr) => {
        $color & 0xFF
    };
}

#[macro_export]
macro_rules! rgba_red {
    ($color:expr) => {
        ($color >> 24) & 0xFF
    };
}

#[macro_export]
macro_rules! rgba_green {
    ($color:expr) => {
        ($color >> 16) & 0xFF
    };
}

#[macro_export]
macro_rules! rgba_blue {
    ($color:expr) => {
        ($color >> 8) & 0xFF
    };
}

#[macro_export]
macro_rules! rgba_alpha {
    ($color:expr) => {
        ((($color & 0xFF) as f32) / 255.0) as f32
    };
}

pub fn rgba_from_rgb(color_rgb: u32, alpha: f32) -> u32 {
    rgba!(rgb_red!(color_rgb), rgb_green!(color_rgb), rgb_blue!(color_rgb), (alpha * 255.0) as u32)
}

pub fn rgba_merge(bg_rgb: u32, fg_rgba: u32) -> u32 {
    let fg_alpha = rgba_alpha!(fg_rgba);
    let bg_alpha = 1.0 - fg_alpha;
    let r = (rgb_red!(bg_rgb) as f32 * bg_alpha + rgba_red!(fg_rgba) as f32 * fg_alpha) as u32;
    let g = (rgb_green!(bg_rgb) as f32 * bg_alpha + rgba_green!(fg_rgba) as f32 * fg_alpha) as u32;
    let b = (rgb_blue!(bg_rgb) as f32 * bg_alpha + rgba_blue!(fg_rgba) as f32 * fg_alpha) as u32;
    rgb!(r, g, b)
}
