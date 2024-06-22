Here's the corrected version:

---

# HTTP API server Rust example and deploy to setip.io

This is an example of a Rust project that can be available from a public URL and run on a computer locally or, without change, from your setip.io account in case the computer would be unavailable.

This code can also be instantly redeployed remotely after being changed locally so it always reflects the local changes: Simply include the deploy.sh script into your preferred CI/CD.

Rust code is very efficient, and running it directly behind one's URLs on a setip.io account provides some unique performance advantages when compared with other deployment methods. Added to this is the ability to keep deployment in-house without changing any of the code, depending on whether it is deployed in-house or over the public cloud.

The definition of which port is available for public URL access does not need to be known when coding and will be automatically available from the SETIP_LISTEN_PORT environment variable. 
Since each bucket available from one's setip.io account is assigned at least one listening port of its own to be accessible through a secured URL this makes the code compatible with any setip.io bucket that one will choose to deploy to:

```rust
let port = env::var("SETIP_LISTEN_PORT").unwrap_or_else(|_| "8099".to_string());
let addr: SocketAddr = format!("127.0.0.1:{}", port).parse()?;
```

By default one's setip.io account provides a total of 10 buckets, the first 7 buckets are assigned to deploy static code only (Think a React.js site's build directory) while the last 3 buckets are assigned to deploying code that can be made accessible through any of the predefined URLs provided by one's setip.io account.
Buckets accessible to deploy rust code is therefore between b8 and b11 included. This means that if one connects to `https://b8.projectname.wg0.io` then one will be redirect to your code runninng from bucket `b8` and you will need to change the value set for `BUCKET_LOCATION='https://b11.setip.io/upload'` to be `BUCKET_LOCATION='https://b8.projectname.wg0.io/upload'`
To define any URL name you can just create a URL in the URL's aread of the Manage section of the setip.io account and use `b8-APP` as the origin for that URL so it connects directly to the port defined by the SETIP_LISTEN_PORT variable in your code.


## Context

An account on setip.io lets one create public and secure URLs and connect the URLs with code accessible directly behind the URLs and running in-house connected with pre-configured WireGuard-based tunnels.
If wanting to keep always-on access to the code running behind one's URLs, it is very simple to push a Rust binary to one's setip.io account and associate it with any of the pre-configured URLs.
This script just needs to be edited to include the authentication key to one's account to then allow pushing code for instant availability behind one's URLs.
The most obvious example would be an API server to serve as a backend to web or mobile applications or directly as a front end and backend in other contexts.

## What's in the code

The code contains one file that creates a simple API server that listens on a port that is automatically associated with any of one's URLs defined on the setip.io management console.

To understand how the API server is written in Rust, read the main.rs file. Then read the deploy.sh file to understand how the code is compiled and pushed to a setip.io's account URL for deployment.

## Deploy to your setip.io account

Edit the deploy.sh file and edit the following:

```bash
BUCKET_LOCATION=https://b5.yourprojectname.wg0.io # Use your domain name if registered with your setip.io account.
BEARER=xxxxxxxxx # Replace xxxxxxxxx with the Authentication Token found under the Deploy Key section from the Keys menu available in the Manage area on setip.io after you are logged in.
```

The script will compile the Rust code and execute it behind a URL of your choice or any of the preset bucket URLs available with setip.io accounts.
The script will then upload the compiled Rust binary to the bucket location of your choice.
To do so, copy and run the following from this project cloned directory on your local machine using the shell.

```bash
chmod +x ./deploy.sh
./deploy.sh
```

## Build

```bash
cargo build --target wasm32-wasi --release
```

## Run locally

Make sure you have installed wasmedge:

```bash
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
source $HOME/.wasmedge/env
```

And run the compiled Rust binary locally within wasmedge:

```bash
wasmedge target/wasm32-wasi/release/wasmedge_setip_demo.wasm
```

## Test

Run the following from another terminal:

```bash
$ curl http://localhost:8099/
Try POSTing data to /echo such as: `curl localhost:8080/echo -XPOST -d 'hello world'`
```

```bash
$ curl http://localhost:8099/echo -X POST -d "WasmEdge"
WasmEdge
```

---

Please replace the placeholders like `yourprojectname` and `xxxxxxxxx` with your actual project name and authentication token.