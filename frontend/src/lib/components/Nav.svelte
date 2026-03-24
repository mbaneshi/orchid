<script lang="ts">
	import { page } from '$app/state';

	const links = [
		{ href: '/dashboard', label: 'Dashboard', icon: 'grid' },
		{ href: '/agents', label: 'Agents', icon: 'cpu' },
		{ href: '/flows', label: 'Flows', icon: 'git-branch' },
		{ href: '/artifacts', label: 'Artifacts', icon: 'file-text' },
		{ href: '/sessions', label: 'Sessions', icon: 'clock' }
	] as const;

	function isActive(href: string): boolean {
		return page.url.pathname.startsWith(href);
	}
</script>

<nav class="fixed top-0 left-0 h-screen w-56 bg-gray-900 border-r border-gray-800 flex flex-col">
	<div class="p-4 border-b border-gray-800">
		<a href="/dashboard" class="flex items-center gap-2">
			<span class="text-xl font-bold text-emerald-400">Orchid</span>
		</a>
		<p class="text-xs text-gray-500 mt-1">Agentic Dev OS</p>
	</div>

	<ul class="flex-1 py-4 space-y-1">
		{#each links as { href, label }}
			<li>
				<a
					{href}
					class="flex items-center gap-3 px-4 py-2 text-sm transition-colors
						{isActive(href)
						? 'bg-emerald-900/30 text-emerald-400 border-r-2 border-emerald-400'
						: 'text-gray-400 hover:text-gray-200 hover:bg-gray-800/50'}"
				>
					{label}
				</a>
			</li>
		{/each}
	</ul>

	<div class="p-4 border-t border-gray-800 text-xs text-gray-600">
		v0.1.0
	</div>
</nav>
