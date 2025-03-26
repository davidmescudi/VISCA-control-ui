<script lang="ts">
    import type { Camera } from '../types/camera';
    import { writable } from 'svelte/store';
    import { colord } from 'colord';
    import ColorPicker from 'svelte-awesome-color-picker';
    import { onMount } from 'svelte';

    export let cameras = writable<Camera[]>([]);
    
    async function saveColor(camera: Camera, hex: string): Promise<boolean> {
        camera.color = hex;
        console.log(camera.color);
        // TODO: Implement update on backend
        try {
            const response = await fetch(`http://127.0.0.1:8000/api/camera/update/color/${camera.id}`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Cache-Control': 'no-cache, no-store, must-revalidate',
                    'Pragma': 'no-cache',
                    'Expires': '0'
                },
                body: JSON.stringify(camera.color)
            });
            
            if (response.ok) {
                return true;
            } else {
                console.error(await response.text());
                return false;
            }
        } catch (error) {
            console.error('Error updating camera color:', error);
            return false;
        }
    }
</script>

<style>
	.dark {
		--cp-bg-color: #333;
		--cp-border-color: white;
		--cp-text-color: white;
		--cp-input-color: #555;
		--cp-button-hover-color: #777;
	}
</style>

<div class="text-white text-sm">
    {#each $cameras as camera}
        <div class="flex justify-between p-3 border-b border-neutral-700">
            <div class="flex flex-col">
                <span class="font-bold pb-1">{camera.name}</span>
                <div class="flex flex-col text-xs text-neutral-500">
                    <div class="flex flex-row items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="m11.25 11.25.041-.02a.75.75 0 0 1 1.063.852l-.708 2.836a.75.75 0 0 0 1.063.853l.041-.021M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-3.75h.008v.008H12V8.25Z" />
                        </svg>
                        <span class="pl-2">{camera.id}</span>
                    </div>
                    <div class="flex flex-row items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418" />
                        </svg>
                        <span class="pl-2">{camera.ip_address}</span>                         
                    </div>
                </div>
            </div>
            <div class="relative overflow-visible dark">
                <ColorPicker
                    color={colord(camera.color || '#FF5722')}
                    position="responsive"
                    onInput={(event) => event.hex && saveColor(camera, event.hex)}
                />
            </div>
        </div>
    {/each}
</div>