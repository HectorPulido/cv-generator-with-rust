use comrak::{markdown_to_html, ComrakExtensionOptions, ComrakOptions};
use std::fs::File;

use wkhtmltopdf::*;
#[allow(dead_code)]
const DATA: &str = r###"
[
    {
        "type": "config",
        "data": {
            "title": "Hector CV",
            "theme": "default"
        }
    },
    {
        "type": "intro",
        "data": "Hello, Hector Pulido is Here! 👋"
    },
    {
        "type": "description",
        "data": "`Third Clarke law; Any sufficiently advanced technology is indistinguishable from magic.`<br><em> I'm just a simple **Mechatronic engineer**, **backend developer**, **#MachineLearning** and **#Gamedev** lover, I have delusion of **youtuber** and now **streamer**, There is no challenge big enough not to try, My code will change the **world** some day.</em>"
    },
    {
        "type": "rightImage",
        "data": {
            "image": "https://github.com/HectorPulido/HectorPulido/raw/master/img/pequesoft.png",
            "link": "https://twitter.com/Hector_Pulido_"
        }
    },
    {
        "type": "techStack",
        "data": {
            "title": "Tech Stack ⌨",
            "tech": [
                "Python and Flask, Django ⭐",
                "Pytorch ⭐",
                "C# and .Net Technologies",
                "Rust and WASM ⭐",
                "SQL and Relational Databases",
                "Linux, AWS and Docker",
                "Unity3D ⭐",
                "Arduino and electronics",
                "ASM for Pics"
            ]
        }
    },
    {
        "type": "space",
        "data": {}
    },
    {
        "type": "extra",
        "data": "<p align=\"center\">\n<a href=\"#user-30538313-pinned-items-reorder-form\">\n<img align=\"center\" src=\"https://github-readme-stats.vercel.app/api?username=HectorPulido&bg_color=30,e96443,904e95&title_color=fff&text_color=fff\" alt=\"Hector's Github Stats\"/>\n</a>\n</p>"
    },
    {
        "type": "social",
        "data": {
            "title": "@<div align=\"center\">\n<h3 align=\"center\">Let's connect 😋</h3>\n</div>",
            "social": [
                {
                    "alt": "Hector's LinkedIn",
                    "url": "https://www.linkedin.com/in/hector-pulido-17547369/",
                    "image": "https://www.vectorlogo.zone/logos/linkedin/linkedin-icon.svg"
                },
                {
                    "alt": "Hector's Twitter",
                    "url": "https://twitter.com/Hector_Pulido_",
                    "image": "https://www.vectorlogo.zone/logos/twitter/twitter-official.svg"
                },
                {
                    "alt": "Hector's Twitch",
                    "url": "https://www.twitch.tv/hector_pulido_",
                    "image": "https://www.vectorlogo.zone/logos/twitch/twitch-icon.svg"
                },
                {
                    "alt": "Hector's Youtube",
                    "url": "https://www.youtube.com/channel/UCS_iMeH0P0nsIDPvBaJckOw",
                    "image": "https://www.vectorlogo.zone/logos/youtube/youtube-icon.svg"
                }
            ]
        }
    }
]
"###;

struct Generator {
    title: String,
    theme: String,
}

