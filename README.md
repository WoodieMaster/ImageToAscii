# Image To Ascii

This is a little program that converts images into ascii character art

## About
I was interested in the rust programming language, so I wrote
a few small programs and this is one of them. It can convert
Images into Ascii Art.

## Overview
- The project is written in rust v1.73.0
- The entire code is located in the main.rs file
- Supports all common image formats (More details in the image crate docs)
- Used crates:
  - [image](https://crates.io/crates/image)


## Installation
If you have rust you can compile the project yourself,
otherwise copy the content of the target folder into your desired location.

If you want to use the command you can move the command.cmd file.

## Usage
Either call the command or the executable with the following arguments:
- The path to the image file you want to use
- The width of the output text
- The height of the output (the output will be smaller to account for the difference of height and width of the characters)