# HTTP server example using the Warp crate

This is an example of a Rust project that can run from your setip.io account and be associated with a URL of your choice from the URL menu.

## Deploy to your setip.io account

Edit the deploy.sh file and edit the following:



```bash
BUCKET_LOCATION=https://b5.yourprojectname.wg0.io #use your domain name if registered with your setip.io account.
BEARER=xxxxxxxxx # Replace xxxxxxxxx with the Authentication Token found under the Deploy Key section from the Keys menu available in the Manage area on setip.io after you are logged in.
```

The script will compile the rust code and execute it behind a URL of your choice or any of the preset bucket URLs available with setip.io accounts.
The script will then upload the compiled rust binary to the bucket location of your choice.

```bash
chmod +x ./deploy.sh
./deploy.sh
```

## Build

```bash
cargo build --target wasm32-wasi --release
```

## Run locally.

Make sure you have installed wasmedge: 

```bash
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
source $HOME/.wasmedge/env
```


And run the compiled rust binary locally within wasmedge.
```bash
wasmedge target/wasm32-wasi/release/wasmedge_setip_demo.wasm
```

## Test

Run the following from another terminal.

```bash
$ curl http://localhost:8099/
Try POSTing data to /echo such as: `curl localhost:8080/echo -XPOST -d 'hello world'`
```

```bash
$ curl http://localhost:8099/echo -X POST -d "WasmEdge"
WasmEdge
```
