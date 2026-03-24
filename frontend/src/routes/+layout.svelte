<script lang="ts">
	import '../app.css';
	import Nav from '$lib/components/Nav.svelte';

	interface Props {
		children: import('svelte').Snippet;
	}

	let { children }: Props = $props();
</script>

<div class="flex min-h-screen">
	<Nav />
	<main class="flex-1 ml-56 p-6">
		{@render children()}
	</main>
</div>
