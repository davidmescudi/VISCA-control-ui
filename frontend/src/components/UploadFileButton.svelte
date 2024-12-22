<script>
    async function uploadJSONfile() {
        const fileInput = document.createElement('input');
        fileInput.type = 'file';
        fileInput.accept = '.viscaui';
        fileInput.onchange = async (event) => {
            if (!(event.target instanceof HTMLInputElement) || !event.target.files) {
                return;
            }
            const file = event.target.files[0];
            const reader = new FileReader();
            reader.onload = async (event) => {
                try {
                    const response = await fetch('http://127.0.0.1:8000/backup/upload', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                        },
                        body: reader.result,
                    });
                    if (!response.ok) {
                        throw new Error('Network response was not ok');
                    }
                    location.reload();
                } catch (error) {
                    console.error('Error parsing JSON:', error);
                }
            };
            reader.readAsText(file);
    };
    fileInput.click();
}
</script>

<button class="flex items-center px-4 py-2 text-gray-100 hover:bg-orange-500 w-full justify-center" on:click={uploadJSONfile} aria-label="Upload state with a file">
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6 mr-2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5m-13.5-9L12 3m0 0 4.5 4.5M12 3v13.5" />
    </svg>      
</button>