use moon::*;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("atelier.eto.al")
        .append_to_head(include_str!("../favicon.html")) // realfavicongenerator.net
        .append_to_head(
            "
        <style>
            @import url('https://fonts.googleapis.com/css2?family=Murecho:wght@100..900&display=swap');
            html {
                background-color: #1e1e2e;
            }
        </style>",
        )
        .body_content(r#"<div id="app"></div>"#)
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
