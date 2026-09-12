<script lang="ts">
	import type { Snippet } from "svelte"

	import { page } from "$app/state"
	import {
		BookOpen,
		BookText,
		GraduationCap,
		Users,
		GitFork,
		Settings,
		Menu,
	} from "@lucide/svelte"
	import { CONFIG_TABS } from "$shared/config/tabs"

	let { children }: { children: Snippet } = $props()

	const navItems = [
		{ href: "/admin/config", label: "Configuración", icon: Settings },
		{ href: "/admin/research-lines", label: "Líneas Investigación", icon: GitFork },
		{ href: "/academics", label: "Académicos", icon: GraduationCap },
		{ href: "/works", label: "Publicaciones", icon: BookOpen },
		{ href: "/admin/users", label: "Usuarios", icon: Users },
		{ href: "/admin/manual", label: "Manual", icon: BookText },
	] as const

	const currentPagePath = $derived(page.url.pathname)
	const isConfig = $derived(currentPagePath === "/admin/config")
	const isManual = $derived(currentPagePath.startsWith("/admin/manual"))
	const isNavItemActive = (href: string) =>
		currentPagePath === href || currentPagePath.startsWith(`${href}/`)
	const configTab = $derived(
		(CONFIG_TABS.find((t) => t.id === page.url.searchParams.get("tab")) ?? CONFIG_TABS[0]).id,
	)

	let collapsed = $state(false)
</script>

<div class="mx-auto flex h-full max-w-[1600px] flex-col px-4 py-8 sm:px-6 lg:px-8">
	<div class="flex min-h-0 flex-1 gap-8">
		{#if !isManual}
			<aside
				class="relative hidden shrink-0 self-start rounded-xl border border-corp-gray/20 bg-white transition-all duration-200 lg:block {collapsed
					? 'w-16'
					: 'w-72'}"
			>
				<div
					class="flex items-center p-4 {collapsed
						? 'justify-center px-3'
						: 'justify-between'}"
				>
					{#if !collapsed}
						<div>
							<h1 class="text-lg font-semibold text-[#1A1A1A]">Administración</h1>
							<p class="mt-1 text-sm text-corp-gray">Gestión del sistema</p>
						</div>
					{/if}
					<button
						type="button"
						class="flex items-center justify-center rounded-lg text-corp-gray transition-colors hover:text-[#1A1A1A]"
						onclick={() => (collapsed = !collapsed)}
						title={collapsed ? "Expandir menú" : "Colapsar menú"}
					>
						<Menu class="size-5" />
					</button>
				</div>

				<nav class="mt-2 space-y-1 px-2 pb-4">
					{#each navItems as item (item.href)}
						<a
							href={item.href}
							class="flex items-center rounded-lg px-3 py-2 text-sm font-medium transition-colors {isNavItemActive(
								item.href,
							)
								? 'bg-corp-gray/10 text-[#1A1A1A]'
								: 'text-corp-gray hover:bg-corp-gray/5 hover:text-[#1A1A1A]'} {collapsed
								? 'justify-center px-0'
								: 'gap-2.5'}"
							title={collapsed ? item.label : undefined}
						>
							<item.icon class="size-4 shrink-0" />
							{#if !collapsed}
								{item.label}
							{/if}
						</a>

						{#if !collapsed && isConfig && item.href === "/admin/config"}
							<div
								class="mb-1 ml-3 mt-1 space-y-0.5 border-l border-corp-gray/20 pl-2"
							>
								{#each CONFIG_TABS as tab (tab.id)}
									<a
										href={tab.href}
										class="flex items-center rounded-md px-2.5 py-1.5 text-[13px] font-medium transition-colors {configTab ===
										tab.id
											? 'bg-corp-blue/10 text-corp-blue'
											: 'text-corp-gray hover:bg-corp-gray/5 hover:text-corp-ink'}"
									>
										{tab.label}
									</a>
								{/each}
							</div>
						{/if}
					{/each}
				</nav>
			</aside>
		{/if}

		<div class="min-w-0 flex-1 overflow-y-auto overscroll-contain">
			{@render children()}
		</div>
	</div>
</div>
