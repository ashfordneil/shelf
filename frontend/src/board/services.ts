import axios from "axios";

import { axiosConfig } from "../config";
import { Board } from "./models";

export const get = async (id: string): Promise<Board | null> => {
    try {
        const output = await axios.get(`/board/${id}`, axiosConfig);
        return output.data as Board;
    } catch (error) {
        return null;
    }
}
