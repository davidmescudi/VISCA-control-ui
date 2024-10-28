import type { Load } from '@sveltejs/kit';

export type CameraPosition = {
    x: number;
    y: number;
    z: number;
};

export const load: Load = async ({ fetch }) => {
    let position: CameraPosition;

    try {
        const response = await fetch('http://127.0.0.1:8000/api/camera/position');
        if (!response.ok) {
            throw new Error('Failed to fetch camera position');
        }
        position = await response.json();
        console.log(position)
    } catch (error) {
        position = { x: 0, y: 0, z: 0 };
        console.error('Error fetching camera position:', error);
    }

    return { position };
};