<script lang="ts">
	import { onMount } from 'svelte';
	import interact from 'interactjs';
	import type { CameraPreset } from '../types/cameraPreset';
	import { updateCameraPreset, deleteCameraPreset } from '../api/cameraPreset';

	export let cameraPreset: CameraPreset;

	let showSettings = false;

	async function handleSubmit(event: Event) {
		const submitSuccess = await updateCameraPreset(cameraPreset);
		if (submitSuccess) toggleSettings();
	}

	async function handeDelete(event: Event) {
		const deleteSuccess = await deleteCameraPreset(cameraPreset.id);
		if (deleteSuccess) toggleSettings();
	}

	function handleBlur(event: FocusEvent) {
		updateCameraPreset(cameraPreset);
	}

	function toggleSettings() {
		showSettings = !showSettings;
	}

	onMount(() => {
		interact(`.draggable-${cameraPreset.id}`).draggable({
			allowFrom: `.handle-${cameraPreset.id}`,
			listeners: {
				move(event) {
					cameraPreset.workspace_position.x += event.dx;
					cameraPreset.workspace_position.y += event.dy;
					event.target.style.transform = `translate(${cameraPreset.workspace_position.x}px, ${cameraPreset.workspace_position.y}px)`;
				},
				end(event) {
					updateCameraPreset(cameraPreset);
				}
			}
		});
	});
</script>

