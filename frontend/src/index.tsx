import * as React from "react";

import * as board from "./board/services";

const main = async () => {
    const x = await board.get("7bbd0911-a374-4ba4-a650-59b843d6fb9a");
    console.log(x);
}

main();
