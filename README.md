[![CI](https://github.com/swlody/setlist_list/actions/workflows/ci.yaml/badge.svg)](https://github.com/swlody/setlist_list/actions/workflows/ci.yaml)
<a href='http://www.recurse.com' title='Made with love at the Recurse Center'><img src='https://cloud.githubusercontent.com/assets/2883345/11325206/336ea5f4-9150-11e5-9e90-d86ad31993d8.png' height='20px'/></a>

Built with https://loco.rs

WIP build (development env):

```sh
docker run -d -p 5432:5432 \
  -e POSTGRES_DB=setlist_list_development \
  -e POSTGRES_USER=loco \
  -e POSTGRES_PASSWORD="loco" \
  postgres:latest
docker run -p 6379:6379 -d redis redis-server
cargo sqlx database setup
bun install --cwd assets
bun --cwd assets tailwindcss -c tailwind.config.js -i styles/input.css -o static/dist/output.css [--minify]
bun build assets/src/main.js --outdir assets/static/dist --sourcemap=linked [--minify]
cargo run start
```
