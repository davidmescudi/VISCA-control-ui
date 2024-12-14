<script lang="ts">
    import { onMount } from 'svelte';
    import interact from 'interactjs';

    export let id: number;
    export let position = { x: 0, y: 0 };

    onMount(() => {
        interact('.draggable')
            .draggable({
                listeners: {
                    move(event) {
                        position.x += event.dx;
                        position.y += event.dy;
                        event.target.style.transform = `translate(${position.x}px, ${position.y}px)`;
                    },
                    end(event) {
                        // Synchronize position with backend
                        fetch('http://127.0.0.1:8000/api/button/position', {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify(position)
                        })
                            .then(response => response.json())
                            .then(data => console.log('Position updated:', data))
                            .catch(error => console.error('Error updating position:', error));
                    }
                }
            });
    });
</script>

<style>
    .draggable {
        width: 100px;
        height: 100px;
        background-color: lightblue;
        border: 1px solid #000;
        cursor: move;
        position: absolute;
    }
</style>

<div class="draggable" style="transform: translate({position.x}px, {position.y}px);">
    Button {id}
</div>