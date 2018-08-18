import axios from "axios";

import { axiosConfig } from "../config";
import { Tile } from "./models";

export const get = async (id: string): Promise<Board | null> => {
    try {
        const output = await axios.get(`/tile/${id}`, axiosConfig);
        return output.data as Board;
    } catch (error) {
        return null;
    }
}

export const post = async (tile: Tile): Promise<string> => {
    const output = await axios.post(`/tile`, tile, axiosConfig);
    return output.data as string;
}
