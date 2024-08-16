mod cms;
mod header;
mod theme;
mod shaders;

use header::header;
use theme::THEME;
use zoon::*;

// ------ Router ------

#[route]
#[derive(Clone, Debug)]
pub enum Route {
    #[route()]
    Home,
    #[route("shaders")]
    Shaders,
    #[route("shaders", title)]
    ShaderPage { title: String },
}

pub static ROUTER: Lazy<Router<Route>> = lazy::default();

// ------ Layout ------

#[derive(Clone, Copy, Debug, PartialEq)]
enum Layout {
    Desktop,
    Mobile,
}

pub static SCREEN_WIDTH: Lazy<Mutable<U32Width>> = lazy::default();

fn layout_signal() -> impl Signal<Item = Layout> {
    SCREEN_WIDTH.signal().map(|width| match width >= 768 {
        true => Layout::Desktop,
        false => Layout::Mobile,
    }).dedupe()
}

fn mobile_layout_signal() -> impl Signal<Item = bool> {
    layout_signal().map(|layout| layout == Layout::Mobile).dedupe()
}

// ------ View ------

fn root() -> impl Element {
    Stack::new()
        .s(Width::fill())
        .s(Height::screen())
        .s(Font::new()
            .family([
                FontFamily::new("Murecho"),
                FontFamily::SansSerif,
            ])
            .color_signal(theme::primary_text_color())
        )
        .s(Background::new().color_signal(theme::primary_background_color()))
        .on_viewport_size_change(|width, _| SCREEN_WIDTH.set_neq(width))
        // .layer(shaders::global_canvas())
        .layer(
            Column::new()
                .s(Width::fill().max(1200))
                .s(Padding::new().x_signal(mobile_layout_signal().map_bool(|| 20, || 40)))
                .s(Align::new().top().center_x())
                .item(header())
                .item(page_content())
        )
}

fn page_content() -> impl Element {
    El::with_tag(Tag::Main)
        .child_signal(ROUTER.route().signal_cloned().map(|route| match route {
            NoRoute => None,
            UnknownRoute => El::new().child("404").unify_option(),
            KnownRoute(route) => match route {
                Route::Home => shaders::page_content(None).unify_option(),
                Route::Shaders => shaders::page_content(None).unify_option(),
                Route::ShaderPage { title } => shaders::page_content(Some(title)).unify_option(),
            }
        }))
}

// ------ Main (Init) ------

pub fn init() -> impl Element {
    Lazy::force(&ROUTER);
    Lazy::force(&THEME);

    root()
}
