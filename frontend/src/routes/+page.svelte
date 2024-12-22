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

	function toggleTabletMode() {
		tabletMode.update((value) => !value);
	}

</script>

<div class="flex h-screen bg-transparent">
	<!-- sidebar -->
	<div class="flex flex-col w-64 bg-neutral-950" class:hidden={$tabletMode}>
		<div class="flex items-center justify-center h-16 gap-3">
			<img
				src="https://www.unibw.de/code/@@images/02539a69-4f5e-4965-9a55-226f3437cf06.png"
				alt="FI Code Logo"
				class="h-11"
			/>
			<div class="flex flex-col">
				<h2 class="text-base/5 font-semibold text-orange-500">VISCA Protocol</h2>
				<h2 class="text-base/5 font-semibold text-orange-500">Camera Control</h2>
			</div>
		</div>
		<div class="flex flex-col flex-1 overflow-y-auto">
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
				<button class="flex items-center px-4 py-2 text-gray-100 hover:bg-orange-500 w-full" on:click|preventDefault={addCamera}>
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
							d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z"
						/>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
						/>
					</svg>
					Add Camera
				</button>
				<CameraList />
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
        	<CameraPresetDraggableButton {cameraPreset} />
    	{/each}
	</div>
</div>
