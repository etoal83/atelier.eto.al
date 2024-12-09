use zoon::*;

use crate::{theme, SCREEN_WIDTH};

pub fn header() -> impl Element {
    Row::with_tag(Tag::Header)
        .s(Width::fill())
        .s(Height::exact(80)) // or 64
        .s(Align::new().left().center_y())
        .item(logo())
        .item(theme::theme_toggle_switch())
}

fn logo() -> impl Element {
    Link::new().to("/").label(
        Row::new()
            .s(AlignContent::new().left())
            .s(Gap::new().x(14))
            .item(logo_svg())
            .item(El::new()
                .child_signal(SCREEN_WIDTH.signal().map(|w| if w < 360 {
                    sitename_compact_svg()
                } else {
                    sitename_jp_svg()
                })))
    )
}

fn logo_svg() -> RawSvgEl<web_sys::SvgsvgElement> {
    RawSvgEl::from_markup(include_str!("../../public/logo.svg"))
        .unwrap_throw()
        .attr("width", "50")
        .attr("height", "40")
        .attr_signal("fill", theme::primary_text_color_hex())
}

fn sitename_jp_svg() -> RawSvgEl<web_sys::SvgsvgElement> {
    RawSvgEl::from_markup(include_str!("../../public/sitename_jp.svg"))
        .unwrap_throw()
        .attr("width", "206")
        .attr("height", "22")
        .attr_signal("fill", theme::primary_text_color_hex())
}

// fn sitename_en_svg() -> RawSvgEl<web_sys::SvgsvgElement> {
//     RawSvgEl::from_markup(include_str!("../../public/sitename_en.svg"))
//         .unwrap_throw()
//         .attr("width", "237")
//         .attr("height", "18")
//         .attr_signal("fill", theme::primary_text_color_hex())
// }

fn sitename_compact_svg() -> RawSvgEl<web_sys::SvgsvgElement> {
    RawSvgEl::from_markup(include_str!("../../public/sitename_compact.svg"))
        .unwrap_throw()
        .attr("width", "80")
        .attr("height", "28")
        .attr_signal("fill", theme::primary_text_color_hex())
}
