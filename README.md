# RISC - Β
RISC-Β (Really fast Indonesian Spell Ceker) is a **blazingly fast** Bahasa Indonesia Spell Ceker powered by **blazingly fast** rust programming language and **blazingly fast** redis in-memory database, the **blazyingly fast** svelte framework is also used in the frontend.

## HOW TO BUILD
Dependencies:
- rust
- redis + redisearch or redistack
- node.js

clone/download this repository and build the frontend or backend with the following instruction.

### backend
1. Setup redis database `$ cd backend && python scripts/setup_redis.py`
2. The database is not saved automatically, to save run `BGSAVE` in redis-cli. 
3. Build the webserver binary: `$ cargo build --release`
4. The release binary is located in `target/release`

### frontend
1. `$ cd frontend && npm install`
2. `$ npm run build` for building web extension
3. `$ npm build:webapp` for building web application
4. The generated build will be located in `dist` folder

