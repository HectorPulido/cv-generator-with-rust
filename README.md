# CV Generator made with rust
This is a template based cv generator made with rust, comrak, wkhtmltopdf. It converts a json file to a pdf version

## Features
- Nodes: intro, description, rightImage, techStack, space, extra (html), social
- json convertion to pdf

## TO DO
- ~~Expose as Rest API~~
- ~~More than 1 pdf output~~
- Graphic interface

## How to run 
Configure in the .env file the port and the host
1. docker build -t web:latest .
2. docker run -d --name cv-gen -e "PORT=8765" -e "DEBUG=0" -p 8081:8765 web:latest

## Example
http://localhost:8081/generate_cv
```json
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
        "data": "`Third Clarke law; Any suffici..."
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
                ...
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
                ...
            ]
        }
    }
]
```

![Example](/img/img.png) <br/>

<div align="center">
<h3 align="center">Let's connect 😋</h3>
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
