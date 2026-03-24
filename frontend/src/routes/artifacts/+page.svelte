<script lang="ts">
	import { api } from '$lib/api';
	import type { Artifact } from '$lib/types';
	import ArtifactList from '$lib/components/ArtifactList.svelte';

	let artifacts = $state<Artifact[]>([]);
	let error = $state<string | null>(null);
	let filterType = $state('');

	const artifactTypes = ['', 'changelog', 'blog_post', 'tweet', 'summary'] as const;

	async function load() {
		try {
			artifacts = await api.listArtifacts(filterType || undefined);
		} catch {
			error = 'Could not load artifacts. Is the API running?';
			artifacts = [];
		}
	}

	$effect(() => {
		load();
	});

	function handleFilter(type: string) {
		filterType = type;
		load();
	}
</script>

<svelte:head>
	<title>Artifacts | Orchid</title>
</svelte:head>

<div class="max-w-4xl">
	<h1 class="text-2xl font-bold text-gray-100 mb-6">Artifacts</h1>

	{#if error}
		<div class="rounded-lg border border-yellow-900 bg-yellow-950 p-3 mb-4">
			<p class="text-xs text-yellow-400">{error}</p>
		</div>
	{/if}

	<div class="flex gap-2 mb-6">
		{#each artifactTypes as type}
			<button
				onclick={() => handleFilter(type)}
				class="px-3 py-1 text-xs rounded-full transition-colors
					{filterType === type
					? 'bg-emerald-700 text-white'
					: 'bg-gray-800 text-gray-400 hover:bg-gray-700'}"
			>
				{type || 'All'}
			</button>
		{/each}
	</div>

	<ArtifactList {artifacts} />
</div>
