use zoon::*;

use crate::theme;

pub fn header() -> impl Element {
    Row::with_tag(Tag::Nav)
        .s(Height::exact(64))
        .s(Align::new().left())
        .s(Padding::new().x(12))
        .item(logo())
}

fn logo() -> impl Element {
    Link::new()
        .to("/")
        .label(Row::new()
            .item(logo_svg())
            .item(Row::new()
                .s(Padding::new().left(12))
                .s(Font::new()
                    .size(24)
                    .weight(FontWeight::Bold)
                    .color_signal(theme::primary_text_color())
                    .family([
                        FontFamily::new("Futura"),
                        FontFamily::new("Century Gothic"),
                        FontFamily::new("CenturyGothic"),
                        FontFamily::new("Apple Sans"),
                        FontFamily::SansSerif,
                    ]))
                .item("Curiosity Driven")
            )
        )
}

fn logo_svg() -> RawSvgEl<web_sys::SvgsvgElement> {
    RawSvgEl::from_markup(include_str!("../../public/logo.svg"))
        .unwrap_throw()
        .attr("width", "50")
        .attr("height", "40")
        .style_signal("fill", theme::primary_text_color())
}
