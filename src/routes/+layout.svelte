<script lang="ts">
  import '../app.scss';
  const { children } = $props();
</script>

<main class="bg-black h-lvh text-white w-lvw">
  {@render children()}
</main>
