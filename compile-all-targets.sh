# WARNING: This script is NOT meant for normal installation, it's dedicated
# to the compilation of all supported targets, from a linux machine.
# This is a long process and it involves specialized toolchains.
# For usual compilation do
#     cargo build --release

H1="\n\e[30;104;1m\e[2K\n\e[A" # style first header
H2="\n\e[30;104m\e[1K\n\e[A" # style second header
EH="\e[00m\n\e[2K" # end header

version=$(./version.sh)
echo -e "${H1}Compilation of all targets for dysk $version${EH}"
 
# clean previous build
rm -rf build
mkdir build
echo "   build cleaned"

# build versions for other platforms using cargo cross
cross_build() {
    name="$1"
    target="$2"
    cargo clean
    echo -e "${H2}Compiling the $name / $target version${EH}"
    cross build --target "$target" --release
    mkdir "build/$target"
    cp "target/$target/release/dysk" "build/$target/"
}

cross_build "Linux GLIBC" "x86_64-unknown-linux-gnu"
cross_build "MUSL" "x86_64-unknown-linux-musl"
cross_build "ARM 32" "armv7-unknown-linux-gnueabihf"
cross_build "ARM 32 MUSL" "armv7-unknown-linux-musleabi"
cross_build "ARM 64" "aarch64-unknown-linux-gnu"
cross_build "ARM 64 MUSL" "aarch64-unknown-linux-musl"
cross_build "NetBSD/amd64" "x86_64-unknown-netbsd"

# build the (local) linux version
target="x86_64-linux"
echo -e "${H2}Compiling the linux version - $target${EH}"
cargo clean
cargo build --release 
mkdir "build/$target/"
cp target/release/dysk "build/$target/"

# Find, and copy the completion scripts and the man page
# (they're built as part of the normal compilation by build.rs)
# (this script uses broot, which is available on my computer...)
echo -e "${H2}Copying completion scripts${EH}"
mkdir build/completion
cp "$(broot -c ":gi;release;:focus;/dysk.bash;:parent;:pp" target)/"* build/completion
mkdir build/man
mv build/completion/dysk.1 build/man

echo -e "${H1}Compilations done${EH}"