<div class="rounded-lg border border-neutral-700 bg-neutral-900 p-4 draggable draggable-{cameraPreset.id} w-max absolute" style="transform: translate({cameraPreset.workspace_position.x}px, {cameraPreset.workspace_position.y}px); z-index: {showSettings ? 1000 : 10};">
	<div class="flex items-center gap-4">
		<!-- Play button to execute camera preset -->
		<div class="bg-neutral-800 p-2 rounded-full text-orange-500 hover:bg-orange-500 hover:text-white">
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
				<path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z" />
			  </svg>			  
		</div>
		<!-- Input for camere preset name -->
		<div class="h-fit">
			<label for="CameraPresetName" class="relative block border-b bg-transparent focus-within:border-orange-500 border-neutral-700">
			<input type="text"
				id="CameraPresetName"
				placeholder="Name"
				bind:value={cameraPreset.name}
				on:blur={handleBlur}
				class="peer h-8 border-none bg-transparent p-0 placeholder-transparent focus:border-transparent focus:outline-none focus:ring-0 sm:text-sm text-white max-w-40"
			/>
			<!-- Show placeholder when no name is set -->
			{#if !cameraPreset.name}
				<span
					class="absolute start-0 top-0 -translate-y-1/2 text-xs transition-all peer-placeholder-shown:top-1/2 peer-placeholder-shown:text-sm peer-focus:top-0 peer-focus:text-xs text-neutral-700 overflow-visible"
				>
					Name
				</span>
			{/if}
		</label>
		</div>
		<!-- Button for toggling settings menu -->
		<div class="bg-neutral-800 p-2 rounded-full h-fit text-orange-500 hover:bg-orange-500 hover:text-white" role="button" tabindex="0" on:click|preventDefault={toggleSettings} on:keydown|preventDefault={(e) => e.key === 'Enter' && toggleSettings()}>
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
				<path stroke-linecap="round" stroke-linejoin="round" d="M6 13.5V3.75m0 9.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 3.75V16.5m12-3V3.75m0 9.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 3.75V16.5m-6-9V3.75m0 3.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 9.75V10.5" />
			  </svg>			  
		</div>
		<!-- Button for dragging -->
		<div class="bg-neutral-800 p-2 rounded-full text-orange-500 hover:bg-orange-500 hover:text-white handle-{cameraPreset.id}">
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" transform="rotate(45)" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
				<path stroke-linecap="round" stroke-linejoin="round" d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15" />
			  </svg>			  
		</div>
	</div>
	<!-- SettingsMenu -->
	<!-- TODO: rename showSettings to showSettings -->
	{#if showSettings}
	<form on:submit|preventDefault={handleSubmit} class="mt-2"> 
		<div class="flex">
			<!-- Input to update settings for camera_settings_position zoom -->
			<div class="w-full flex flex-col items-center text-xs">
				<label for="camera_setting_preset_position_x" class="p-2 text-neutral-500">X</label>
				<div class="flex items-center rounded border border-gray-800">
					<button type="button" class="size-6 leading-6 transition hover:text-orange-500 text-neutral-400"
						on:click|preventDefault={() => (cameraPreset.camera_settings.position.x = Math.max(0, cameraPreset.camera_settings.position.x - 1))}
					>
						&minus;
					</button>

					<input type="number" id="camera_setting_preset_position_x" class="focus:border-orange-500 focus:outline-none focus:ring-0 h-10 w-14 border-transparent text-center [-moz-appearance:_textfield] sm:text-sm bg-neutral-800 text-white [&::-webkit-inner-spin-button]:m-0 [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:m-0 [&::-webkit-outer-spin-button]:appearance-none"
						bind:value={cameraPreset.camera_settings.position.x}
					/>

					<button type="button" class="size-6 leading-6 transition hover:text-orange-500 text-neutral-400"
						on:click|preventDefault={() => (cameraPreset.camera_settings.position.x += 1)}
					>
						&plus;
					</button>
				</div>
			</div>
			<!-- Input to update settings for camera_settings_position zoom -->
			<div class="w-full flex flex-col items-center text-xs">
				<label for="camera_setting_preset_position_y" class="p-2 text-neutral-500">Y</label>
				<div class="flex items-center rounded border border-gray-800">
					<button type="button" class="size-6 leading-6 transition hover:text-orange-500 text-neutral-400"
						on:click|preventDefault={() => (cameraPreset.camera_settings.position.y = Math.max(0, cameraPreset.camera_settings.position.y - 1))}
					>
						&minus;
					</button>

					<input type="number" id="camera_setting_preset_position_y" class="focus:border-orange-500 focus:outline-none focus:ring-0 h-10 w-14 border-transparent text-center [-moz-appearance:_textfield] sm:text-sm bg-neutral-800 text-white [&::-webkit-inner-spin-button]:m-0 [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:m-0 [&::-webkit-outer-spin-button]:appearance-none"
						bind:value={cameraPreset.camera_settings.position.y}
					/>

					<button type="button" class="size-6 leading-6 transition hover:text-orange-500 text-neutral-400"
						on:click|preventDefault={() => (cameraPreset.camera_settings.position.y += 1)}
					>
						&plus;
					</button>
				</div>
			</div>
			<!-- Input to update settings for camera_settings_position zoom -->
			<div class="w-full flex flex-col items-center text-xs">
				<label for="camera_setting_preset_zoom" class="p-2 text-neutral-500">Zoom</label>
				<div class="flex items-center rounded border border-gray-800">
					<button type="button" class="size-6 leading-6 transition hover:text-orange-500 text-neutral-400"
						on:click|preventDefault={() => (cameraPreset.camera_settings.zoom = Math.max(0, cameraPreset.camera_settings.zoom - 1))}
					>
						&minus;
					</button>

					<input type="number" id="camera_setting_preset_zoom" class="focus:border-orange-500 focus:outline-none focus:ring-0 h-10 w-14 border-transparent text-center [-moz-appearance:_textfield] sm:text-sm bg-neutral-800 text-white [&::-webkit-inner-spin-button]:m-0 [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:m-0 [&::-webkit-outer-spin-button]:appearance-none"
						bind:value={cameraPreset.camera_settings.zoom}
					/>

					<button type="button" class="size-6 leading-6 transition hover:text-orange-500 text-neutral-400"
						on:click|preventDefault={() => (cameraPreset.camera_settings.zoom += 1)}
					>
						&plus;
					</button>
				</div>
			</div>
		</div>
		<div class="flex items-center mt-4 justify-center gap-4">
			<button type="submit" class="bg-green-700 text-white rounded p-3 hover:bg-green-600" aria-label="Update Button">
				<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
					<path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
				  </svg>				  
			</button>
			<button type="button" class="bg-red-700 text-white rounded p-3 hover:bg-red-600" aria-label="Delete Button" on:click|preventDefault={handeDelete}>
				<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
					<path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
				  </svg>				  
			</button>
		</div>
	</form>
	{/if}
</div>
<style>
</style>
