import L from "leaflet";
import { processActions } from "./actions";
import getMsgPack from "./get-msgpack";
import { map, mapcoord2 } from "./map";

export type FromLoc = {
  tail: [number, number];
  vec: [number, number];
};

export type AirportCode = string;
export type Id = string;

export type Waypoint = {
  name: string;
  coords: [number, number];
};

export type ActiveFlightInfo = {
  airline_name: string;
  aircraft: string;
  registry_code: string;
  from: AirportCode;
  to: AirportCode;
  waypoints: Waypoint[];
};

export type ActiveFlight = {
  id: Id;
  depart_time: number;
  arrival_time: number;
  info: ActiveFlightInfo;
  marker?: L.CircleMarker;
};

export type FlightAction =
  | { type: "Add"; flight: ActiveFlight; vec: FromLoc }
  | { type: "Move"; id: Id; vec: FromLoc }
  | { type: "Remove"; id: Id };

export const URL = import.meta.env.PROD
  ? "https://mrt-flightradar.iiiii7d.repl.co/"
  : "http://localhost:8000/";

export var planes: ActiveFlight[] = [];
export var prevActions: Record<string, FlightAction[]> = {};
export var airports: [string, L.CircleMarker][] = [];

export function setPlanes(n: ActiveFlight[]) {
  planes = n;
}

let response = await getMsgPack<ActiveFlight[]>(URL + "flights").catch(
  console.error,
);
if (response === undefined) {
  console.log("Failed to get flights");
} else {
  planes = response;
}

async function updateActions() {
  let response = await getMsgPack<Record<string, FlightAction[]>>(
    URL + "actions",
  ).catch(console.error);
  if (response === undefined) {
    console.log("Failed to get new actions");
    return;
  }
  let newActionsSet = Object.entries(response).filter(
    ([k, _]) => !Object.keys(prevActions).includes(k),
  );
  console.log(`Retrieved ${newActionsSet.length} new action sets`);
  for (let [time, newActions] of newActionsSet) {
    console.log(`Processing ${newActions.length} actions at ${time}`);
    processActions(parseInt(time), newActions);
  }
  prevActions = response;
}
updateActions();
setInterval(updateActions, 30_000);

let response2 = await getMsgPack<Record<string, [number, number]>>(
  URL + "airports",
).catch(console.error);
if (response2 === undefined) {
  console.log("Failed to get airports");
} else {
  airports = Object.entries(response2).map(([airport, pos]) => {
    return [
      airport,
      L.circleMarker(mapcoord2(pos), {
        radius: 4,
        color: "yellow",
        fillColor: "yellow",
      })
        .bindPopup(airport)
        .addTo(map),
    ];
  });
}
