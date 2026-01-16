# This script is used to maintain dysk official doc at https://dystroy.org/dysk
#
# Obviously, it requires some rights over the server to be ran

# build the site
ddoc

# deploy it on dystroy.org
scp -r site/* dys@dystroy.org:~/prod/www.dystroy.org/dysk/
