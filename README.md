Here's the corrected version:

---

# Deploy a HTTP API server to a setip.io defined public URL

This is an example of a Rust project that can run an API server and be made instantly available over secured public URLs from any or limited public IP addresses . 
Using the same public URLs connections can be routed to a local machine or on the cloud so the local machine can be taken offline or if offline for unexpected reason (Local connection drop, hardware issues etc...).

Running the same code on the cloud and locally behind the same public URLs allows to save on some cloud related costs with in-house resources while still responding if in-house resources are no longer available.

This code is instantly redeployed after being changed locally so it always reflects the local changes any way it is accessible by includin the deploy.sh script into your preferred CI/CD scripts.

Rust code is very efficient, and running it directly behind one's URLs on a setip.io account provides some unique performance advantages when compared with other deployment methods. Added to this is the ability to keep deployment in-house without changing any of the code, depending on whether it is deployed in-house or over the public cloud.

The definition of which port one's code must be listening to so it is available through the public URL  does not need to be known when coding nor deploying and is automatically made available at runtime through the `SETIP_LISTEN_PORT` environment variable. 
Since each bucket available from one's setip.io account is assigned at least one listening port of its own the use of this variable instead of a specific port number makes the code compatible with any setip.io bucket that one will choose to deploy to. Below is an example of how the listening port is simply defined as `SETIP_LISTEN_PORT` and only defined as `8099`when running locally if `SETIP_LISTEN_PORT` has not been defined in a local build script and unlilke while executing on setip.io where it's always made available.

```rust
let port = env::var("SETIP_LISTEN_PORT").unwrap_or_else(|_| "8099".to_string());
let addr: SocketAddr = format!("127.0.0.1:{}", port).parse()?;
```

By default one's setip.io account provides a total of 10 buckets, the first 7 buckets are assigned to deploy static code only (Think a React.js site's build directory) while the last 3 buckets are assigned to deploying code. One can set static and dynamic bucket to be accessible from any custom public url as one ,may create under the URL's section while indicating the bucket number as the "Origin".

Buckets accessible to deploy rust code must be then be picked between b8 and b11 included. This means that if one connects to `https://b8.projectname.wg0.io` one will be redirect to one's code running from bucket `b8`; one will need to change the value in the deploy.sh that is set for `BUCKET_LOCATION='https://b11.setip.io/upload'` so it will show instead `BUCKET_LOCATION='https://b8.projectname.wg0.io/upload'`.

To define any URL name you can just create a URL in your setip.io account, and have it point to the default port listening for an API server. As an example, head in the URL's area of the Manage section of the setip.io account and type `b8-APP` for the value in the "Origin". The URL will proxy directly to the port defined by the SETIP_LISTEN_PORT variable from your code running in bucket 8. For the origin simply use the bucket number prefixed by the letter `b` and suffixed by `-APP`, so to forward to the code running in bucket 9 just use `b9-APP` as the origin.

You can also run the same code locally as you can read below and associate the same URLs to code running anywhere, even locally, simply use of one of the Wireguard supplied configurations. Each configuration matches a peer number and, once connected with wireguard and the matching configuration, the URL will hit your local machine local IP address (The private address available with the Wireguard configuration and displayed next to the configuration). For this URL you will then edit the origin to be `http://peer9.yourproject.wg0.io:4444` if you set SETIP_LISTEN_PORT to be 4444.

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
wasmedge target/wasm32-wasi/release/wasmedge_setip_demo.wasm --env SETIP_LISTEN_PORT=4444
```
Note that the  `--env SETIP_LISTEN_PORT=4444` will send the value to listen to your code when executing locally, replace 4444 by any number but make use your add it to the origin for the URL that must reach that port: `http://peer9.yourproject.wg0.io:4444`

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