export interface NewTile {
    title: string;
    content: string;
}

export interface Tile extends NewTile {
    id: string;
}
