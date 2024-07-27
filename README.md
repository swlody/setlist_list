[![CI](https://github.com/swlody/setlist_list/actions/workflows/ci.yaml/badge.svg)](https://github.com/swlody/setlist_list/actions/workflows/ci.yaml)
<a href='http://www.recurse.com' title='Made with love at the Recurse Center'><img src='https://cloud.githubusercontent.com/assets/2883345/11325206/336ea5f4-9150-11e5-9e90-d86ad31993d8.png' height='20px'/></a>

Built with https://loco.rs

WIP build:

```
cargo sqlx database create
bun install --cwd assets
bun --cwd assets tailwindcss -c tailwind.config.js -i styles/input.css -o static/dist/output.css
bun build assets/src/main.js --outdir assets/static/dist [--minify] [--sourcemap=linked]
cargo run start
```
