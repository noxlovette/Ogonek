use comrak::{Options, markdown_to_html};

pub(super) fn render_markdown_page(markdown: &str, css_path: &str) -> String {
    let mut options = Options::default();
    options.extension.table = true;
    let content = markdown_to_html(markdown, &options);

    let topic = "Flying Whales";
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">

    <link rel="stylesheet" href="{}">
    <title>Document</title>
</head>
<body>
<h1>{}</h1>
    <article>
        {}
    </article>
</body>
</html>"#,
        css_path, topic, content
    )
}
