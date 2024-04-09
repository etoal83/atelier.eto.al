mod header;
mod theme;

use header::header;
use theme::THEME;
use zoon::*;


// ------ Router ------

#[route]
#[derive(Clone, Debug)]
enum Route {
    #[route()]
    Home,

    #[route("articles")]
    Articles,
    #[route("articles", slug)]
    Article { slug: String },
}

pub static ROUTER: Lazy<Router<Route>> = lazy::default();


// ------ Page ------

#[derive(Clone)]
struct Page;

impl Page {
    fn new() -> impl Element {
        Self.root()
    }

    fn root(&self) -> impl Element {
        self.header_layout()
    }

    fn header_layout(&self) -> impl Element {
        Stack::new()
            .s(Width::fill())
            .s(Height::screen())
            .layer(Column::new()
                .s(Width::exact(1200))
                .s(Height::screen())
                .s(Align::center())
                .item(header())
            )
            .layer(Column::new()
                .s(Align::center())
                .s(Transform::new().move_up(20))
                .s(Gap::both(20))
                .s(Font::new().color_signal(theme::primary_text_color()).size(30))
                .item(self.page_content())
            )
    }

    fn page_content(&self) -> impl Element {
        El::new().child_signal(ROUTER.route().signal_cloned().map(move |route| {
            match route {
                NoRoute => None,
                UnknownRoute => El::new().child("404").unify_option(),
                KnownRoute(route) => match route {
                    Route::Home => El::new().child("Home").unify_option(),
                    Route::Articles => El::new().child("Articles").unify_option(),
                    Route::Article { slug } => El::new().child(format!("Article: {}", slug)).unify_option(),
                }
            }
        }))
    }
}


// ------ Main (Init) ------

pub fn init() -> impl Element {
    Lazy::force(&ROUTER);
    Lazy::force(&THEME);

    Page::new()
}