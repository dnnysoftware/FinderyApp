export function clearMapSearch() {
    document.getElementById("device-name").value = "";
    document.getElementById("search-name-result").value = "";
    document.getElementById("search-desc-result").value = "";
    document.getElementById("search-lat-result").value = "";
    document.getElementById("search-long-result").value = "";
    document.getElementById("search-life-result").value = "";
    document.getElementById("search-time-result").value = "";
}

export function updateMap(url_together, device_name, long, lat, description, battery_life, time) {
    var map = document.getElementById("mapping-structure");
    map.innerHTML = "";
    var temp = `<iframe id="map-visual" width="500" height="450" frameborder="5" scrolling="yes" marginheight="0" marginwidth="0" src="${url_together}" />`
    map.innerHTML = temp;
    document.getElementById("search-name-result").value = device_name;
    document.getElementById("search-desc-result").value = description;
    document.getElementById("search-lat-result").value = lat;
    document.getElementById("search-long-result").value = long;
    document.getElementById("search-life-result").value = battery_life;
    document.getElementById("search-time-result").value = time;
}

