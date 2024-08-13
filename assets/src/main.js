// Alpine init
import Alpine from "alpinejs";

import persist from "@alpinejs/persist";
Alpine.plugin(persist);

window.Alpine = Alpine;
Alpine.start();

// HTMX init
import htmx from "htmx.org";

// TODO it would be nice if this were all TypeScript

if (!Array.prototype.last) {
  Array.prototype.last = function () {
    return this[this.length - 1];
  };
}

htmx.defineExtension("json-enc", {
  onEvent: function (name, evt) {
    if (name === "htmx:configRequest") {
      evt.detail.headers["Content-Type"] = "application/json";
    }
  },

  encodeParameters: function (xhr, parameters, _elt) {
    xhr.overrideMimeType("text/json");

    const dataObject = {};

    // key[0] = a
    // key[1] = b
    // maps to JSON => key: ["a", "b"]

    // key[0].a = "a"
    // key[0].b = "b"
    // key[1].a = "c"
    // key[1].b = "d"
    // maps to JSON => [
    //   { a: "a", b: "b" },
    //   { a: "c", b: "d" }
    // ]
    parameters.forEach((value, key) => {
      const [arrayKey, memberName] = key.split(".", 2);

      if (arrayKey.endsWith("]")) {
        const [newKey, index] = arrayKey.slice(0, -1).split("[", 2);
        if (!dataObject[newKey]) {
          dataObject[newKey] = [];
        }

        if (memberName) {
          if (!dataObject[newKey][index]) {
            dataObject[newKey][index] = {};
          }

          dataObject[newKey][index][memberName] = value;
        } else {
          dataObject[newKey][index] = value;
        }
      } else {
        if (memberName) {
          dataObject[arrayKey][memberName] = value;
        } else {
          dataObject[arrayKey] = value;
        }
      }
    });

    return JSON.stringify(dataObject);
  },
});
