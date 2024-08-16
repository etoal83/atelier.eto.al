use catppuccin::{ColorName, Flavor, PALETTE};
use icondata::{RiMoonClearWeatherLine, RiSunWeatherLine};
use zoon::*;

// ------ Types ------

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    fn flavor(&self) -> &Flavor {
        match self {
            Theme::Light => &PALETTE.latte,
            Theme::Dark => &PALETTE.mocha,
        }
    }
}

// ------ Events ------

#[derive(Clone, Copy)]
struct ToggleThemeClicked;


// ------ States ------

pub static THEME: Lazy<Mutable<Theme>> = Lazy::new(|| {
    on(|ToggleThemeClicked| {
        THEME.update(|theme| match theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        })
    });
    Mutable::new(Theme::Dark)
});


// ------ View ------

pub fn theme_toggle_switch() -> impl Element {
    Checkbox::new()
        .update_raw_el(|el| el.style("margin-left", "auto"))
        .id("theme-toggle-switch")
        .label_hidden("Dark mode")
        .icon(|checked| checkbox_icon(checked.signal()))
        .checked_signal(THEME.signal().map(|theme| theme == Theme::Dark))
        .on_change(|_| emit(ToggleThemeClicked))
}

fn checkbox_icon(checked_signal: MutableSignal<bool>) -> impl Element {
    static CHECKED: &str = RiMoonClearWeatherLine.data;
    static UNCHECKED: &str = RiSunWeatherLine.data;

    El::new()
        .s(Width::exact(36))
        .s(Height::exact(36))
        .s(AlignContent::center())
        .s(RoundedCorners::all(4))
        .child(RawSvgEl::new("svg")
            .style("width", "24px")
            .attr("viewBox", "0 0 24 24")
            // .attr("preserveAspectRatio", "xMidYMid")
            .attr_signal("fill", primary_text_color_hex())
            .inner_markup_signal(checked_signal.map_bool(|| CHECKED, || UNCHECKED)))
}

// ------ Color assignment ------

macro_rules! assign_color {
    ($color:ident => $schema:ident) => {
        paste! {
            #[allow(dead_code)]
            pub fn $color() -> impl Signal<Item = String> {
                // Redirects to <$color _hex>() so signal item is temporarily `String`
                // TODO: use MZ color system when it gets stable
                [< $color _hex>]()

                // THEME.signal().map(|theme| {
                //     let Hsl { h, s, l } = theme.flavor().get_color(ColorName::$schema).hsl;
                //     HSLuv::hsl(h, s, l)
                // })
            }

            pub fn [<$color _hex>]() -> impl Signal<Item = String> {
                THEME.signal().map(|theme| {
                    let hex = theme.flavor().get_color(ColorName::$schema).hex;
                    hex.to_string()
                })
            }

            pub fn [<dark_ $color>]() -> String {
                PALETTE.mocha.get_color(ColorName::$schema).hex.to_string()
            }

            pub fn [<light_ $color>]() -> String {
                PALETTE.latte.get_color(ColorName::$schema).hex.to_string()
            }
        }
    };
}

assign_color!(primary_text_color => Text);
assign_color!(secondary_text_color => Subtext0);
assign_color!(primary_accent_color => Mauve);
assign_color!(border_color => Overlay0);
assign_color!(primary_background_color => Base);
assign_color!(secondary_background_color => Crust);
assign_color!(hovered_background_color => Surface0);
