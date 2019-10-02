# make the string_defaults.rs file so that main.js gets compiled into the lib.

mkdir -p tempwk
cp index.template.html tempwk/ihttemp
cd tempwk
ambr {{elm-main}} --rep-file ../../static/main.js ihttemp --no-interactive
mv ihttemp ../../example/index.html
cd ..
rmdir tempwk
