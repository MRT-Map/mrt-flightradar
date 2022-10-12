import L from "leaflet";
import {
  ActiveFlight,
  ActiveFlightInfo,
  FlightAction,
  FromLoc,
  planes,
  setPlanes,
} from "./load-data";
import { map, mapcoord2 } from "./map";

function schedule(fn: () => void, timestamp: number) {
  let delta = timestamp * 1000 - Date.now();
  if (delta < 0) {
    console.warn(`Scheduling a date in the past, ${delta}`);
  }
  setTimeout(fn, delta);
}

function popup(info: ActiveFlightInfo, uuid: string): string {
  let airlineName = `<b>${info.airline_name}</b>`;
  let route = `<b>${info.from}</b> → <b>${info.to}</b>`;
  let waypoints = `<small>via ${info.waypoints
    .map((w) => w.name)
    .join(", ")}</small>`; // TODO temp
  let aircraftReg = `on a(n) ${info.aircraft}`;
  let id = `<i><small>ID: ${uuid}</small></i>`;
  return [airlineName, route, waypoints, aircraftReg, id].join("<br>");
}

function addMarker(flight: ActiveFlight, loc: [number, number]) {
  flight.marker = L.circleMarker(mapcoord2(loc), {
    radius: 5,
  })
    .bindPopup(popup(flight.info, flight.id), { autoPan: false })
    .addTo(map);
}

function movePlane(time: number, vec: FromLoc, flight: ActiveFlight) {
  if (flight.marker === undefined) {
    addMarker(flight, vec.tail);
    planes[planes.findIndex((p) => p.id === flight.id)] = flight;
    setPlanes(planes);
  }
  for (let i = 0; i <= 4.75; i += 0.25) {
    schedule(() => {
      console.log(`Moving ${flight.id}`);
      flight.marker?.setLatLng(
        mapcoord2([
          vec.tail[0] + (vec.vec[0] * i) / 5,
          vec.tail[1] + (vec.vec[1] * i) / 5,
        ]),
      );
      //planes[planes.findIndex(p => p.id === flight.id)] = flight
      //setPlanes(planes)
    }, time + i);
  }
}

export function processActions(time: number, actions: FlightAction[]) {
  for (let action of actions) {
    switch (action.type) {
      case "Add":
        let addedFlight = action.flight;
        let vec = action.vec;
        planes.push(addedFlight);
        setPlanes(planes);
        console.log(`Added plane for ${addedFlight.id}`);
        movePlane(time, vec, addedFlight);
        console.log(`Scheduled move for plane ${addedFlight.id} at ${time}`);
        break;
      case "Move":
        let moveId = action.id;
        let movedFlight = planes.find((p) => p.id === moveId);
        if (movedFlight !== undefined) {
          movePlane(time, action.vec, movedFlight);
        } else {
          console.error(`Unable to find ${moveId}`);
        }
        console.log(`Scheduled move for plane ${moveId} at ${time}`);
        break;
      case "Remove":
        let removeId = action.id;
        schedule(() => {
          planes.find((p) => p.id === removeId)?.marker?.remove();
          setPlanes(planes.filter((p) => p.id != removeId));
        }, time);
        console.log(`Scheduled remove for plane ${action.id} at ${time}`);
        break;
    }
  }
}
