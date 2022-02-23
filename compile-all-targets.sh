# WARNING: This script is NOT meant for normal installation, it's dedicated
# to the compilation of all supported targets, from a linux machine.
# This is a long process and it involves specialized toolchains.
# For usual compilation do
#     cargo build --release

H1="\n\e[30;104;1m\e[2K\n\e[A" # style first header
H2="\n\e[30;104m\e[1K\n\e[A" # style second header
EH="\e[00m\n\e[2K" # end header

version=$(./version.sh)
echo -e "${H1}Compilation of all targets for lfs $version${EH}"
 
# clean previous build
rm -rf build
mkdir build
echo "   build cleaned"

# build the linux version
target="x86_64-linux"
echo -e "${H2}Compiling the linux version - $target${EH}"
cargo build --release 
strip target/release/lfs
mkdir "build/$target/"
cp target/release/lfs "build/$target/"

# build versions for other platforms using cargo cross
cross_build() {
    name="$1"
    target="$2"
    echo -e "${H2}Compiling the $name / $target version${EH}"
    cross build --target "$target" --release
    mkdir "build/$target"
    cp "target/$target/release/lfs" "build/$target/"
}
cross_build "Raspberry 32" "armv7-unknown-linux-gnueabihf"
cross_build "Android" "aarch64-linux-android"
cross_build "MUSL" "x86_64-unknown-linux-musl"
cross_build "NetBSD/amd64" "x86_64-unknown-netbsd"

echo -e "${H1}Compilations done${EH}"
