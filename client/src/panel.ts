import L from "leaflet";
import "leaflet-sidebar-v2";
import { map } from "./map";
import $ from "jquery";
import DoorOpen from "~icons/mdi/door-open?raw";
import Info from "~icons/mdi/flight?raw";
import Airport from "~icons/mdi/airport?raw";
import ArrowTopRightBoldBox from "~icons/mdi/arrow-top-right-bold-box?raw";
import { ActiveFlight, planes } from "./load-data";

export var sidebar = L.control
  .sidebar({
    closeButton: true,
    container: "sidebar",
    position: "left",
  })
  .addTo(map);

sidebar.addPanel({
  id: "panel-welcome",
  tab: DoorOpen,
  pane: $("#welcome")[0].innerHTML,
  title: "MRT FlightRadar",
});

sidebar.addPanel({
  id: "panel-flight",
  tab: Info,
  pane: $("#flight")[0].innerHTML,
  title: "Flight information",
});

export function updateFlightPanel(flight: ActiveFlight) {
  let departTime = new Date(flight.depart_time * 1000);
  let arrivalTime = new Date(flight.arrival_time * 1000);
  let delta_s = flight.arrival_time - flight.depart_time;
  let delta_min = Math.floor(delta_s / 60);
  delta_s -= 60 * delta_min;
  let delta_h = Math.floor(delta_min / 60);
  delta_min -= 60 * delta_h;
  $("#panel-flight > .inner")[0].innerHTML = $("#flight-template > .inner")[0]
    .innerHTML.replace("{from}", flight.info.from)
    .replace("{to}", flight.info.to)
    .replace("{airlineName}", flight.info.airline_name)
    .replace("{aircraft}", flight.info.aircraft)
    .replace("{departTime}", departTime.toLocaleTimeString())
    .replace("{arrivalTime}", arrivalTime.toLocaleTimeString())
    .replace("{duration}", `${delta_h}:${delta_min}:${delta_s}`)
    .replace("{waypoints}", flight.info.waypoints.map((w) => w.name).join(", "))
    .replace("{id}", flight.id.toString());
}
export function updateFlightPanel2(route: [number, number][]) {
  let length = 0;
  for (let i = 0; i < route.length - 1; i++) {
    length += Math.sqrt(
      Math.pow(route[i][0] - route[i + 1][0], 2) +
        Math.pow(route[i][1] - route[i + 1][1], 2),
    );
  }
  $("#panel-flight > .inner")[0].innerHTML = $(
    "#panel-flight > .inner",
  )[0].innerHTML.replace("{distance}", Math.round(length).toString());
}

export function resetFlightPanel() {
  $("#panel-flight > .inner")[0].innerHTML = $("#flight")[0].innerHTML;
}

sidebar.addPanel({
  id: "panel-airport",
  tab: Airport,
  pane: $("#airport")[0].innerHTML,
  title: "Airport information",
});

function getDepartures(airport: string): ActiveFlight[] {
  return planes.filter((p) => p.info.from == airport);
}
function getArrivals(airport: string): ActiveFlight[] {
  return planes.filter((p) => p.info.to == airport);
}
function toTemplate(flight: ActiveFlight, direction: "to" | "from"): string {
  return $("#airport-flight-template")[0]
    .innerHTML.replace("{direction}", direction)
    .replace(
      "{otherAirport}",
      direction == "to" ? flight.info.to : flight.info.from,
    )
    .replace(
      "{time}",
      new Date(
        (direction == "to" ? flight.depart_time : flight.arrival_time) * 1000,
      ).toLocaleTimeString(),
    )
    .replace("{airlineName}", flight.info.airline_name)
    .replace("{id}", flight.id)
    .replace("{icon}", ArrowTopRightBoldBox);
}
export function updateAirportPanel(airport: string) {
  $("#panel-airport > .inner")[0].innerHTML = $("#airport-template > .inner")[0]
    .innerHTML.replace("{code}", airport)
    .replace(
      "{departures}",
      getDepartures(airport)
        .map((a) => toTemplate(a, "to"))
        .join(""),
    )
    .replace(
      "{arrivals}",
      getArrivals(airport)
        .map((a) => toTemplate(a, "from"))
        .join(""),
    );
}
export function resetAirportPanel() {
  $("#panel-airport > .inner")[0].innerHTML = $("#airport")[0].innerHTML;
}

sidebar.open("panel-welcome");
