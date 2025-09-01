<script lang="ts">
	import "../app.css";
	import dice from "$lib/assets/dice.webp";
	import type { LayoutProps } from "./$types";
	import { format_num_string, PER_PAGE, random_bigint } from "$lib";
	import { PUBLIC_API } from "$env/static/public";
	import { goto } from "$app/navigation";

	let { data, children }: LayoutProps = $props();

	// yoinked from +page
	let navigation_disabled = $state(false);
	function nav() {
		navigation_disabled = true;
		setTimeout(() => {
			navigation_disabled = false;
		}, 200);
	}
</script>

<svelte:head>
	<link rel="icon" href={dice} />
	<title>AllMinecraftBanners.com</title>

	<meta property="og:title" content="All Minecraft Banners" />
	<meta
		property="og:image"
		content="{PUBLIC_API}/banner/{data.header_seed}"
	/>
	<meta
		property="og:description"
		content="View all {data.metadata
			.combinations} different banners from Minecraft."
	/>
</svelte:head>

<main
	class="w-dvw h-dvh flex flex-col bg-neutral-900 text-neutral-200 font-monocraft"
>
	<header
		class="flex justify-between z-10 bg-neutral-950 py-2 px-4 w-full items-end gap-5 drop-shadow-[0px_6px_5px_rgba(0,0,0,0.75)]"
	>
		<div class="flex gap-5">
			<a href="/b/{data.header_seed}">
				<img
					src="{PUBLIC_API}/banner/{data.header_seed}"
					alt=""
					class="w-[1.1rem] object-contain bg-neutral-900"
				/></a
			>

			<button
				disabled={navigation_disabled}
				onclick={async () => {
					const seed = random_bigint(
						BigInt(data.metadata.combinations) / PER_PAGE,
					);
					await goto(`/?page=${seed}`);
					nav();
				}}
				class="text-3xl flex gap-3 items-end hover:text-neutral-400 transition-colors cursor-pointer"
			>
				<span>All</span>
				<span>Minecraft</span>
				<span>Banners</span>
				<span class="text-neutral-400 text-xl">.com</span>
			</button>
		</div>

		<!-- <div class="w-1 h-full bg-neutral-900 rounded-sm"></div> -->

		<p class="text-2xl">
			<span class="text-lime-400"
				>{format_num_string(data.metadata.combinations)}</span
			> unique banners!
		</p>
	</header>

	<div class="w-full px-4 pb-4 pt-4 min-h-0 overflow-y-auto flex-1">
		{@render children?.()}
	</div>
</main>
