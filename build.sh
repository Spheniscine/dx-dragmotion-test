rm -r ./docs
dx bundle --release
mv ./target/dx/dx-dragmotion-test/release/web/public ./docs
cp docs/index.html docs/404.html