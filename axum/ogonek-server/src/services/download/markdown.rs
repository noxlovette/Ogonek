use comrak::{Options, markdown_to_html};
use ogonek_types::PDFData;

pub(super) fn render_markdown_page(pdf_data: &PDFData, css_path: &str) -> String {
    let mut options = Options::default();
    options.extension.table = true;
    let content = markdown_to_html(&pdf_data.markdown, &options);

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
        css_path, pdf_data.title, content
    )
}