impl Generator {
    const CALLBACKS: [(&'static str, fn(&serde_json::Value) -> String); 7] = [
        ("intro", Generator::intro),
        ("description", Generator::description),
        ("rightImage", Generator::right_image),
        ("techStack", Generator::tech_stack),
        ("space", Generator::space),
        ("extra", Generator::extra),
        ("social", Generator::social),
    ];

    fn config(&mut self, input: &serde_json::Value) {
        self.theme = input["theme"].to_string();
        self.title = input["title"].to_string();
    }

    fn intro(input: &serde_json::Value) -> String {
        return format!("# {}\n", input["data"].as_str().unwrap());
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

    fn tech_stack(input: &serde_json::Value) -> String {
        let title = Self::format_title(input["data"]["title"].as_str().unwrap().to_string());
        let tech: Vec<String> = input["data"]["tech"]
            .as_array()
            .expect("Invalid json format")
            .into_iter()
            .map(|x| format!("- {}", x.as_str().unwrap()))
            .collect();
        let tech_string = tech.join("\n");
        return format!("{}\n{}\n", title, tech_string);
    }

    fn space(_: &serde_json::Value) -> String {
        return String::from("<br>");
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

    pub fn new() -> Self {
        return Self {
            title: String::from("default"),
            theme: String::from("default"),
        };
    }

    pub fn format(&mut self, json: &str) -> String {
        let json_serde: Value = serde_json::from_str(json).expect("Could not parse json");
        let json_serde = json_serde.as_array().expect("Invalid json format");
        let mut markdown: String = String::from("");
        for field in json_serde {
            if field["type"] == "config" {
                self.config(field);
                continue;
            }

            for (name, callback) in &Self::CALLBACKS {
                if *name == field["type"].as_str().expect("Invalid json format") {
                    markdown = format!("{}\n{}", markdown, callback(field));
                    continue;
                }
            }
        }

        return markdown;
    }

    pub fn generate_html(markdown: String) -> String {
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

    pub fn apply_theme(html: String) -> String {
        let header_html = r###"
        <!doctype html>
        <html lang="en">
        <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">    
        
        <!-- Bootstrap core CSS -->
        <link href="https://getbootstrap.com/docs/5.0/dist/css/bootstrap.min.css" rel="stylesheet" crossorigin="anonymous">
        
        <style>
            .bd-placeholder-img {
            font-size: 1.125rem;
            text-anchor: middle;
            -webkit-user-select: none;
            -moz-user-select: none;
            user-select: none;
            }
        
            @media (min-width: 768px) {
            .bd-placeholder-img-lg {
                font-size: 3.5rem;
            }
            }
        </style>;
        <!-- Custom styles for this template -->
        <link href="https://getbootstrap.com/docs/5.0/examples/starter-template/starter-template.css" rel="stylesheet">
        </head>
        <body>
    "###;

        let footer_html = r###"
        </body>

        <style>
        img.emoji {
        height: 1em;
        width: 1em;
        margin: 0 .05em 0 .1em;
        vertical-align: -0.1em;
        }
        </style>
        <script src="https://twemoji.maxcdn.com/2/twemoji.min.js?11.2"></script>
        <script>window.onload = function () { twemoji.parse(document.body);}</script>

        </html>
    "###;

        return format!("{}{}{}", header_html, html, footer_html);
    }

    pub fn generate_pdf(html: String) {
        let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
        let mut settings = pdf_app.builder();
        settings
            .orientation(Orientation::Portrait)
            .margin(Size::Millimeters(12))
            .title("CV");
        unsafe {
            // Enables warning for JavaScript errors that may occur
            settings.object_setting("load.debugJavascript", "true");
        }
        let gs = settings
            .global_settings()
            .expect("failed to create global settings");
        let os = settings
            .object_settings()
            .expect("failed to create object settings");
        let mut c = gs.create_converter();
        c.set_warning_callback(Some(Box::new(|warn| {
            println!("warning: {}", warn);
        })));
        c.add_html_object(os, &html);
        let mut pdfout = c.convert().expect("failed to convert");
        let mut file = File::create("basic.pdf").expect("failed to create basic.pdf");
        let bytes = std::io::copy(&mut pdfout, &mut file).expect("failed to write to basic.pdf");
        println!("wrote {} bytes to file: basic.pdf", bytes)
    }
}

use serde_json::Value;

fn main() {
    let mut generator = Generator::new();
    let markdown = generator.format(&DATA);
    let html = Generator::generate_html(markdown);
    let final_html = Generator::apply_theme(html);
    Generator::generate_pdf(final_html);
}
