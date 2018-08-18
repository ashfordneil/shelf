import * as React from "react";

import * as tile from "./tile/services";

const main = async () => {
    const id = await tile.post({ content: "hello" });
    console.log(id);
    const data = await tile.get(id);
    console.log(data);
}

window["main"] = main;
