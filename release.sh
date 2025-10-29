# build a new release of dysk for distribution
# 
# WARNING: this is not intented for normal usage but
# for the official release. It involves a heavy tool
# chain running on linux.
#
# For your own usage, you should rather do
#
#     cargo install --path .
# 
# or
#
#     cargo build --release
#
version=$(./version.sh)

echo "Building release $version"

# clean previous build
rm -rf build
mkdir build

# compile all targets
./build-all-targets.sh

# add the readme and changelog in the build directory
echo "This is dysk. More info and installation instructions on https://github.com/Canop/dysk" > build/README.md
cp CHANGELOG.md build

# publish version number
echo "$version" > build/version

# prepare the release archive
rm dysk_*.zip
zip -r "dysk_$version.zip" build/*

# copy it to releases folder
mkdir releases
cp "dysk_$version.zip" releases
