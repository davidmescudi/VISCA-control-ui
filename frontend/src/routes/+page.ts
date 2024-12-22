import type { Load } from '@sveltejs/kit';
import type { CameraPreset } from '../types/cameraPreset';
import { cameraPresets } from '../stores/cameraPresets';

export const load: Load = async ({ fetch }) => {
    try {
        const response = await fetch('http://127.0.0.1:8000/api/camera_presets');
        if (!response.ok) {
            throw new Error('Failed to fetch camera presets');
        }
        const presets: CameraPreset[] = await response.json();
        cameraPresets.set(presets);
    } catch (error) {
        console.error('Error fetching camera presets:', error);
    }

    return {};
};