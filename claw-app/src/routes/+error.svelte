<script lang="ts">
    import { page } from "$app/state";
    import BackPattern from "$lib/components/BackPattern.svelte";
    import awkClaw from "$lib/images/awk-claw.png";
</script>

<section class="relative isolate place-items-center bg-white dark:bg-brand-dark">
    <BackPattern />

    <div
        class="justify-center text-center flex flex-col mx-auto max-w-7xl px-6 py-12 sm:py-12 lg:flex lg:items-center lg:gap-x-10 lg:px-8 lg:py-24"
    >
        <div class="lg:flex-shrink-0 lg:flex-grow justify-center flex">
            <img class="w-72" src={awkClaw} alt="" />
        </div>
        <p class="text-3xl font-semibold text-brand">{page.status}</p>
        <p class="mt-4 text-3xl font-bold tracking-tight text-brand sm:text-5xl">
            {page.error?.message || "Page not found"}
        </p>
        <div class="mt-10 flex items-center justify-center gap-x-6">
            <a
                href="/"
                class="rounded-md bg-brand px-3.5 py-2.5 text-sm font-semibold text-brand-dark shadow-sm hover:bg-brand/70 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2"
            >
                Go back home
            </a>
        </div>
    </div>
</section>
