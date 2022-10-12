import L from "leaflet";
import "leaflet-sidebar-v2";
import DoorOpen from "~icons/mdi/door-open?raw";

// copied from our other project, https://github.com/MRT-Map/map/blob/main/scripts/map.js
export var map = L.map("map", {
  crs: L.CRS.Simple,
}).setView([0, 0], 8);

//override the default
//@ts-ignore
L.TileLayer.customTileLayer = L.TileLayer.extend({
  //@ts-ignore
  getTileUrl: function (coords) {
    let Zcoord = 2 ** (8 - coords.z);
    let Xcoord = coords.x * 1;
    let Ycoord = coords.y * -1;

    let group = {
      x: Math.floor((Xcoord * Zcoord) / 32),
      y: Math.floor((Ycoord * Zcoord) / 32),
    };

    let numberInGroup = {
      x: Math.floor(Xcoord * Zcoord),
      y: Math.floor(Ycoord * Zcoord),
    };

    /* console.log(coords);
     console.log(group);
     console.log(numberInGroup);*/

    let zzz = "";

    for (var i = 8; i > coords.z; i--) {
      zzz += "z";
    }

    if (zzz.length != 0) zzz += "_";

    let url = `https://dynmap.minecartrapidtransit.net/tiles/new/flat/${group.x}_${group.y}/${zzz}${numberInGroup.x}_${numberInGroup.y}.png`;
    //console.log(url)
    return url;

    // return L.TileLayer.prototype.getTileUrl.call(this, coords);
  },
});

// static factory as recommended by http://leafletjs.com/reference-1.0.3.html#class
//@ts-ignore
L.tileLayer.customTileLayer = function (templateUrl, options) {
  //@ts-ignore
  return new L.TileLayer.customTileLayer(templateUrl, options);
};

//@ts-ignore
L.tileLayer
  .customTileLayer("unused url; check custom function", {
    maxZoom: 8,
    zoomControl: false, //there's also css to do this bc this line doesn't work
    id: "map",
    tileSize: 128,
    zoomOffset: 0,
    noWrap: true,
    bounds: [
      [-900, -900],
      [900, 900],
    ],
    attribution: "Minecart Rapid Transit",
  })
  .addTo(map);

var sidebar = L.control
  .sidebar({
    closeButton: true,
    container: "sidebar",
    position: "left",
  })
  .addTo(map);
sidebar.addPanel({
  id: "panel-welcome",
  tab: DoorOpen,
  pane: "aaa",
  title: "MRT FlightRadar",
});
sidebar.open("panel-welcome");

export function mapcoord([x, y]: [number, number]): [number, number] {
  let NewX = y / -64 - 0.5;
  let NewY = x / 64;
  return [NewX, NewY];
}
export function worldcoord([x, y]: [number, number]): [number, number] {
  let NewX = y * 64;
  let NewY = (x + 0.5) * -64;
  return [NewX, NewY];
}
