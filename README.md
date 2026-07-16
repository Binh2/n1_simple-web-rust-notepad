# Introduction

This project is a web notepad with tabs. No save features yet. It is made in Rust Leptos. Just exclusively client-side for now.

In the future, I could add cache, so the notes is save locally on the user's browser.

## Demo

Check out my website at [https://binh2.github.io/n1_simple-web-rust-notepad/](https://binh2.github.io/n1_simple-web-rust-notepad/)

Screenshot of the demo website: ![](https://github.com/Binh2/n1_simple-web-rust-notepad/blob/main/demo.png?raw=true)

## Missing features (could be added in the future if I decided to work on it again)

1. Cache: can't store any data after the browser tab is closed.
2. Cross sharing between devices: I need to implement a backend so I can store account password securely. I can use Firebase or any non relational database.
3. Server side rendering: It is a little faster for page loading.
4. Cross compatibility: Make it looks good on mobile device as well or I can make an mobile app instead of a web app. And more user friendly for mobile users.
5. AI: Sorry stupid idea :)

# How to run this project locally

`git clone https://github.com/Binh2/n1_simple-web-rust-notepad.git`
to clone the project locally

`cd n1_simple-web-rust-notepad`
to go to n1_simple-web-rust-notepad directory

`trunk serve --open`
to install all the dependencies and run a server locally

Open your browser to localhost:8080 to access the website

# What I learned

1. Rust
    * Option/Result data structure
    * Borrowing/referencing/deref-ing/moving value
    * Life cycle
    * Random number generator

2. Rust Leptos
    * Iteration (Surprisingly hard :) )
    * Component
    * Event listener
    * Node ref

# Technologies used

* Rust
* Rust Leptos
* Vanilla CSS (BEM naming convention)

# Time to complete

3 days

# Contributers

[Binh2](https://github.com/Binh2)