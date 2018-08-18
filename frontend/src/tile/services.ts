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

export const checkout = async (id: string): Promise<string> => {
    const output = await axios.post(`/tile/${id}/edit`, axiosConfig);
    return output.data as string;
}

export const checkin = async (id: string, jwt: string, body: Tile): Promise<void> => {
    const config = JSON.parse(JSON.stringify(axiosConfig)) as typeof axiosConfig;
    config.headers["auth"] = jwt;
    await axios.patch(`/tile/${id}`, body, axiosConfig);
}
