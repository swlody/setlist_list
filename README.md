Built with https://loco.rs

WIP build:
```
cd assets
npm install
npx tailwindcss -i static/styles/input.css -o static/dist/styles/output.css
npx webpack --no-devtool --mode development
cd ..
cargo loco start
```