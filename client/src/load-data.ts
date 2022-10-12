import axios from "axios";
import { processActions } from "./actions";

export type FromLoc = {
  tail: [number, number];
  vec: [number, number];
};

export type AirportCode = string;
export type Id = string;

export type ActiveFlightInfo = {
  airline_name: string;
  aircraft: string;
  registry_code: string;
  from: AirportCode;
  to: AirportCode;
};

export type ActiveFlight = {
  id: Id;
  depart_time: number;
  arrive_time: number;
  info: ActiveFlightInfo;
  marker?: L.CircleMarker;
};

export type FlightAction =
  | { type: "Add"; flight: ActiveFlight; vec: FromLoc }
  | { type: "Move"; id: Id; vec: FromLoc }
  | { type: "Remove"; id: Id };

const URL = "http://localhost:8000/";
export var planes: ActiveFlight[] = [];
export var prevActions: Record<string, FlightAction[]> = {};

export function setPlanes(n: ActiveFlight[]) {
  planes = n;
}

let response = await axios
  .get<ActiveFlight[]>(URL + "flights")
  .catch(console.error);
if (response === undefined) {
  console.log("Failed to get flights");
} else {
  planes = response.data;
}

async function updateActions() {
  let response = await axios
    .get<Record<string, FlightAction[]>>(URL + "actions")
    .catch(console.error);
  if (response === undefined) {
    console.log("Failed to get new actions");
    return;
  }
  let newActionsSet = Object.entries(response.data).filter(
    ([k, _]) => !Object.keys(prevActions).includes(k),
  );
  console.log(`Retrieved ${newActionsSet.length} new action sets`);
  for (let [time, newActions] of newActionsSet) {
    console.log(`Processing ${newActions.length} actions at ${time}`);
    processActions(parseInt(time), newActions);
  }
  prevActions = response.data;
}
updateActions();
setInterval(updateActions, 30_000);
