
trunk build --release

git push

cp -r dist/* ../210_web_fella/
cd ../210_web_fella/

git stage *
git commit -m "update website"
git push
