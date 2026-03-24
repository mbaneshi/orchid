<script lang="ts">
	import { api } from '$lib/api';
	import type { Session } from '$lib/types';

	let sessions = $state<Session[]>([]);
	let error = $state<string | null>(null);
	let selected = $state<Session | null>(null);

	async function load() {
		try {
			sessions = await api.listSessions();
		} catch {
			error = 'Could not load sessions. Is the API running?';
			sessions = [];
		}
	}

	$effect(() => {
		load();
	});

	function formatDate(iso: string): string {
		return new Date(iso).toLocaleString();
	}
</script>

<svelte:head>
	<title>Sessions | Orchid</title>
</svelte:head>

<div class="max-w-4xl">
	<h1 class="text-2xl font-bold text-gray-100 mb-6">Sessions</h1>

	{#if error}
		<div class="rounded-lg border border-yellow-900 bg-yellow-950 p-3 mb-4">
			<p class="text-xs text-yellow-400">{error}</p>
		</div>
	{/if}

	<div class="space-y-2">
		{#each sessions as session (session.id)}
			<button
				onclick={() => (selected = selected?.id === session.id ? null : session)}
				class="w-full text-left rounded-lg border bg-gray-900 p-4 transition-colors
					{selected?.id === session.id
					? 'border-emerald-700'
					: 'border-gray-800 hover:border-gray-700'}"
			>
				<div class="flex items-center justify-between">
					<h3 class="text-sm font-medium text-gray-200">{session.name}</h3>
					<span class="text-xs text-gray-500">{formatDate(session.updated_at)}</span>
				</div>
				<p class="text-xs text-gray-600 mt-1">{session.id}</p>
			</button>
			{#if selected?.id === session.id}
				<div class="rounded-lg border border-gray-800 bg-gray-950 p-4 ml-4">
					<dl class="grid grid-cols-2 gap-2 text-xs">
						<dt class="text-gray-500">ID</dt>
						<dd class="text-gray-300 font-mono">{session.id}</dd>
						<dt class="text-gray-500">Created</dt>
						<dd class="text-gray-300">{formatDate(session.created_at)}</dd>
						<dt class="text-gray-500">Updated</dt>
						<dd class="text-gray-300">{formatDate(session.updated_at)}</dd>
					</dl>
				</div>
			{/if}
		{/each}

		{#if sessions.length === 0 && !error}
			<p class="text-sm text-gray-500 text-center py-12">No sessions found.</p>
		{/if}
	</div>
</div>
