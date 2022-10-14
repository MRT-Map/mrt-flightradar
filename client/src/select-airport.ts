import { airports, planes } from "./load-data";
import { map } from "./map";
import { resetAirportPanel, sidebar, updateAirportPanel } from "./panel";

map.on("popupopen", async (e) => {
  //@ts-ignore
  let marker = e.popup._source;
  let airport = airports.find(([_, m]) => marker == m);
  if (airport === undefined) return;
  updateAirportPanel(airport[0]);
  if (window.innerWidth >= 768) sidebar.open("panel-airport");
});

map.on("popupclose", (e) => {
  //@ts-ignore
  let marker = e.popup._source;
  let airport = airports.find(([_, m]) => marker == m);
  if (airport === undefined) return;
  resetAirportPanel();
});

function airportOpenPlane(button: HTMLButtonElement) {
  let id = button.getAttribute("plane-id") ?? "";
  let plane = planes.find((p) => p.id == id);
  if (plane === undefined) console.error(`No id ${id}`);
  else {
    map.closePopup();
    setTimeout(() => plane?.marker?.fire("click"), 0);
  }
}
//@ts-ignore
window.airportOpenPlane = airportOpenPlane;
