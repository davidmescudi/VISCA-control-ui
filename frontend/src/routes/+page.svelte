<script lang="ts">
	export let data;
	type CameraPosition = {
		x: number;
		y: number;
		z: number;
	};
	
	let position: CameraPosition = data.position;
	

	async function handleSubmit(event: Event) {
		event.preventDefault();

		const response = await fetch('http://127.0.0.1:8000/api/camera/position', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ x: Number(position.x), y: Number(position.y), z: Number(position.z)})
		});

		if (response.ok) {
			console.log('Updated Camera')
		} else {
			console.error('Failed to update camera position');
		}
	}
</script>

<div class="flex h-screen bg-gray-100">
	<!-- sidebar -->
	<div class="hidden md:flex flex-col w-64 bg-neutral-950">
		<div class="flex items-center justify-center h-16 gap-3">
			<div class="flex flex-col">
				<h2 class="text-base/5 font-semibold text-orange-500">VISCA Protocol</h2>
				<h2 class="text-base/5 font-semibold text-orange-500">Camera Control</h2>
			</div>
		</div>
		<div class="flex flex-col flex-1 overflow-y-auto">
			<nav class="flex-1 px-2 py-4 bg-neutral-900">
				<a href="#" class="flex items-center px-4 py-2 text-gray-100 hover:bg-orange-500">
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
					Add Camera Preset
				</a>
				<a href="#" class="flex items-center px-4 py-2 mt-2 text-gray-100 hover:bg-orange-500">
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
				</a>
			</nav>
		</div>
	</div>

	<!-- Main content -->
	<div class="flex flex-col flex-1 overflow-y-auto bg-neutral-800 pattern justify-center">
		<form on:submit|preventDefault={handleSubmit} class="bg-neutral-50 flex flex-col mx-auto p-5 rounded gap-5">
			<label>
				X:
				<input type="number" bind:value={position.x} />
			</label>
			<label>
				Y:
				<input type="number" bind:value={position.y} />
			</label>
			<label>
				Z:
				<input type="number" bind:value={position.z} />
			</label>
			<button type="submit" class="bg-orange-500 text-white rounded p-5">Update Position</button>
		</form>
	</div>
</div>
