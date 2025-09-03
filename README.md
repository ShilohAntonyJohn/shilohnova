# shilohnova
How to run

Clone repository in your computer 

Install rust

$curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Install necessary tools for wasm compilation

$rustup target add wasm32-unknown-unknown

$cargo install wasm-pack

Install clang:

On Fedora

$sudo dnf install clang

For other distributions:

Ask Gemini or ChatGPT or any generative AI of your choice

Install tailwind:

npm install tailwindcss @tailwindcss/cli

Finally to compile app

Navigate to folder and run ./build_script.sh

run npm run dev for css styling
