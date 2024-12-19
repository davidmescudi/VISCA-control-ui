import { get } from 'svelte/store';
import { cameras } from '../stores/camera';
import type { Camera } from '../types/camera';

// TODO: Implement backend calls
export async function addCamera(): Promise<void> {
    const newCamera: Camera = {
        id: get(cameras).length + 1,
        name: '' 
    };

    cameras.update(cameras => [
        ...cameras, newCamera
    ]);
}