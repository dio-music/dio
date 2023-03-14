'use strict';

// const { main_js } = require("../pkg/index.js");

var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

function _possibleConstructorReturn(self, call) { if (!self) { throw new ReferenceError("this hasn't been initialised - super() hasn't been called"); } return call && (typeof call === "object" || typeof call === "function") ? call : self; }

function _inherits(subClass, superClass) { if (typeof superClass !== "function" && superClass !== null) { throw new TypeError("Super expression must either be null or a function, not " + typeof superClass); } subClass.prototype = Object.create(superClass && superClass.prototype, { constructor: { value: subClass, enumerable: false, writable: true, configurable: true } }); if (superClass) Object.setPrototypeOf ? Object.setPrototypeOf(subClass, superClass) : subClass.__proto__ = superClass; }

var wasm = import("../pkg/index.js");
// import * as zip from "https://deno.land/x/zipjs/index.js";
// import * as zip from "@zip.js/zip.js";
var unzip = require('unzip-js');

var e = React.createElement;

var OpenZipButton = function (_React$Component) {
    _inherits(OpenZipButton, _React$Component);

    function OpenZipButton(props) {
        _classCallCheck(this, OpenZipButton);

        var _this = _possibleConstructorReturn(this, (OpenZipButton.__proto__ || Object.getPrototypeOf(OpenZipButton)).call(this, props));

        _this.state = { liked: false };
        return _this;
    }

    _createClass(OpenZipButton, [{
        key: 'render',
        value: function render() {
            if (this.state.liked) {
                return 'You liked this.';
            }

            return (
                // <button onClick={() => true}>
                //     Open Folder
                // </button>
                React.createElement('input', { type: 'file', onChange: function onChange(e) {

                        var zip_file = e.target.files.item(0);

                        // const fread = FileReader().read_as
                        // const zipFileReader = new BlobReader(zipFileBlob);
                        console.log(zip_file);
                        // console.log(zipFileReader);

                        // var b = new Blob()
                        // reader.readAsBinaryString(b)

                        // console.log(b)


                        unzip(zip_file, function (err, zipFile) {
                            if (err) {
                                return console.error(err);
                            }

                            zipFile.readEntries(function (err, entries) {
                                if (err) {
                                    return console.error(err);
                                }

                                entries.forEach(function (entry) {

                                    // console.log(entry.name)

                                    if (false) {
                                        return;
                                    }

                                    var chunks = [];

                                    zipFile.readEntryData(entry, false, function (err, readStream) {
                                        if (err) {
                                            return console.error(err);
                                        }

                                        readStream.on('data', function (chunk) {
                                            chunks.push(Buffer.from(chunk));
                                        });
                                        readStream.on('error', function (err) {
                                            reject(err);
                                        });
                                        readStream.on('end', function () {});

                                        console.log(chunks);
                                    });

                                    var a = Buffer.concat(chunks).toString('utf8');

                                    console.log(a);
                                });
                            });
                        });
                    } })
            );
        }
    }]);

    return OpenZipButton;
}(React.Component);

var domContainer = document.querySelector('#dom_container');
var root = ReactDOM.createRoot(domContainer);
root.render(e(OpenZipButton));

console.log("WOW!");