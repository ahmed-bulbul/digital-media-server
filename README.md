📡 Digital Media Server (DMS)

An open-source Digital Media Server written in Rust.
The long-term goal is to support:

🎥 Video streaming (VOD & live)

🎧 Audio streaming

📞 Audio & Video calling (WebRTC)

📡 VoIP / SIP integration

📦 Recording & Transcoding (FFmpeg / GStreamer)


🚀 Project Status

Currently in early learning & prototyping phase.
We are building this project step by step — lesson by lesson — from zero to production.

🏗️ Project Structure
digital-media-server/
├── core/         # Core library (types, media primitives, utils)
├── playground/   # Sandbox binary for experiments
└── Cargo.toml    # Workspace configuration

📚 Learning Roadmap

We are documenting the journey in lessons.

✅ Lesson 1: Setup Rust, Cargo workspace, Hello World

⬜ Lesson 2: Rust basics (variables, types, functions, ownership)

⬜ Lesson 3: Modules, structs, enums, traits, error handling

⬜ Lesson 4+: Async, networking, RTP, WebRTC, etc.

Stay tuned — this repo is both a learning log and a real project.

🔧 Getting Started
Prerequisites

Rust
 (installed via rustup)

Cargo (comes with Rust)

Run the playground app
cd playground
cargo run


Expected output:

Hello, world!

🤝 Contributing

This is an open learning project — contributions are welcome!
Ways to help:

Open issues with ideas or improvements

Suggest learning resources

Submit PRs with beginner-friendly fixes

