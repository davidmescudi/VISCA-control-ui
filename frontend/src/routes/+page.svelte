<script lang="ts">
	// Components import
	import CameraPresetDraggableButton from '../components/CameraPresetDraggableButton.svelte';
	import CameraList from '../components/CameraList.svelte';
	// API call imports
	import { addCameraPreset } from '../api/cameraPreset';
	import { addCamera } from '../api/camera';
	// Store imports
	import { cameraPresets } from '../stores/cameraPresets';
	import { tabletMode } from '../stores/tabletMode';
	import DownloadAsFileButton from '../components/DownloadAsFileButton.svelte';
	import UploadFileButton from '../components/UploadFileButton.svelte';
	import { onMount } from 'svelte';
	import type { Camera } from '../types/camera';
	import { writable } from 'svelte/store';

	const cameras = writable<Camera[]>([]);

	onMount(async () => {
		const response = await fetch('http://127.0.0.1:8000/api/cameras');
		if (response.ok) {
			const data: Camera[] = await response.json();
			cameras.set(data);
		} else {
			console.error('Failed to fetch cameras');
		}
	});

	function toggleTabletMode() {
		tabletMode.update((value) => !value);
	}

</script>

<div class="flex h-screen bg-transparent">
	<!-- sidebar -->
	<div class="flex flex-col w-64 bg-neutral-950" class:hidden={$tabletMode}>
		<div class="flex items-center justify-center h-16 gap-3">
			<a href="https://www.unibw.de/code" target="_blank" rel="noopener noreferrer">
				<img src="unibw_fi_code_logo.png" alt="FI Code Logo" class="h-11">
			</a>
			<div class="flex flex-col">
				<h2 class="text-base/5 font-semibold text-orange-500">VISCA Protocol</h2>
				<h2 class="text-base/5 font-semibold text-orange-500">Camera Control</h2>
			</div>
		</div>
		<div class="flex flex-col flex-1">
			<nav class="flex-1 py-4 bg-neutral-900 flex flex-col">
				<button class="flex items-center px-4 py-2 text-gray-100 hover:bg-orange-500 w-full" on:click|preventDefault={addCameraPreset}>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="1.5"
						stroke="currentColor"
						class="h-6 w-6 mr-2"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
						/>
					</svg>
					Camera Preset
				</button>
				<CameraList {cameras} />
				<div class="mt-auto border-t-2 border-neutral-700 divide-y-2 divide-neutral-700">
					<div class="flex divide-x-2 divide-neutral-700">
						<DownloadAsFileButton></DownloadAsFileButton>
						<UploadFileButton></UploadFileButton>
					</div>
					<button class="flex items-center px-4 py-2 text-gray-100 hover:bg-orange-500 w-full mt-auto justify-center" on:click={toggleTabletMode}>
						<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6 mr-2">
							<path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5h3m-6.75 2.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-15a2.25 2.25 0 0 0-2.25-2.25H6.75A2.25 2.25 0 0 0 4.5 4.5v15a2.25 2.25 0 0 0 2.25 2.25Z" />
						</svg>
						Tablet Mode
					</button>
				</div>
			</nav>
		</div>
	</div>

	<div class="absolute bottom-0 left-0" class:hidden={!$tabletMode}>
		<button class="flex items-center px-4 py-2 bg-neutral-900 text-gray-100 hover:bg-orange-500 w-full rounded-tr-lg" on:click={toggleTabletMode}>
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6 mr-2">
				<path stroke-linecap="round" stroke-linejoin="round" d="M9 17.25v1.007a3 3 0 0 1-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0 1 15 18.257V17.25m6-12V15a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 15V5.25m18 0A2.25 2.25 0 0 0 18.75 3H5.25A2.25 2.25 0 0 0 3 5.25m18 0V12a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 12V5.25" />
			  </svg>			  
			  Desktop Mode
		</button>
	</div>

	<!-- Main content -->
	<div class="overflow-y-auto bg-neutral-800 pattern w-full h-full">
		{#each $cameraPresets as cameraPreset (cameraPreset.id)}
			<CameraPresetDraggableButton {cameraPreset} cameras={$cameras} />
    	{/each}
	</div>
</div>
