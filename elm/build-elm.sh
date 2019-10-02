#  this standard preamble doens't work in nixos.
#  #!/bin/bash

# clear the terminal window for a fresh compile.
clear

: '
# allow dirty repos, but not in production mode.
if [ "$1" = "--optimize" ]; then
  DIRTY=""
else
  DIRTY="--dirtyok"
fi
  
# get git dependencies.  Or if we have them already, do nothing.
mkdir gitdeps -p
dl=("git-at-revision cbbb2ee3ab7cdf41b95b9e5d41bb305021fb071d git@github.com:bburdette/dial-a-log.git gitdeps/dial-a-log $DIRTY")
ec=("git-at-revision a7ebd901230b7d9c6e3e921406e47caf86c17342 git@github.com:bburdette/elm-common.git gitdeps/elm-common $DIRTY")

# everything ok?
if !($dl && $ec)
then
  echo "git dependency problems! exiting build."
  exit 1
fi
'

# this seems to help clear the screen before the build.
# 'clear' alone sometimes doesn't happen until the build is almost done.
echo "starting elm make... wait for 'build complete'!"

# do the build!
time elm make src/Main.elm --output ../static/main.js $1 2> >(tee build-elm-out.txt)

grep -q "elm: not enough bytes" build-elm-out.txt
if [ $? -eq 0 ] 
then
  echo "deleting elm-stuff and retrying compile!"
  rm -rf elm-stuff
  time elm make src/Main.elm --output ../static/main.js $1 2> >(tee build-elm-out.txt)
fi 


# optionally update the rust string_default.rs
# ./make-string-defaults.sh

# make ../example/index.html
./make-example-html.sh

# print this because elm doesn't print a message when the link
# step is finally done.
echo build complete! 
