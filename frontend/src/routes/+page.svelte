<script lang="ts">
	import interact from 'interactjs';
	import { onMount } from 'svelte';
	import DraggableButton from '../components/DraggableButton.svelte';
	import type { Button } from '../types/button';
	import type {Camera} from '../types/camera';

	let buttons: Button[] = [];

	function addButton() {
		buttons = [...buttons, { id: buttons.length + 1, workspace_position: { x: 0, y: 0 }, camera_settings: {zoom: 0, position: {x: 0,y:0}}, name: '' }];
		console.log(buttons);
	}

	// TODO: Refactor, as this will be part of the Button component
	const drag_position: { x: Number; y: Number } = {
		x: 0,
		y: 0
	};

	// TODO: Refactor, as this will be part of the Button component
	export let data;
	type CameraPosition = {
		x: number;
		y: number;
		z: number;
	};
	// TODO: Refactor, as this will be part of the Button component
	let position: CameraPosition = data.position;

	async function handleSubmit(event: Event) {
		event.preventDefault();

		const response = await fetch('http://127.0.0.1:8000/api/camera/position', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ x: Number(position.x), y: Number(position.y), z: Number(position.z) })
		});

		if (response.ok) {
			reloadState();
			console.log('Updated Camera');
		} else {
			console.error('Failed to update camera position');
		}
	}

	async function reloadState() {
		const response = await fetch('http://127.0.0.1:8000/api/camera/position');
		position = await response.json();
		data.position = position;
	}

	// TODO: Refactor, as this will be part of the Button component
	onMount(() => {
		interact('.draggable')
			.draggable({
				listeners: {
					move(event) {
						drag_position.x += event.dx;
						drag_position.y += event.dy;
						event.target.style.transform =
							'translate(' + drag_position.x + 'px, ' + drag_position.y + 'px)';
					},
					start(event) {
						console.log(event.type, event.target);
					},
					end(event) {
						console.log(event.type, event.target);
					}
				}
			})
			.resizable({
				edges: { top: false, left: false, bottom: true, right: true },
				listeners: {
					move(event) {
						var target = event.target;
						var x = parseFloat(target.getAttribute('data-x')) || 0;
						var y = parseFloat(target.getAttribute('data-y')) || 0;

						// update the element's style
						target.style.width = event.rect.width + 'px';
						target.style.height = event.rect.height + 'px';

						target.setAttribute('data-x', x);
						target.setAttribute('data-y', y);
					},
					start(event) {
						console.log(event.type, event.target);
					},
					end(event) {
						console.log(event.type, event.target);
					}
				}
			});
	});
</script>

<div class="flex h-screen bg-gray-100">
	<!-- sidebar -->
	<div class="hidden md:flex flex-col w-64 bg-neutral-950">
		<div class="flex items-center justify-center h-16 gap-3">
			<img
				src="https://www.unibw.de/code/@@images/75d98095-a6fb-4004-b16f-cdbb1ef8c157.png"
				alt="FI Code Logo"
				class="h-11"
			/>
			<div class="flex flex-col">
				<h2 class="text-base/5 font-semibold text-orange-500">VISCA Protocol</h2>
				<h2 class="text-base/5 font-semibold text-orange-500">Camera Control</h2>
			</div>
		</div>
		<div class="flex flex-col flex-1 overflow-y-auto">
			<nav class="flex-1 px-2 py-4 bg-neutral-900">
				<button class="flex items-center px-4 py-2 text-gray-100 hover:bg-orange-500 w-full" on:click|preventDefault={addButton}>
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
					Add Button
				</button>
				<button class="flex items-center px-4 py-2 text-gray-100 hover:bg-orange-500 w-full">
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
					Settings
				</button>
			</nav>
		</div>
	</div>

	<!-- Main content -->
	<div class="flex flex-col flex-1 overflow-y-auto bg-neutral-800 pattern justify-center">
		{#each buttons as button (button.id)}
        <DraggableButton id={button.id} position={button.workspace_position} />
    	{/each}
		<div class="mx-auto draggable bg-neutral-950 rounded-lg">
			<div class="bg-neutral-900 rounded-t-lg p-3 relative">
				<p class="text-center text-sm text-neutral-300 font-light">Current state</p>
				<div>
					<div class="flex items-center justify-center space-x-10 mt-3">
						<div class="flex flex-col items-center">
							<span class="text-3xl text-orange-500 font-bold">{data.position.x}</span>
							<p class="text-sm text-neutral-600">X</p>
						</div>
						<div class="flex flex-col items-center">
							<span class="text-3xl text-orange-500 font-bold">{data.position.y}</span>
							<p class="text-sm text-neutral-600">Y</p>
						</div>
						<div class="flex flex-col items-center">
							<span class="text-3xl text-orange-500 font-bold">{data.position.z}</span>
							<p class="text-sm text-neutral-600">Z</p>
						</div>
					</div>
					<button
						on:click|preventDefault={reloadState}
						type="button"
						aria-label="Reload"
						class="absolute top-0 right-0 p-3 text-orange-500 hover:bg-orange-500 hover:text-white rounded-tr-lg"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							class="size-6"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"
							/>
						</svg>
					</button>
				</div>
			</div>
			<div class="rounded-b-lg py-6 px-4 lg:px-12">
				<p class="text-center text-sm text-neutral-300 font-light">Update state</p>
				<form on:submit|preventDefault={handleSubmit} class="mt-4">
					<div class="flex items-center text-xl font-bold">
						<span class="p-3 text-orange-500">X:</span>
						<input
							type="number"
							bind:value={position.x}
							class="appearance-none border pl-12 shadow-sm border-neutral-700 focus:border-orange-500 focus:shadow-md focus:placeholder-orange-700 transition rounded-md w-full py-3 text-neutral-400 leading-tight focus:outline-none focus:ring-orange-600 focus:shadow-outline bg-neutral-900"
						/>
					</div>
					<div class="flex items-center text-xl font-bold">
						<span class="p-3 text-orange-500">Y:</span>
						<input
							type="number"
							bind:value={position.y}
							class="appearance-none border pl-12 shadow-sm border-neutral-700 focus:border-orange-500 focus:shadow-md focus:placeholder-orange-700 transition rounded-md w-full py-3 text-neutral-400 leading-tight focus:outline-none focus:ring-orange-600 focus:shadow-outline bg-neutral-900"
						/>
					</div>
					<div class="flex items-center text-xl font-bold">
						<span class="p-3 text-orange-500">Z:</span>
						<input
							type="number"
							bind:value={position.z}
							class="appearance-none border pl-12 shadow-sm border-neutral-700 focus:border-orange-500 focus:shadow-md focus:placeholder-orange-700 transition rounded-md w-full py-3 text-neutral-400 leading-tight focus:outline-none focus:ring-orange-600 focus:shadow-outline bg-neutral-900"
						/>
					</div>
					<div class="flex items-center justify-center mt-4">
						<button type="submit" class="bg-orange-500 text-white rounded p-3 hover:bg-orange-600"
							>Update Position</button
						>
					</div>
				</form>
			</div>
		</div>
	</div>
</div>
