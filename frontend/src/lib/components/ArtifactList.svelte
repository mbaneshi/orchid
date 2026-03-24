<script lang="ts">
	import type { Artifact } from '$lib/types';

	interface Props {
		artifacts: Artifact[];
	}

	let { artifacts }: Props = $props();

	let selected = $state<Artifact | null>(null);
</script>

<div class="space-y-2">
	{#each artifacts as artifact (artifact.id)}
		<button
			onclick={() => (selected = selected?.id === artifact.id ? null : artifact)}
			class="w-full text-left rounded-lg border border-gray-800 bg-gray-900 p-3 hover:border-teal-800 transition-colors"
		>
			<div class="flex items-center justify-between">
				<span class="text-sm font-medium text-gray-200">{artifact.artifact_type}</span>
				<span class="text-xs text-gray-500">{new Date(artifact.created_at).toLocaleDateString()}</span>
			</div>
			{#if artifact.metadata}
				<p class="text-xs text-gray-500 mt-1">{artifact.metadata}</p>
			{/if}
		</button>
		{#if selected?.id === artifact.id}
			<div class="rounded-lg border border-teal-900 bg-gray-950 p-4 ml-4">
				<pre class="text-xs text-gray-300 whitespace-pre-wrap overflow-auto max-h-64">{artifact.content}</pre>
			</div>
		{/if}
	{/each}

	{#if artifacts.length === 0}
		<p class="text-sm text-gray-500 text-center py-8">No artifacts yet.</p>
	{/if}
</div>
