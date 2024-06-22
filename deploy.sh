# ! /bin/bash
BUCKET_LOCATION='https://b11.setip.io/upload'
# BUCKET_LOCATION='http://localhost:42115/upload'
BEARER='YOUR_KEY'

set -x

# comment below if running this script and have not yet accessed the repository.
rm -fR wasmedge_setip_demo
git clone https://github.com/setipio/wasmedge_setip_demo.git ./wasmedge_setip_demo
cd ./wasmedge_setip_demo
cargo build --target wasm32-wasi --release
mkdir buildwasm
mv ./buildwasm/zipped.zip ./"testwasmdeployzipped.$(date +%Y%m%d%H%M%S).zip"
# cd buildwasm
cp ./target/wasm32-wasi/release/wasmedge_setip_demo.wasm ./index.wasm
zip -r -D ./buildwasm/zipped.zip ./index.wasm
curl --location $BUCKET_LOCATION \
--header 'Authorization: Bearer '$BEARER \
--form 'files=@"./buildwasm/zipped.zip"'
