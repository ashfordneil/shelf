import { Tile } from "../tile/models";

export interface RawBoard {
    title: string;
    tiles: string[];
}

export interface Board {
    tiles: Tile[];
}
