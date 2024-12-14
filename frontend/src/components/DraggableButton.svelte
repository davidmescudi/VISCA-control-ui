<script lang="ts">
	import { onMount } from 'svelte';
	import interact from 'interactjs';

	export let id: number;
	export let position = { x: 0, y: 0 };

	let showForm = false;

	function toggleForm() {
		showForm = !showForm;
	}

	function updateSettings(event: Event) {
		// TODO: Handle settings update logic here
		console.log(`Updated settings for button ${id}:`, { name });
		showForm = false;
	}

	onMount(() => {
		interact(`.draggable-${id}`).draggable({
			allowFrom: `.handle-${id}`,
			listeners: {
				move(event) {
					position.x += event.dx;
					position.y += event.dy;
					event.target.style.transform = `translate(${position.x}px, ${position.y}px)`;
				},
				end(event) {
					// Synchronize position with backend
					/*
                        fetch('http://127.0.0.1:8000/api/button/position', {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify(position)
                        })
                            .then(response => response.json())
                            .then(data => console.log('Position updated:', data))
                            .catch(error => console.error('Error updating position:', error));
                        */
				}
			}
		});
	});
</script>

<div
	class="rounded-lg border border-neutral-700 bg-neutral-900 p-6 draggable draggable-{id} w-max absolute"
	style="transform: translate({position.x}px, {position.y}px);"
>
	<div class="flex items-center justify-between">
		<div class="handle-{id}">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="size-5 text-orange-500"
				transform="rotate(45)"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15"
				/>
			</svg>
		</div>

		<label
			for="UserEmail"
			class="relative block overflow-hidden border-b border-gray-200 bg-transparent pt-3 focus-within:border-orange-500 dark:border-gray-700 ml-3"
		>
			<input
				type="email"
				id="UserEmail"
				placeholder="Email"
				class="peer h-8 w-full border-none bg-transparent p-0 placeholder-transparent focus:border-transparent focus:outline-none focus:ring-0 sm:text-sm dark:text-white"
			/>

			<span
				class="absolute start-0 top-2 -translate-y-1/2 text-xs text-gray-700 transition-all peer-placeholder-shown:top-1/2 peer-placeholder-shown:text-sm peer-focus:top-2 peer-focus:text-xs dark:text-gray-200"
			>
				Name
			</span>
		</label>
	</div>

	<div class="mt-1 flex gap-2 text-gray-400 text-xs items-center">
	<svg
		xmlns="http://www.w3.org/2000/svg"
		fill="none"
		viewBox="0 0 24 24"
		stroke-width="1.5"
		stroke="currentColor"
		class="size-5 text-orange-500"
	  on:click|preventDefault={toggleForm}
	  on:keydown|preventDefault={(e) => e.key === 'Enter' && toggleForm()}
	  tabindex="0"
	  role="button"
	>
		<path
			stroke-linecap="round"
			stroke-linejoin="round"
			d="M6 13.5V3.75m0 9.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 3.75V16.5m12-3V3.75m0 9.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 3.75V16.5m-6-9V3.75m0 3.75a1.5 1.5 0 0 1 0 3m0-3a1.5 1.5 0 0 0 0 3m0 9.75V10.5"
		/>
	</svg>
		<p class="flex gap-2 ml-auto">
			<span class="font-medium">Button-Id:</span>

			<span class="text-gray-500 dark:text-gray-400">{id}</span>
		</p>
	</div>
  {#if showForm}
    <form on:submit|preventDefault={updateSettings} class="mt-4 text-xs">
      <!-- Input to update settings for position x -->
      <div class="w-fit flex items-center">
        <label for="workspace_position_x" class="p-3 text-orange-500">X: </label>
        <div class="flex items-center rounded border border-gray-200 dark:border-gray-800">
          <button
            type="button"
            class="size-10 leading-10 text-gray-600 transition hover:opacity-75 dark:text-gray-300"
          >
            &minus;
          </button>
      
          <input
            type="number"
            id="workspace_position_x"
            bind:value={position.x}
            class="h-10 w-16 border-transparent text-center [-moz-appearance:_textfield] sm:text-sm dark:bg-gray-900 dark:text-white [&::-webkit-inner-spin-button]:m-0 [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:m-0 [&::-webkit-outer-spin-button]:appearance-none"
          />
      
          <button
            type="button"
            class="size-10 leading-10 text-gray-600 transition hover:opacity-75 dark:text-gray-300"
          >
            &plus;
          </button>
        </div>
      </div>

      <!-- Input to update settings for position y -->
      <div class="w-fit flex items-center">
        <label for="workspace_position_y" class="p-3 text-orange-500">Y: </label>
        <div class="flex items-center rounded border border-gray-200 dark:border-gray-800">
          <button
            type="button"
            class="size-10 leading-10 text-gray-600 transition hover:opacity-75 dark:text-gray-300"
          >
            &minus;
          </button>
      
          <input
            type="number"
            id="workspace_position_y"
            bind:value={position.y}
            class="h-10 w-16 border-transparent text-center [-moz-appearance:_textfield] sm:text-sm dark:bg-gray-900 dark:text-white [&::-webkit-inner-spin-button]:m-0 [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:m-0 [&::-webkit-outer-spin-button]:appearance-none"
          />
      
          <button
            type="button"
            class="size-10 leading-10 text-gray-600 transition hover:opacity-75 dark:text-gray-300"
          >
            &plus;
          </button>
        </div>
      </div>

      <div class="flex items-center justify-center mt-4">
        <button type="submit" class="bg-orange-500 text-white rounded p-3 hover:bg-orange-600"
          >Update Position</button
        >
      </div>
    </form>
  {/if}
</div>

<style>
</style>
