
mkdir -p tempwk
cp index.template.html tempwk/ihttemp
cd tempwk
ambr {{elm-main}} --rep-file ../../static/main.js ihttemp --no-interactive
mv ihttemp ../../example/index.html
cd ..
rmdir tempwk
