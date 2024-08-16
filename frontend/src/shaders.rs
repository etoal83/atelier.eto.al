mod base;

use crate::{cms, theme, mobile_layout_signal};
use heck::{ToUpperCamelCase, ToSnakeCase, ToTitleCase};
use std::cmp::max;
use std::str::FromStr;
use std::collections::VecDeque;
use strum::{Display, EnumString, EnumIter, IntoEnumIterator};
use wgpu::*;
#[allow(unused_imports)]
use zoon::{web_sys::HtmlCanvasElement, println, eprintln, *};


// ------ Page content ------

pub fn page_content(slug: Option<String>) -> impl Element {
    if let Some(slug) = slug {
        let Ok(slug) = Slug::from_str(&slug) else {
            return El::new().child("Shader not found").unify()
        };
        shader_page(slug).unify()
    } else {
        shader_gallery().unify()
    }
}

// /shaders
fn shader_gallery() -> impl Element {
    let width: Mutable<U32Width> = Mutable::new(0);

    Row::new()
        .s(Gap::both(16))
        .multiline()
        .on_viewport_size_change(clone!((width) move |w, _| width.set_neq(w)))
        .items(Slug::iter()
            .rev()
            .map(|slug| Link::new()
                .update_raw_el(|el| el
                    .style("flex-basis", "180px")
                    .style_signal("min-width", width.signal().map(|w| w < 376).map_true(|| "100%"))
                )
                .to(slug.page_url())
                .label(Image::new()
                    .s(Width::fill())
                    .url(slug.thumbnail_url())
                    .description(format!("Shader work: {}", slug.title())))
        ))
}

// /shaders/{slug}
fn shader_page(slug: Slug) -> impl Element {
    let content_id = slug.to_string();
    let (shader_title, title_signal) = Mutable::new_and_signal_cloned("Loading...".to_string());
    let shader_description: Mutable<Option<cms::ContentI18ned>> = Mutable::new(None);
    let description_buffer: Mutable<String> = Mutable::new("".to_string());
    let (displayed_description, displayed_description_signal) = Mutable::new_and_signal_cloned("".to_string());
    let (is_typing, is_typing_signal) = Mutable::new_and_signal(false);
    let blink_oscillator = Oscillator::new(Duration::seconds(1));
    blink_oscillator.cycle_wrap();
    let blinking_color_signal = map_ref! {
        let theme = theme::THEME.signal(),
        let oscillator = blink_oscillator.signal() => {
            match theme {
                theme::Theme::Light => if *oscillator > 0.5 { theme::light_primary_text_color() } else { theme::light_primary_background_color() },
                theme::Theme::Dark => if *oscillator > 0.5 { theme::dark_primary_text_color() } else { theme::dark_primary_background_color() },
            }
        }
    };

    run_once!(|| {
        global_styles()
            .style_group(StyleGroup::new(".description > p a").style_signal("color", theme::primary_accent_color()));
    });

    // Typing animation for shader description
    Task::start(clone!((shader_description, description_buffer, displayed_description, is_typing) async move {
        description_buffer
            .signal_cloned()
            .for_each_sync(move |s| {
                let mut v = VecDeque::from_iter(s.chars());
                Task::start(clone!((shader_description, displayed_description, is_typing) async move {
                    is_typing.set(true);

                    while let Some(c) = v.pop_front() {
                        displayed_description.lock_mut().push(c);
                        Timer::sleep(10).await;
                    }

                    displayed_description.set(match shader_description.get_cloned() {
                        Some(cms::ContentI18ned { ja, en: _ }) => format!("{}\n", ja.clone()),
                        _ => "".to_string(),
                    });
                    is_typing.set(false);
                }));
            })
            .await
    }));

    // fetch shader contenr from CMS
    Task::start(async move {
        match cms::fetch_shader_content(content_id).await {
            Ok(cms::ShaderContent { title, description }) => {
                shader_title.set_neq(title);
                shader_description.set(Some(description.clone()));
                description_buffer.set_neq(markup_to_string(description.ja.clone()));
            },
            Err(err) => {
                eprintln!("Failed to fetch shader content: {:?}", err);
            },
        }
    });

    Column::new()
        .s(Width::fill())
        .s(Padding::new().x_signal(mobile_layout_signal().map_bool(|| 0, || 16)))
        .s(Gap::new().y(20))
        .item(Canvas::new()
            .width(300)
            .height(150)
            .s(Width::fill().max(800))
            .s(Height::exact_signal(CANVAS_WIDTH.signal().map(|w| {
                if w < 568 { w } else { (w as f32 * 9.0 / 16.0) as u32 }
            })))
            .update_raw_el(|el| el
                .on_resize(|w, h| {
                    CANVAS_WIDTH.set_neq(max(w, 1));
                    CANVAS_HEIGHT.set_neq(max(h, 1));
                })
                .event_handler(|event: events_extra::PointerMove| {
                    CANVAS_POINTER_POSITION.set_neq((event.offset_x(), event.offset_y()));
                })
            )
            .after_insert(move |canvas| Task::start(run_shader(canvas, slug)))
            .after_remove(|_| ANIMATION_LOOP.set(None)))
        .item(Paragraph::with_tag(Tag::H1)
            .s(Font::new()
                .weight(FontWeight::Bold)
                .size(24))
            .content_signal(title_signal))
        .item(Paragraph::new().contents(vec![
            El::new()
                .update_raw_el(|el| el
                    .class("description")
                    .inner_markup_signal(displayed_description_signal)),
            El::new()
                .s(Height::exact(16))
                .s(Background::new().color_signal(theme::primary_text_color()))
                .update_raw_el(clone!((is_typing) |el| el.inner_markup_signal(is_typing.signal().map_bool(|| "//", || "")))),
            El::new()
                .s(Height::exact(40))
                .s(Borders::all_signal(map_ref! {
                    let theme = theme::THEME.signal(),
                    let oscillator = blink_oscillator.signal() => {
                        match theme {
                            theme::Theme::Light => if *oscillator > 0.5 { Border::new().color(theme::light_primary_text_color()) } else { Border::new().color(theme::light_primary_background_color()) },
                            theme::Theme::Dark => if *oscillator > 0.5 { Border::new().color(theme::dark_primary_text_color()) } else { Border::new().color(theme::dark_primary_background_color()) },
                        }
                    }
                }))
                .s(RoundedCorners::all(2))
                .s(Font::new().size(12).color_signal(blinking_color_signal))
                .update_raw_el(|el| el.inner_markup_signal(is_typing_signal.map_bool(|| "", || " ⏎ "))),
        ])).after_remove(move |_| drop(blink_oscillator))
}

