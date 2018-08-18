import axios from "axios";

import { axiosConfig } from "../config";
import { NewTile, Tile } from "./models";

export const get = async (id: string): Promise<Tile | null> => {
    try {
        const output = await axios.get(`/tile/${id}`, axiosConfig);
        return {...output.data, id } as Tile;
    } catch (error) {
        return null;
    }
}

export const post = async (tile: NewTile): Promise<string> => {
    const output = await axios.post(`/tile`, tile, axiosConfig);
    return output.data as string;
}
