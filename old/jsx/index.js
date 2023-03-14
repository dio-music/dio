'use strict';

// const { main_js } = require("../pkg/index.js");

let wasm = import("../pkg/index.js")
// import * as zip from "https://deno.land/x/zipjs/index.js";
// import * as zip from "@zip.js/zip.js";
var unzip = require('unzip-js')

const e = React.createElement;

class OpenZipButton extends React.Component {
    constructor(props) {
        super(props);
        this.state = { liked: false };
    }

    render() {
        if (this.state.liked) {
            return 'You liked this.';
        }

        return (
            // <button onClick={() => true}>
            //     Open Folder
            // </button>
            <input type="file" onChange={(e) => {

                let zip_file = e.target.files.item(0);


                // const fread = FileReader().read_as
                // const zipFileReader = new BlobReader(zipFileBlob);
                console.log(zip_file);
                // console.log(zipFileReader);

                // var b = new Blob()
                // reader.readAsBinaryString(b)

                // console.log(b)




                unzip(zip_file, function (err, zipFile) {
                    if (err) {
                        return console.error(err)
                    }

                    zipFile.readEntries(function (err, entries) {
                        if (err) {
                            return console.error(err)
                        }

                        entries.forEach(function (entry) {

                            // console.log(entry.name)

                            if (false) {
                                return
                            }

                            const chunks = [];

                            zipFile.readEntryData(entry, false, function (err, readStream) {
                                if (err) {
                                    return console.error(err)
                                }

                                readStream.on('data', function (chunk) {
                                    chunks.push(Buffer.from(chunk))
                                })
                                readStream.on('error', function (err) { reject(err) })
                                readStream.on('end', function () { })

                                console.log(chunks)
                            })


                            const a = Buffer.concat(chunks).toString('utf8')

                            console.log(a)
                        })
                    })
                })

            }} />
        );
    }
}

const domContainer = document.querySelector('#dom_container');
const root = ReactDOM.createRoot(domContainer);
root.render(e(OpenZipButton));

console.log("WOW!")