fn markup_to_string(markup: String) -> String {
    let frag = scraper::Html::parse_document(&markup);
    let mut plain = String::new();
    for node in frag.tree {
        if let scraper::node::Node::Text(text) = node {
            plain.push_str(&text.text);
        }
    }

    plain
}

// ------ Slug ------

macro_rules! shader_mod {
    (
        enum Slug {
            $($slug:ident),* $(,)?
        }
    ) => {
        paste! {
            $(mod [< $slug:snake >];)*

            #[derive(Debug, Display, PartialEq, EnumString, EnumIter)]
            #[strum(serialize_all = "kebab-case")]
            enum Slug {
                $($slug,)*
            }

            async fn run_shader(canvas: HtmlCanvasElement, slug: Slug) {
                match slug.to_string().to_upper_camel_case().as_str() {
                    $(
                        stringify!($slug) => [< $slug:snake >]::ShaderWork::run(canvas).await,
                    )*
                    _ => unreachable!(),
                }
            }
        }
    }
}

shader_mod! {
    enum Slug {
        HelloTriangle,
        HelloTriangleWithVertexBuffer,
        GlslsandboxExample,
        // *** Add new shaders here ***
    }
}

impl Slug {
    fn title(&self) -> String {
        self.to_string().to_title_case()
    }

    fn page_url(&self) -> String {
        format!("/shaders/{}", self.to_string())
    }

    fn thumbnail_url(&self) -> String {
        public_url(format!("shaders/thumbnail/{}.webp", self.to_string().to_snake_case()))
    }
}

// ------ Shared by shader works ------

static CANVAS_WIDTH: Lazy<Mutable<U32Width>> = Lazy::new(|| Mutable::new(40));
static CANVAS_HEIGHT: Lazy<Mutable<U32Height>> = Lazy::new(|| Mutable::new(30));
static DEVICE_PIXEL_RATIO: Lazy<Mutable<f64>> = Lazy::new(|| Mutable::new(window().device_pixel_ratio()));
static CANVAS_POINTER_POSITION: Lazy<Mutable<(i32, i32)>> = Lazy::new(|| Mutable::new((0, 0)));
static ANIMATION_LOOP: Lazy<Mutable<Option<AnimationLoop>>> = Lazy::new(|| Mutable::new(None));

trait Shader {
    async fn run(canvas: zoon::web_sys::HtmlCanvasElement);
}

struct GpuContext<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: (U32Width, U32Height),
}

impl<'a> GpuContext<'a> {
    async fn new(canvas: web_sys::HtmlCanvasElement) -> Self {
        let instance = Instance::default();
        let surface = instance.create_surface(SurfaceTarget::Canvas(canvas))
            .expect_throw("Failed to create surface");
        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
            .await
            .expect_throw("Failed to find an appropriate adapter");
    
        let (device, queue) = adapter.request_device(&DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: Limits::downlevel_webgl2_defaults()
                .using_resolution(adapter.limits()),
        }, None)
            .await
            .expect("Failed to create device");
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);
        let size = (CANVAS_WIDTH.get(), CANVAS_HEIGHT.get());
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0 * DEVICE_PIXEL_RATIO.get() as u32,
            height: size.1 * DEVICE_PIXEL_RATIO.get() as u32,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: if !surface_format.is_srgb() {
                vec![surface_format.add_srgb_suffix()]
            } else {
                vec![]
            },
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
    
        Self {
            surface,
            device,
            queue,
            config,
            size,
        }    
    }

    fn resize(&mut self) {
        if self.size.0 != CANVAS_WIDTH.get() || self.size.1 != CANVAS_HEIGHT.get() {
            self.size = (CANVAS_WIDTH.get(), CANVAS_HEIGHT.get());
            self.config.width = self.size.0 * DEVICE_PIXEL_RATIO.get() as u32;
            self.config.height = self.size.1 * DEVICE_PIXEL_RATIO.get() as u32;
            self.surface.configure(&self.device, &self.config);
        }
    }
}


// pub fn global_canvas() -> impl Element {
//     Canvas::new()
//         .width(40)
//         .s(Width::fill())
//         .s(Height::screen())
//         .update_raw_el(|el| el
//             .on_resize(|w, h| {
//                 CANVAS_WIDTH.set_neq(max(w, 1));
//                 CANVAS_HEIGHT.set_neq(max(h, 1));
//             })
//         )
//         .after_insert(|canvas| Task::start(hello_triangle_with_vertex_buffer::run(canvas)))
//         .after_remove(|_| ANIMATION_LOOP.set(None))
// }
