# CV Generator made with rust
This is a template based cv generator made with rust, comrak, wkhtmltopdf. It converts a json file to a pdf version

## Features
- Nodes: intro, description, rightImage, unorderedList, space, extra (html), social, achievement, title
- Rest API to generate CV
- JSON to pdf
- CLI application to generate the CV pdf

## TO DO
- ~~Expose as Rest API~~
- ~~More than 1 pdf output~~
- Graphic interface
- More Nodes

## How to generate the PDF without running the API
1. Install "wkhtmltopdf"
2. cargo run "cv_data.json"
Where cv_data.json is the name or the path of the json file with your cv info.

## How to run API
Configure in the .env file the port and the host
1. docker build -t web:latest .
2. docker run -d --name cv-gen -e "PORT=8765" -p 8081:8765 web:latest

## Example
http://localhost:8081/generate_cv
```json
[
    {
        "type": "config",
        "data": {
            "title": "Hector CV",
            "theme": "default_template.html",
            "extra_data": {
                "font-family": "Open Sans",
                "font-type": "sans-serif",
                "font-url": "https://fonts.googleapis.com/css?family=Open+Sans"
            }
        }
    },
    {
        "type": "section",
        "size": "8",
        "data": [
            {
                "type": "intro",
                "data": "Hector Pulido"
            },
            {
                "type": "description",
                "data": "<b>Backend developer, AI enthusiast, Mechatronic Engineer, Math MSC (2023)</b>"
            },
            {
                "type": "description",
                "data": "I'm just a simple <b>Computational math</b> MSC student..."
            },

            {
                "type": "title",
                "data": "{i fas fa-star} Achievements and Experience"
            },
            {
                "type": "achievement",
                "data": {
                    "title": "{i fas fa-briefcase} Backend developer at Platzi",
                    "list": [
                        "B2C & B2B EdTech web software, more than 3 million students.",
                        "..."
                    ],
                    "data_range": "August 2020 - Now"
                }
            },
            {
                "type": "achievement",
                "data": {
                    "title": "{i fas fa-graduation-cap} Mechatronic Engineering, Universidad Aut??noma Del Caribe, Barranquilla Colombia",
                    "list": [
                        "Honorable mention: ..."
                    ],
                    "data_range": "June 2015 - September 2019"
                }
            },
            {
                "type": "title",
                "data": "{i fas fa-keyboard} Related open source projects"
            },
            {
                "type": "achievement",
                "data": {
                    "title": "{i fas fa-robot} Evolutionary Neural Networks (C#, Unity)",
                    "list": [
                        "This is a <b>machine learning</b> ...."
                    ],
                    "url": "https://github.com/HectorPulido/Evolutionary-Neural-Networks-on-unity-for-bots"
                }
            }            
        ]
    },
    {
    
        "type": "section",
        "size": "4",
        "data": [
            {
                "type": "unorderedList",
                "data": {
                    "title": "{i fas fa-book} Contact",
                    "list": [
                        "{i fas fa-phone} +57 999999999",
                        "<a href='mailto:aaaaaaa@gmail.com'>{i fas fa-envelope} aaaaaaa@gmail.com</a>",
                        "<a href='https://github.com/HectorPulido'>{i fab fa-github} Github (HectorPulido)</a>",
                        "<a href='https://www.youtube.com/channel/UCS_iMeH0P0nsIDPvBaJckOw'>{i  fab fa-youtube} Youtube (Hector Pulido)</a>",
                        "<a href='https://twitter.com/Hector_Pulido_'>{i fab fa-twitter} Twitter (@Hector_Pulido_)</a>"
                    ]
                }
            },
            {
                "type": "unorderedList",
                "data": {
                    "title": "{i fas fa-magic} Tech Stack",
                    "list": [
                        "Backend with Python; Django stack",
                        "....",
                        "Unity3d"
                    ]
                }
            },
            {
                "type": "unorderedList",
                "data": {
                    "title": "{i fas fa-globe-europe} Languages",
                    "list": [
                        "Spanish (<b>Native</b>)",
                        "English (<b>Profesional profiency</b>)"
                    ]
                }
            },
            {
                "type": "unorderedList",
                "data": {
                    "title": "{i fas fa-fire} Soft Skill",
                    "list": [
                        "I like to learn about complex topics.",
                        "...",
                        "I can speak in public and transmit ideas easily.",
                    ]
                }
            }
        ]
    } 
]
```

![Example](/img/img.png) <br/>

<div align="center">
<h3 align="center">Let's connect ????</h3>
</div>
<p align="center">
<a href="https://www.linkedin.com/in/hector-pulido-17547369/" target="blank">
<img align="center" width="30px" alt="Hector's LinkedIn" src="https://www.vectorlogo.zone/logos/linkedin/linkedin-icon.svg"/></a> &nbsp; &nbsp;
<a href="https://twitter.com/Hector_Pulido_" target="blank">
<img align="center" width="30px" alt="Hector's Twitter" src="https://www.vectorlogo.zone/logos/twitter/twitter-official.svg"/></a> &nbsp; &nbsp;
<a href="https://www.twitch.tv/hector_pulido_" target="blank">
<img align="center" width="30px" alt="Hector's Twitch" src="https://www.vectorlogo.zone/logos/twitch/twitch-icon.svg"/></a> &nbsp; &nbsp;
<a href="https://www.youtube.com/channel/UCS_iMeH0P0nsIDPvBaJckOw" target="blank">
<img align="center" width="30px" alt="Hector's Youtube" src="https://www.vectorlogo.zone/logos/youtube/youtube-icon.svg"/></a> &nbsp; &nbsp;

</p>
