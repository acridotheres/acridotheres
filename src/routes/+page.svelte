<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    alert(await invoke('greet', { name: 'test' }));
  }
</script>

<div class="flex flex-col items-center justify-center">
  <h1 class="text-4xl font-bold">Hello, world!</h1>
  <button on:click={greet} class="mt-4 px-4 py-2 bg-blue-500 text-white rounded-md">Greet</button>
</div>