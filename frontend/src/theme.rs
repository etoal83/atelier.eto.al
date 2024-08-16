use catppuccin::{ColorName, Flavor, PALETTE};
use icondata::{ImCheckboxUnchecked, ImCheckboxChecked};
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
            Theme::Light => { zoon::println!("Light => Dark"); Theme::Dark },
            Theme::Dark => { zoon::println!("Dark => Light"); Theme::Light },
        })
    });
    Mutable::new(Theme::Dark)
});


// ------ View ------

pub fn theme_toggle_switch() -> impl Element {
    Checkbox::new()
        .id("theme-toggle-switch")
        .label_hidden("Dark mode")
        .icon(|checked| checkbox_icon(checked.signal()))
        .checked_signal(THEME.signal().map(|theme| theme == Theme::Dark))
        .on_change(|_| emit(ToggleThemeClicked))
}

fn checkbox_icon(checked_signal: MutableSignal<bool>) -> impl Element {
    static CHECKED: &str = ImCheckboxChecked.data;
    static UNCHECKED: &str = ImCheckboxUnchecked.data;

    El::new()
        .s(Width::exact(20))
        .s(Height::exact(20))
        .child(RawSvgEl::new("svg")
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
