import { Tile } from "../tile/models";

export interface RawBoard {
    tiles: string[];
}

export interface Board {
    tiles: Tile[];
}
