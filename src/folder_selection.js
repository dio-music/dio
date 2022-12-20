const { invoke } = window.__TAURI__.tauri;

let spotify_data_folder_path;

async function select_spotify_data_folder() {
  spotify_data_folder_path = await invoke("select_spotify_data_folder")

  if (spotify_data_folder_path === null) {
    document.getElementById("folder-selection-msg").innerHTML = "There was an error while trying to load the folder. Make sure you selected an unzipped copy of the zip file provided by Spotify. Please try again.";
  } else {
    // document.getElementById("folder-selection-msg").innerHTML = spotify_data_folder_path
    window.location.assign("dashboard.html")
  }
}

window.select_spotify_data_folder = select_spotify_data_folder
