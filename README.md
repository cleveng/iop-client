# [Document](https://open.alibaba.com/doc/api.htm?spm=a2o9m.11193494.0.0.24813a3aBw28pU#/api?cid=2&path=/auth/token/create&methodType=GET/POST)

Alibaba Trade SDK for Rust

## Usage

```rust
let iop_client = IopClient::new(appid, app_secret);
let redirect_url = iop_client.get_redirect_url(redirect_uri, state); // state is optional
println!("{}", redirect_url);
```
