# make the string_defaults.rs file so that main.js gets compiled into the lib.

mkdir -p tempwk
cp index.template.html tempwk/ihttemp
cd tempwk
ambr {{elm-main}} --rep-file ../../static/main.js ihttemp --no-interactive
cp ../../src/string_defaults.rs.template string_defaults.rs
ambr {{index.html}} --rep-file ihttemp string_defaults.rs --no-interactive
mv string_defaults.rs ../../src/string_defaults.rs
rm ihttemp
cd ..
rmdir tempwk
