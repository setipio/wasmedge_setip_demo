Sure, here is the corrected version with spell-check and improvements:

---

# A Simple HTTP Public API Server to Deploy Locally with Offline Access

This is a simple project that can run an API server and be made instantly available over secured public URLs from any or limited public IP addresses.

It is written in Rust and runnable through WasmEdge. According to WasmEdge's authors, an app could take 1/100 of the size of a similar Linux container app.

Since setip.io accounts provide a server wasm environment that is also compatible with WasmEdge, any code running locally or on the cloud can run indifferently on local machines but also directly behind the setip.io provided public URLs when offline.

Think of this like you would think of a CDN but a caching solution for code-generated content, not only static content. That makes it possible to run code where it's the most efficient and the most cost-effective at the same time, just like a CDN and static pages.

Using the same setip.io supplied public URLs, incoming internet connections can be routed to any local machine (as in: a machine with no public IP addresses that you self-host or hosted) or on the cloud. By offering code execution both online and offline, a local machine can be taken offline for planned or unexpected reasons (local connection drop, hardware issues, etc.) while connections are still served from the public node as a failover.
 This lets local resources be used most of the time and remote resources be accessed only when local resources are unavailable. This limits third-party code execution costs only when necessary.
  Meanwhile, the same code can be used for local or remote execution without any configuration change, which greatly simplifies deployment. Better yet, by using a wasm execution environment, it is possible to write in most popular languages that can compile to the wasm standard so as to benefit from a large ecosystem.

Running the same code on the cloud and locally behind the same public URLs allows saving on some cloud-related costs with in-house resources while still responding if in-house resources are no longer available.

This code is instantly redeployed after being changed locally so it always reflects the local changes any way it is accessible by including the deploy.sh script in your preferred CI/CD scripts.

Rust code is very efficient, and running it directly behind one's URLs on a setip.io account provides some unique performance advantages when compared with other deployment methods. Added to this is the ability to keep deployment in-house without changing any of the code, depending on whether it is deployed in-house or over the public cloud.

The definition of which port one's code must be listening to, so this port can be reached through the public URL, does not need to be known when coding or deploying and is automatically made available at runtime through the `SETIP_LISTEN_PORT` environment variable. 
Since each bucket available from one's setip.io account is assigned at least one listening port of its own, the use of this variable instead of a specific port number makes the code compatible with any setip.io bucket that one will choose to deploy to. Below is an example of how the listening port is simply defined as `SETIP_LISTEN_PORT` and only defined as `8099` when running locally if `SETIP_LISTEN_PORT` has not been defined in a local build script, unlike while executing on setip.io where it's always made available.

```rust
let port = env::var("SETIP_LISTEN_PORT").unwrap_or_else(|_| "8099".to_string());
let addr: SocketAddr = format!("127.0.0.1:{}", port).parse()?;
```

By default, one's setip.io account provides a total of 10 buckets. The first 7 buckets are assigned to deploy static code only (think a React.js site's build directory) while the last 3 buckets are assigned to deploying code. One can set static and dynamic buckets to be accessible from any custom public URL as one may create under the URLs section while indicating the bucket number as the "Origin".

Buckets accessible to deploy Rust code must be picked between b8 and b11 inclusive. This means that if one connects to `https://c8.projectname.wg0.io` one will be redirected to one's code running from bucket `b8`. One will need to change the value in the deploy.sh that is set for `BUCKET_LOCATION='https://b11.setip.io/upload'` so it will show instead `BUCKET_LOCATION='https://b8.projectname.wg0.io/upload'`.
Please note that code is uploaded through buckets using the "b" preffix to the bucket number, as in: `https://b8.projectname.wg0.io`, but the direct public access URL is provided through the public matching public URL using "n" as a prefix to the bucket number, as in `https://c8.projectname.wg0.io`.


To define any URL name, you can just create a URL in your setip.io account, and have it point to the default port listening for an API server. As an example, head to the URLs area of the Manage section of the setip.io account and type `b8-APP` for the value in the "Origin". The URL will proxy directly to the port defined by the SETIP_LISTEN_PORT variable from your code running in bucket 8. For the origin simply use the bucket number prefixed by the letter `b` and suffixed by `-APP`, so to forward to the code running in bucket 9 just use `b9-APP` as the origin.

You can also run the same code locally as you can read below and associate the same URLs to code running anywhere, even locally, simply by using one of the WireGuard supplied configurations. Each configuration matches a peer number and, once connected with WireGuard and the matching configuration, the URL will hit your local machine's local IP address (the private address available with the WireGuard configuration and displayed next to the configuration). For this URL you will then edit the origin to be `http://peer9.yourproject.wg0.io:4444` if you set SETIP_LISTEN_PORT to be 4444.

## Context

An account on setip.io lets one create public and secure URLs and connect the URLs with code accessible directly behind the URLs and running in-house connected with pre-configured WireGuard-based tunnels.
If wanting to keep always-on access to the code running behind one's URLs, it is very simple to push a Rust binary to one's setip.io account and associate it with any of the pre-configured URLs.
This script just needs to be edited to include the authentication key to one's account to then allow pushing code for instant availability behind one's URLs.
The most obvious example would be an API server to serve as a backend to web or mobile applications or directly as a front end and backend in other contexts.

## What's in the Code

The code contains one file that creates a simple API server that listens on a port that is automatically associated with any of one's URLs defined on the setip.io management console.

To understand how the API server is written in Rust, read the main.rs file. Then read the deploy.sh file to understand how the code is compiled and pushed to a setip.io's account URL for deployment.

## Deploy to Your setip.io Account

Edit the deploy.sh file and edit the following:

```bash
BUCKET_LOCATION=https://b5.yourprojectname.wg0.io # Use your domain name if registered with your setip.io account.
BEARER=xxxxxxxxx # Replace xxxxxxxxx with the Authentication Token found under the Deploy Key section from the Keys menu available in the Manage area on setip.io after you are logged in.
```

The script will compile the Rust code and execute it behind a URL of your choice or any of the preset bucket URLs available with setip.io accounts.
The script will then upload the compiled Rust binary to the bucket location of your choice.
To do so, copy and run the following from this project's cloned directory on your local machine using the shell.

```bash
chmod +x ./deploy.sh
./deploy.sh
```

## Build

```bash
cargo build --target wasm32-wasi --release
```

## Run Locally

Make sure you have installed [WasmEdge](https://wasmedge.org). 

```bash
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
source $HOME/.wasmedge/env
```

And run the compiled Rust binary locally within WasmEdge:

```bash
wasmedge target/wasm32-wasi/release/wasmedge_setip_demo.wasm --env SETIP_LISTEN_PORT=4444
```
Note that the `--env SETIP_LISTEN_PORT=4444` will set the port for your code when executing locally. Replace 4444 with any number but make sure you add it to the origin for the URL that must reach that port: `http://peer9.yourproject.wg0.io:4444`.

## Test

Run the following from another terminal:

```bash
$ curl http://localhost:8099/
Try POSTing data to /echo such as: `curl localhost:8099/echo -XPOST -d 'hello world'`
```

```bash
$ curl http://localhost:8099/echo -X POST -d "WasmEdge"
WasmEdge
```

---

Please replace the placeholders like `yourprojectname` and `xxxxxxxxx` with your actual project name and authentication token.