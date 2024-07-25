[![CI](https://github.com/swlody/setlist_list/actions/workflows/ci.yaml/badge.svg)](https://github.com/swlody/setlist_list/actions/workflows/ci.yaml)
<a href='http://www.recurse.com' title='Made with love at the Recurse Center'><img src='https://cloud.githubusercontent.com/assets/2883345/11325206/336ea5f4-9150-11e5-9e90-d86ad31993d8.png' height='20px'/></a>

Built with https://loco.rs

WIP build:

```
npm install --prefix ./assets
./assets/node_modules/.bin/tailwindcss -c assets/tailwind.config.js -i assets/styles/input.css -o assets/static/dist/output.css
./assets/node_modules/.bin/webpack -c assets/webpack.config.js --no-devtool --mode development
cargo run start
```
