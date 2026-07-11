use comrak::{markdown_to_html, ComrakOptions};
mod template;

pub fn github_markdown_to_html(md: String, filename: String) -> String {
    let mut contents = String::from("");
    let options = set_opts();
    let markdown = markdown_to_html(&md, &options);
    let boilerplate = template::format_boilerplate(&filename);
    let css: &str = template::CSS;
    let footer: &str = template::FOOTER;
    contents.push_str(boilerplate.as_str());
    contents.push_str(css);
    contents.push_str(&markdown);
    contents.push_str(footer);
    contents    
}

fn set_opts() -> ComrakOptions {
    comrak::ComrakOptions {
        unsafe_: true,
        github_pre_lang: true,
        ext_table: true,
        ext_tagfilter: true,
        ext_strikethrough: true,
        ext_footnotes: true,
        ext_superscript: true,
        ext_autolink: true,
        ext_tasklist: true,
        ext_description_lists: true,
        ..Default::default()
    }
}
