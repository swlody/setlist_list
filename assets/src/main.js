// Alpine init
import Alpine from "alpinejs";

import persist from "@alpinejs/persist";
Alpine.plugin(persist);

window.Alpine = Alpine;
Alpine.start();

// HTMX init
import htmx from "htmx.org";

htmx.defineExtension("json-enc", {
  onEvent: function (name, evt) {
    if (name === "htmx:configRequest") {
      evt.detail.headers["Content-Type"] = "application/json";
    }
  },

  encodeParameters: function (xhr, parameters, _elt) {
    xhr.overrideMimeType("text/json");

    const dataObject = {};

    parameters.forEach((value, key) => {
      if (key.endsWith("[]")) {
        const arrayKey = key.slice(0, -2);
        if (!dataObject[arrayKey]) {
          dataObject[arrayKey] = [];
        }
        dataObject[arrayKey].push(value);
      } else {
        dataObject[key] = value;
      }
    });

    return JSON.stringify(dataObject);
  },
});
