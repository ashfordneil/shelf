import axios from "axios";

import { axiosConfig } from "../config";
import { RawBoard, Board } from "./models";

import * as tile from "../tile/services";

export const get = async (id: string): Promise<Board | null> => {
    try {
        const raw = await axios.get(`/board/${id}`, axiosConfig);
        const shallow =  raw.data as Board;
        const tiles = await Promise.all(shallow.tiles.map(id => tile.get(id)));
        return { tiles };
    } catch (error) {
        return null;
    }
}

export const post = async (title: string): Promise<stirng> => {
    const output = await axios.post(`/board/${title}`, axiosConfig);
    return output.data as string;
}
