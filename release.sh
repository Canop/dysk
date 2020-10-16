# build a new release of lfs for distribution
# 
# For your own usage, you should rather do
#     cargo install --path .
# 
version=$(sed 's/version = "\([0-9.]\{1,\}\)"/\1/;t;d' Cargo.toml | head -1)

echo "Building release $version"

# clean previous build
rm -rf build
mkdir build

# build the linux version
cargo build --release
strip target/release/lfs
cp target/release/lfs build

# add the readme and changelog in the build directory
echo "This is lfs. More info and installation instructions on https://github.com/Canop/lfs" > build/README.md
cp CHANGELOG.md build

# prepare the release archive
rm lfs_*.zip
zip -r "lfs_$version.zip" build/*

# copy it to releases folder
mkdir releases
cp "lfs_$version.zip" releases
