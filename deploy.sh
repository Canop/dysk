# Deploy the website and the downloadable application to the official server
# (obviously this won't work if you don't have access to the server, this
#  script is only to be used by official maintainers - to install the 
#  application, head up to https://dystroy.org/dysk/install/)

# build the release zip
./release.sh

version=$(./version.sh)

# # deploy on dystroy.org
rm -rf ~/dev/www/dystroy/dysk/download/*
cp -r build/* ~/dev/www/dystroy/dysk/download/
cp "dysk_$version.zip"  ~/dev/www/dystroy/dysk/download/
~/dev/www/dystroy/deploy.sh
