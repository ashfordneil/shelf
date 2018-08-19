import axios from "axios";

import { axiosConfig } from "../config";
import { RawBoard, Board } from "./models";

import * as tile from "../tile/services";

export const get = async (id: string): Promise<Board | null> => {
    try {
        const raw = await axios.get(`/board/${id}`, axiosConfig);
        const shallow =  raw.data as Board;
        const tiles = await Promise.all(shallow.tiles.map(id => tile.get(id)));
        return { title: shallow.title, tiles };
    } catch (error) {
        return null;
    }
}

export const post = async (title: string): Promise<string> => {
    const newBoard: Board = {
        title,
        tiles: []
    };
    const output = await axios.post(`/board`, newBoard, axiosConfig);
    return output.data as string;
}

export const checkout = async (id: string): Promise<string> => {
    const output = await axios.post(`/board/${id}/edit`, axiosConfig);
    return output.data as string;
}

export const checkin = async (id: string, jwt: string, body: Board): Promise<void> => {
    const config = JSON.parse(JSON.stringify(axiosConfig)) as typeof axiosConfig;
    config.headers["auth"] = jwt;
    await axios.patch(`/board/${id}`, body, config);
}

export const cheekyupdate = async (id: string, body: Board): Promise<void> => {
    const jwt = await checkout(id);
    await checkin(id, jwt, body);
}