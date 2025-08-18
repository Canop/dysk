# Deploy the website and the downloadable application to the official server
# (obviously this won't work if you don't have access to the server, this
#  script is only to be used by official maintainers - to install the 
#  application, head up to https://dystroy.org/dysk/install/)

# build the release zip
./release.sh

version=$(./version.sh)

# # deploy on dystroy.org
scp -r build/* dys@dystroy.org:~/prod/www.dystroy.org/dysk/download/
scp "dysk_$version.zip"  dys@dystroy.org:~/prod/www.dystroy.org/dysk/download/
