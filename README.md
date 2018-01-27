# Rust-webapp-starter
Rust single page webapp written in actix-web with vuejs.

## start
1 - first create a name 'webapp' postgresql database for this project.

2 - backend server
```bash
$ git clone https://github.com/OUISRC/Rust-webapp-starter.git
$ cd Rust-webapp-starter/backend
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
$ cargo run
```
3 - fontend server
```bash
$ git clone https://github.com/OUISRC/Rust-webapp-starter.git
$ cd Rust-webapp-starter/fontend
$ npm install
$ npm run dev
```
then open broswer 'http://localhost:1234/'
