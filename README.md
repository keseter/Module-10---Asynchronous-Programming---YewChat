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

## 3.2 Be Creative!
![alt text](<images/Screenshot 2026-05-18 121604.png>)
![alt text](<images/Screenshot 2026-05-18 121710.png>)
![alt text](<images/Screenshot 2026-05-18 121848.png>)


For the creative version, I redesigned the original tutorial interface into a more themed experience called the "Campfire Lounge".

Creative additions:

- I added a new landing page before login to make the app feel more like a complete product instead of a single-screen demo.
- I redesigned the login page with a stronger visual identity, warmer colors, and more playful onboarding text.
- I restyled the main chat page with a themed sidebar, a live conversation panel, and a more expressive empty state.
- I kept the original real-time WebSocket functionality intact, so the creative version still behaves like the tutorial app while looking much more intentional.


## How to Run

1. Start the websocket server in the server project.
2. In this `YewChat` folder, run `npm.cmd run build:web`.
3. Move into `webpkg` and run `python -m http.server 8000`.
4. Open `http://localhost:8000`.

## Bonus: Rust Websocket Server for YewChat

For the bonus task, I replaced the JavaScript websocket server from Tutorial 3 with the Rust broadcast server from Tutorial 2 and modified it so it could understand the Yew chat protocol.

### What I changed

The original Rust server only broadcasted plain text messages such as:

`From client 127.0.0.1: hello`

That format worked for the console chat in Tutorial 2, but it could not work with the Yew web client in Tutorial 3 because the Yew client expects structured JSON messages.

To make the Rust server compatible with YewChat, I changed it so that:

- it accepts incoming JSON text messages from the browser
- it handles a `register` message when a user joins
- it stores connected usernames in shared server state
- it broadcasts the full connected user list as a `users` message
- it broadcasts chat messages back in the same JSON format used by the original JavaScript server

### Why this works

The important idea is that WebSocket communication is still text-based here. The JSON messages are simply text strings that are serialized on send and deserialized on receive.

The Yew client sends messages like:

- `register`
- `message`

The Rust server now reads those JSON strings, interprets the message type, and sends back JSON strings in the exact shape expected by the frontend. Because the protocol now matches the JavaScript version, the Yew web client can use the Rust server without changing its client-side logic.

### Why this is a successful change

This is a successful change because:

- the Rust server now supports the same browser client workflow as the JavaScript server
- usernames are tracked on the server
- connected users are broadcast to all clients
- chat messages are forwarded in the JSON format expected by the Yew frontend
- the frontend does not need to change its WebSocket protocol to use the Rust backend

### My opinion

I prefer the Rust version for learning and for long-term backend development. Rust gives stronger type safety, better concurrency guarantees, and makes the message protocol more explicit in code. That is especially useful when the server starts becoming more complex.

At the same time, the JavaScript version is shorter and easier to prototype quickly. For a very small demo, JavaScript is faster to write. But for reliability, maintainability, and confidence in the code, I prefer the Rust version.
