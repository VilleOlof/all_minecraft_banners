<script lang="ts">
    import { PUBLIC_API } from "$env/static/public";

    let { seed }: { seed: bigint } = $props();
</script>

<a class="cursor-pointer" href="/b/{seed}">
    <img
        src="{PUBLIC_API}/banner/{seed}"
        alt=""
        class="pixelated w-40 drop-shadow-[4px_4px_5px_rgba(0,0,0,0.5)] transition-all hover:scale-95"
    />
</a>
