(npx babel --watch jsx --out-dir _js --presets react-app/prod) &
npm run start
trap ctrl_c INT

ctrl_c () {
    echo "Ctrl + C happened"
    kill $(jobs -p)
}
