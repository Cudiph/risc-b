#!/bin/sh

cd ./dist/webapp/
rm ./{manifest.json,background,_locales} -r
rm ./assets/background* 
mv ./popup/index.html ./
rmdir ./popup
rm browser-polyfill.min.js

