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

For styling:

run $npm run dev

#To run it in production

Install docker and enable your user to be able to use docker without sudo(or root privilages)

Navigate to directory containing this repository

$docker build -t shilohnova .

$docker run -d --name shilohnova   -p 127.0.0.1:8080:3000   -v $(pwd)/db_data:/app/data   -e LEPTOS_SITE_ADDR="0.0.0.0:3000"  -e LEPTOS_SITE_ROOT="site" -e LEPTOS_SITE_PKG_DIR="pkg" shilohnova

Now configure a web server of your choice
