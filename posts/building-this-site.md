# Building This Site with Rust and Leptos

*November 26, 2025*

I decided to rebuild my personal site from the ground up using Rust and Leptos. My previous site had a very bland UI and contained minimal content, so it set a low bar. I wanted to build a site that was visually clean, responsive, and had a rich catalogue of content that I could easily update and expand.

## Encountering Rust

I first directly encountered Rust in 2020 and found it interesting, but didn't really have a practical reason to use it personally or professionally. With time, I began unknowingly adopting Rust in my daily work, first through replacing some of my familiar linux CLI utilities with new tools such as `ripgrep`, `eza` and `fd`. Next, Rust began disrupting the Python ecosystem with tools such as `polars`, `ruff`, `granian`, `uv`, and `py-spy`. I adopted these tools on the basis of their objective superiority to their alternatives, and found them to be quite performant and reliable.

The tipping point for me, was when I tried out PyO3 and maturin in order to attempt to work out some performance critical sections of some of my large personal Python projects. The ergonomics and ease of integrating Rust into my codebase was shocking compared to previous experiences I have had with C extensions. I added the Rust code into my existing Python projects to great effect. Surprisingly, I was even able to approach parity with decades-old C code that I was using as a reference for customization. I was instantly hooked on the idea of writing fast and ergonomic code in Rust. This experience also conveyed to me the craft and generosity of spirit of the Rust community.

## Why Rust for a Personal Site?

Most personal sites are built with static site generators or simple HTML/CSS. I wanted to have some fun exploring the Rust/WASM ecosystem to get some experience and see how Rust held up in domains that it is not traditionally known for. While I enjoy UX and product design, I have never been fond of the Javascript/Typescript languages or framework ecosystem for frontend development, so I am always looking for viable alternatives to bring my visions to life.

Leptos is a reactive web framework for Rust that compiles to WebAssembly. It is designed to provide the reactive, component-based architecture of popular frontend frameworks, but with the performance and safety of Rust. Overall, I found Leptos to be fairly straightforward to learn and use, and I was able to build my site according to my tastes pretty quickly. Admittidly, my site is a fairly simple design and has limited functionality, but it
was still a fun experience. I was able to get my site up and running in just one night of work.

## The Stack

The site uses a simple setup:

- **Leptos** – Reactive UI framework
- **Trunk** – Build tool for Rust WASM apps
- **GitHub Pages** – Static hosting

More posts coming soon, as I find new topics that inspire me to write!
