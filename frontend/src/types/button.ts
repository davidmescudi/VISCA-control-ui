export type Button = {
    id: number;
    camera_id?: number;
    workspace_position: {
        x: number;
        y: number;
    };
    camera_settings: {
        zoom: number;
        position: {
            x: number;
            y: number
        };
    };
    name: string;
};