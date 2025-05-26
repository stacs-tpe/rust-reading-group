#!/bin/bash
pandoc -t revealjs -V width=1600 -V height=900 --css mystyle.css -V revealjs-url=https://unpkg.com/reveal.js@^5 -fmarkdown-implicit_figures --embed-resources --standalone -s chap10.md -o chap10.html
