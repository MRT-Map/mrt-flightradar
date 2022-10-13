import { airports } from "./load-data";
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

map.on("popupclose", (_) => {
  resetAirportPanel();
});
