# Rust-webapp-starter
Rust single page webapp written in actix-web with vuejs.
- Async stable Actix-web framework
- diesel, postgresql r2d2
- SPA CORS JWT
- Vuejs Parcel-bundler

## How To
    first create a name 'webapp' postgresql database for this project.

## when development
```bash
$ git clone https://github.com/OUISRC/Rust-webapp-starter.git
$ cd Rust-webapp-starter
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
$ cargo run

// another shell

$ cd Rust-webapp-starter/webapp
$ npm install
$ npm run dev
```
then open broswer 'http://localhost:1234/'

## when production

```bash
$ git clone https://github.com/OUISRC/Rust-webapp-starter.git
$ cd Rust-webapp-starter
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
$ cd webapp
$ npm install
$ npm run build
$ cd ..
$ cargo run
```
then open broswer 'http://localhost:8000/'

## worked
- signup (done)
- signin (done)
- browse pages data : get/post (done)
- publish article (done)
- user center : jwt (working)

### Contribute
 
welcome to contribute !

### License

[LICENSE-APACHE](https://github.com/OUIRC/Rust-webapp-starter/blob/master/LICENSE).
