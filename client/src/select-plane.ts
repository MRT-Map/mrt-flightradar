import { planes, URL } from "./load-data";
import { map, mapcoord2 } from "./map";
import L from "leaflet";
import {
  resetFlightPanel,
  sidebar,
  updateFlightPanel,
  updateFlightPanel2,
} from "./panel";
import getMsgPack from "./get-msgpack";

var flightRoute: L.Polyline | null = null;

map.on("popupopen", async (e) => {
  //@ts-ignore
  let marker = e.popup._source;
  let flight = planes.find((p) => p.marker == marker);
  if (flight === undefined) return;
  updateFlightPanel(flight);
  if (window.innerWidth >= 768) sidebar.open("panel-flight");

  let response = await getMsgPack<[number, number][]>(
    URL + "route/" + flight.id,
  ).catch(console.error);
  if (response === undefined) return;
  updateFlightPanel2(response);

  flightRoute?.remove();
  flightRoute = L.polyline(response.map(mapcoord2), {
    color: "#ff0000",
    weight: 5,
  }).addTo(map);
});

map.on("popupclose", (e) => {
  //@ts-ignore
  let marker = e.popup._source;
  let flight = planes.find((p) => p.marker == marker);
  if (flight === undefined) return;
  flightRoute?.remove();
  flightRoute = null;
  resetFlightPanel();
});
