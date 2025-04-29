# gdps

Geometry Dash Private Server implemented in Rust (PHP sucks ass, and the way current implementations are written too)

## Why?

1. I don't like PHP.
2. I want to gain experience writing stuff in Rust.
3. I just want to.

## Requirements

- PostgreSQL Server.

## Setup

1. Rename `.env.example` to `.env` and change example data to your's
2. Import `database.sql` into your database (gdps for an example)
3. If you're planning to develop, run: `cargo sqlx prepare` to build a cache for SQL queries
4. Run `cargo run -r` (or `cargo build -r`, if you intend to run it later or by yourself)
5. fin

## Important Notes

- You'll need to change URLs inside of GeometryDash.exe, that will be also the same size as `https://www.boomlings.com` *(25 bytes)*
- It doesn't have any support (and not planned to) for versions lower than latest version of game [(2.207)](https://steamdb.info/patchnotes/16346964/)

## Credits

- [Cvolton/GMDprivateServer](https://github.com/Cvolton/GMDprivateServer)
- [MegaSa1nt/GMDprivateServer](https://github.com/MegaSa1nt/GMDprivateServer)
- [melowody/gdrs](https://github.com/melowody/gdRs)
- [Geometry Dash Documentation](https://wyliemaster.github.io/gddocs/#/) - A lot of explanations and other stuff, really helps in writing this project.