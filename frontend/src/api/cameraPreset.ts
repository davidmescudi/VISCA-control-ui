import { get } from 'svelte/store';
import { cameraPresets } from '../stores/cameraPresets';
import type { CameraPreset } from '../types/cameraPreset';

export async function addCameraPreset(): Promise<void> {
    const newPreset: CameraPreset = {
        id: get(cameraPresets).length + 1,
        workspace_position: { x: 0, y: 0 },
        camera_settings: {zoom: 0, position: {x: 0,y:0}},
        name: '' 
    };

    const response = await fetch('http://127.0.0.1:8000/api/camera_preset/insert', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(newPreset)
    });

    if (response.ok) {
        const updatedPreset = await response.json();
        cameraPresets.update(cameraPresets => [...cameraPresets, updatedPreset]);
    } else {
        const errorMessage = await response.text();
        console.error(errorMessage);
    }
}