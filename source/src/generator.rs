use comrak::{markdown_to_html, ComrakExtensionOptions, ComrakOptions};
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use regex::Regex;

#[derive(Debug)]
pub struct Generator {
    title: String,
    theme: String,
    extra_data: Option<serde_json::Value>,
}

impl Generator {
    const CALLBACKS: [(&'static str, fn(&serde_json::Value) -> String); 9] = [
        ("intro", Generator::intro),
        ("achievement", Generator::achievement),
        ("title", Generator::title),
        ("description", Generator::description),
        ("rightImage", Generator::right_image),
        ("unorderedList", Generator::unordered_list),
        ("space", Generator::space),
        ("extra", Generator::extra),
        ("social", Generator::social),
    ];

    fn format_value(input: &Value) -> String {
        return input.to_string().replace("\"", "");
    }

    fn config(&mut self, input: &serde_json::Value) {
        let input = &input["data"];
        self.theme = Generator::format_value(&input["theme"]);
        self.title = Generator::format_value(&input["title"]);
        self.extra_data = Some(input["extra_data"].clone())
    }

    pub fn new() -> Self {
        return Self {
            title: String::from("default"),
            theme: String::from("default"),
            extra_data: None
        };
    }

    fn intro(input: &serde_json::Value) -> String {
        return format!("# {}\n", input["data"].as_str().unwrap());
    }

    fn title(input: &serde_json::Value) -> String {
        return format!("## {}\n", input["data"].as_str().unwrap());
    }

    fn achievement(input: &serde_json::Value) -> String {
        let title = input["data"]["title"].as_str().unwrap();

        let mut achievement = if let Some(url) = input["data"].get("url") {
            let url = url.as_str().unwrap();
            format!("### <a href=\"{url}\">{title}</a>")
        } else {
            format!("### {title}")
        };

        let description: Vec<String> = input["data"]["list"]
            .as_array()
            .expect("Invalid json format")
            .into_iter()
            .map(|x| format!("- {}", x.as_str().unwrap()))
            .collect();

        let description = description.join("\n");
        achievement = format!("{achievement}\n{description}");

        if let Some(date_range) = input["data"].get("data_range") {
            let date_range = date_range.as_str().unwrap();
            achievement = format!("{achievement}</br><b>{date_range}</b>");
        }

        return achievement;
    }

    fn description(input: &serde_json::Value) -> String {
        return format!("{}\n", input["data"].as_str().unwrap());
    }

    fn right_image(input: &serde_json::Value) -> String {
        let props = "align=\"right\" height=\"auto\" width=\"200\"";
        return format!(
            "<a href=\"{}\">\n<img {} src=\"{}\"/>\n</a>\n",
            input["data"]["link"].as_str().unwrap(),
            props,
            input["data"]["image"].as_str().unwrap()
        );
    }

    fn format_title(title: String) -> String {
        let mut title = title;
        if title.chars().nth(0).unwrap() == '@' {
            title = String::from(&title[1..title.len()]);
            return format!("{}", title);
        }

        return format!("## {}", title);
    }

    fn unordered_list(input: &serde_json::Value) -> String {
        let title = Self::format_title(input["data"]["title"].as_str().unwrap().to_string());
        let tech: Vec<String> = input["data"]["list"]
            .as_array()
            .expect("Invalid json format")
            .into_iter()
            .map(|x| format!("- {}", x.as_str().unwrap()))
            .collect();
        let tech_string = tech.join("\n");
        return format!("{}\n{}\n", title, tech_string);
    }

    fn space(_: &serde_json::Value) -> String {
        return String::from("</br>");
    }

    fn extra(input: &serde_json::Value) -> String {
        return input["data"].as_str().unwrap().to_string();
    }

    fn social(input: &serde_json::Value) -> String {
        let title = Self::format_title(input["data"]["title"].as_str().unwrap().to_string());
        let props = "align=\"center\" width=\"30px\"";

        let social: Vec<String> = input["data"]["social"]
            .as_array()
            .expect("Invalid json format")
            .into_iter()
            .map(|x| {
                format!(
                    "<a href=\"{}\" {}>\n<img {} alt=\"{}\" src=\"{}\"/></a>{}",
                    x["url"].as_str().unwrap(),
                    "target=\"blank\"",
                    props,
                    x["alt"].as_str().unwrap(),
                    x["image"].as_str().unwrap(),
                    "&nbsp; &nbsp;\n"
                )
            })
            .collect();
        let social_string = social.join("");

        return format!("{}\n<p align=\"center\">\n{}\n</p>\n", title, social_string);
    }

    fn format_section(&mut self, json: Value) -> String {
        let mut markdown = String::from("");
        for field in json.as_array().expect("Invalid json format") {
            for (name, callback) in &Self::CALLBACKS {
                if *name == field["type"].as_str().expect("Invalid json format") {
                    markdown = format!("{}\n{}", markdown, callback(field));
                    continue;
                }
            }
        }
        return markdown;
    }

    pub fn format(&mut self, json: Value) -> String {
        let json_serde = json.as_array().expect("Invalid json format");
        let mut html_data = String::from("");

        for field in json_serde {
            if field["type"] == "config" {
                self.config(field);
                continue;
            }

            if field["type"] == "section" {
                let size = Generator::format_value(&field["size"]);
                let formated_data = &self.format_section(field["data"].clone());
                let generated_html = self.generate_html(formated_data.to_string());
                html_data = format!("{html_data} <section class=\"col-md-{size}\">{generated_html}</section>");
            }   
        }
        return html_data;
    }

    pub fn generate_html(&mut self, markdown: String) -> String {
        let mut opts = ComrakOptions {
            extension: ComrakExtensionOptions {
                strikethrough: true,
                tagfilter: true,
                table: true,
                autolink: true,
                tasklist: true,
                superscript: true,
                footnotes: true,
                description_lists: true,
                ..ComrakExtensionOptions::default()
            },
            ..ComrakOptions::default()
        };
        opts.render.unsafe_ = true;

        return markdown_to_html(markdown.as_str(), &opts);
    }

    pub fn apply_theme(&mut self, html: String) -> String {
        let mut template: String = fs::read_to_string(&self.theme).expect("Unable to read theme file");
        template = template.replace("[[[html]]]", &html);
        if let Some(extra_data) = &self.extra_data {
            for (key, value) in extra_data.as_object().unwrap() {
                let key = format!("[[[{}]]]", key);
                let value = value.to_string().replace("\"", "");
                template = template.replace(&key, &value);
            }
        }

        let fa_re = Regex::new(r"\{i (.+)\}").unwrap();
        template = fa_re.replace_all(&template, "<i class=\"$1\"></i>").to_string();

        println!("{template}");
        return template;
    }

    pub fn generate_pdf(html: String, filename: &String) -> String {
        let mut child = Command::new("wkhtmltopdf")
            .arg("--viewport-size")
            .arg("1280x1024")
            .arg("-")
            .arg(filename)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        std::thread::spawn(move || {
            stdin
                .write_all(html.as_bytes())
                .expect("Failed to write to stdin");
        });
        let _ = child.wait_with_output().expect("Failed to read stdout");
        return filename.to_string();
    }
}
