#!/bin/bash
pandoc -t revealjs -V width=1600 -V height=900 --css mystyle.css -fmarkdown-implicit_figures --embed-resources --standalone -s chap1-2.md -o chap1-2.html
