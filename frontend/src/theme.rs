use catppuccin::{Colour, Flavour};
use hsluv::hex_to_hsluv;
use zoon::{once_cell::sync::Lazy, *};

#[derive(Clone, Copy, Debug)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Clone, Copy)]
struct ToggleThemeClicked;


pub static THEME: Lazy<Mutable<Theme>> = Lazy::new(|| {
    on(|ToggleThemeClicked| {
        THEME.update(|theme| match theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        })
    });
    Mutable::new(Theme::Dark)
});

fn hsluv(colour: Colour) -> HSLuv {
    let (h, s, l) = hex_to_hsluv(&colour.hex());

    HSLuv::new_unchecked(h, s, l, 100.)
}

// ------ colors ------

macro_rules! assign_color {
    ($color:ident => $schema:ident) => {
        pub fn $color() -> impl Signal<Item = HSLuv> {
            THEME.signal().map(|theme| match theme {
                Theme::Light => hsluv(Flavour::Latte.$schema()),
                Theme::Dark => hsluv(Flavour::Mocha.$schema()),
            })
        }
    };
}

assign_color!(primary_text_color => text);
assign_color!(secondary_text_color => subtext0);
assign_color!(primary_accent_color => mauve);
assign_color!(border_color => overlay0);
assign_color!(primary_background_color => mantle);
assign_color!(secondary_background_color => crust);
assign_color!(hovered_background_color => surface0);