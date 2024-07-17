Built with https://loco.rs

WIP build:

```
npm install
npx tailwindcss -i assets/styles/input.css -o assets/static/dist/output.css
npx webpack --no-devtool --mode development
cargo loco start
```
