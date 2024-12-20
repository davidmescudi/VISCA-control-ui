import { get } from 'svelte/store';
import { cameraPresets } from '../stores/cameraPresets';
import type { CameraPreset } from '../types/cameraPreset';

export async function addCameraPreset(): Promise<void> {
    // Find the highest id in the current presets and add 1 to it, as previous apporach could lead to duplicate ids
    // ? This is not the best way to generate unique ids, but it works for now
    const existingPresets = get(cameraPresets);
    const existingIds = existingPresets.map(preset => preset.id);
    // If there are no presets, start from 1 otherwise add 1 to the highest id
    const newId = existingIds.length > 0 ? Math.max(...existingIds) + 1 : 1;

    const newPreset: CameraPreset = {
        id: newId,
        workspace_position: { x: 0, y: 0 },
        camera_settings: {zoom: 0, position: {x: 0,y:0}},
        name: '' 
    };

    const response = await fetch('http://127.0.0.1:8000/api/camera_preset/insert', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Cache-Control': 'no-cache, no-store, must-revalidate',
            'Pragma': 'no-cache',
            'Expires': '0'
        },
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

export async function updateCameraPreset(cameraPreset: CameraPreset): Promise<boolean> {
    const response = await fetch(`http://127.0.0.1:8000/api/camera_preset/update/${cameraPreset.id}`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Cache-Control': 'no-cache, no-store, must-revalidate',
            'Pragma': 'no-cache',
            'Expires': '0'
        },
        body: JSON.stringify(cameraPreset)
    });
    if (response.ok) {
        const updatedPreset = await response.json();
        cameraPresets.update(cameraPresets => cameraPresets.map(preset => preset.id === updatedPreset.id ? updatedPreset : preset));
        console.log('Updated settings for button', updatedPreset);
        return true;
    } else {
        const errorMessage = await response.text();
        console.error(errorMessage);
        return false;
    }
}

export async function deleteCameraPreset(id: number): Promise<boolean> {
    const response = await fetch(`http://127.0.0.1:8000/api/camera_preset/delete/${id}`, {
        method: 'DELETE',
        headers: {
            'Cache-Control': 'no-cache, no-store, must-revalidate',
            'Pragma': 'no-cache',
            'Expires': '0'
        }
    });

    if (response.ok) {
        cameraPresets.update(presets => presets.filter(preset => preset.id !== id));
        return true;
    } else {
        const errorMessage = await response.text();
        console.error(errorMessage);
        return false;
    }
}