import { planes, URL } from "./load-data";
import { map, mapcoord2 } from "./map";
import axios from "axios";
import L from "leaflet";

var flightRoute: L.Polyline | null = null;

map.on("popupopen", async (e) => {
  //@ts-ignore
  let marker = e.popup._source;
  let flight = planes.find((p) => p.marker === marker);
  if (flight === undefined) return;

  let response = await axios
    .get<[number, number][]>(URL + "route/" + flight.id)
    .catch(console.error);
  if (response === undefined) return;
  console.error(response.data);

  flightRoute?.remove();
  flightRoute = L.polyline(response.data.map(mapcoord2), {
    color: "#ff0000",
    weight: 5,
  }).addTo(map);
});

map.on("popupclose", (_) => {
  flightRoute?.remove();
  flightRoute = null;
});
