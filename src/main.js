const { invoke } = window.__TAURI__.tauri;

async function load_spotify_data() {
  try {
    document.getElementById("folder-selection-general-status-msg").innerHTML = "Loading..."
    document.getElementById("folder-selection-extended-status-msg").innerHTML = ""
    await invoke("load_spotify_data");
    window.location.assign("dashboard.html");
  } catch (err) {
    document.getElementById("folder-selection-general-status-msg").innerHTML = "There was an error while trying to load the folder. Make sure you selected an unzipped copy of the zip file provided by Spotify. Please try again.";
    document.getElementById("folder-selection-extended-status-msg").innerHTML = err
  }
  // document.getElementById("folder-selection-general-status-msg").innerHTML = "There was an error while trying to load the folder. Make sure you selected an unzipped copy of the zip file provided by Spotify. Please try again.";
  // document.getElementById("folder-selection-extended-status-msg").innerHTML = "ERR ERR ERR ERR"
}

window.load_spotify_data = load_spotify_data
