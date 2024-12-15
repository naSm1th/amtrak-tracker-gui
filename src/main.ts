import { listen } from '@tauri-apps/api/event';

enum Station {
  StPaul = "STP",
  RedWing = "RDW",
  Winona = "WIN",
  LaCrosse = "LSE",
  Tomah = "TOH",
  WisconsinDells = "WDL",
  Portage = "POG",
  Columbus = "CBS",
  Milwaukee = "MKE",
  MilwaukeeAirport = "MKA",
  Sturtevant = "SVT",
  Chicago = "CHI"
}

enum StationState {
  Empty = "Empty",
  Incoming = "Incoming",
  Stopped = "Stopped"
}

enum Train {
  Hiawatha = "Hiawatha",
  Borealis = "Borealis",
  EmpireBuilder = "EmpireBuilder"
}

type StationStateForTrain = {
  train: Train,
  state: StationState
};

type StationStateUpdate = {
  station: Station;
  state: StationStateForTrain[];
};

function StationToId(station: Station): string {
  switch (station) {
    case Station.StPaul:
      return "St_Paul";
    case Station.RedWing:
      return "Red_Wing";
    case Station.Winona:
      return "Winona";
    case Station.LaCrosse:
      return "La_Crosse";
    case Station.Tomah:
      return "Tomah";
    case Station.WisconsinDells:
      return "Wisconsin_Dells";
    case Station.Portage:
      return "Portage";
    case Station.Columbus:
      return "Columbus";
    case Station.Milwaukee:
      return "Milwaukee";
    case Station.MilwaukeeAirport:
      return "Milwaukee_Airport";
    case Station.Sturtevant:
      return "Sturtevant";
    case Station.Chicago:
      return "Chicago";
    default:
      return "";
  }
}

listen<StationStateUpdate>('station-update', (event) => {
  let stationEl = document.querySelector("#map #Stations #" + StationToId(event.payload.station) + " circle");

  if (stationEl) {
    let classList: string = "";
    for (let trainState of event.payload.state) {
      let train: Train = trainState.train;
      let state: StationState = trainState.state;
      
      if (state != StationState.Empty) {
        let className: string = train.toString() + state.toString();
        console.log("Setting class ", className);
        classList += className + " ";
      }
    }
    if (classList == "") {
      console.log("Setting to empty");
      classList = "cls-4";
    }
    console.log("Current class is " + stationEl.classList);
    stationEl.setAttribute("class", classList.trimEnd());
    console.log("New class is " + stationEl.classList);
  }
});
