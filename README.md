# YewChat 💬

> Source code for [Let’s Build a Websocket Chat Project With Rust and Yew 0.19 🦀](#)

## Install

1. Install the required toolchain dependencies:
   ```npm i```

2. Follow the YewChat post!

## Branches

This repository is divided to branches that correspond to the blog post sections:

* main - The starter code.
* routing - The code at the end of the Routing section.
* components-part1 - The code at the end of the Components-Phase 1 section.
* websockets - The code at the end of the Hello Websockets! section.
* components-part2 - The code at the end of the Components-Phase 2 section.
* websockets-part2 - The code at the end of the WebSockets-Phase 2 section.

## 3.1 Original code
![alt text](<images/Screenshot 2026-05-18 115329.png>)

This is the original Yew chat client running locally with the websocket server

Explanation

The websocket server is responsible for handling client connections and broadcasting messages between users.

The YewChat client is the frontend application. It runs in the browser and provides the graphical user interface for the chat system. Yew is written in Rust and compiled into WebAssembly so it can run inside the browser.

When a user types a message in the browser, the Yew client sends the message to the websocket server. The server then forwards the message to the connected clients. This allows the chat application to update in real time without refreshing the page.

This experiment shows a more practical use of asynchronous programming. Chat applications are suitable for asynchronous programming because messages and user connections can happen at unpredictable times. The program must be able to handle sending, receiving, and displaying messages without blocking the whole application.