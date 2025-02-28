<script lang="ts">
    import clawVault from "$lib/images/claw-vault.png";
    import BackPattern from "$lib/components/BackPattern.svelte";
    import { GITHUB_LINK } from "$lib/routes";
    import ClawHome from "$lib/components/ClawHome.svelte";
</script>

<section class="relative isolate pt-14 bg-white">
    <BackPattern />

    <div
        class="mx-auto max-w-7xl px-6 py-12 sm:py-12 lg:flex lg:items-center lg:gap-x-10 lg:px-8 lg:py-24"
    >
        <div class="lg:flex-shrink-0 lg:flex-grow justify-center flex">
            <img class="rounded-3xl shadow-xl" src={clawVault} alt="" />
        </div>
        <div class="mx-auto max-w-2xl lg:mx-0 lg:flex-auto justify-center">
            <h1
                class="mt-10 max-w-lg text-4xl font-bold tracking-tight text-brand-dark sm:text-6xl"
            >
                A better way to share private data
            </h1>
            <p class="mt-6 text-lg leading-8 font-semibold text-brand-dark/70">
                Claw Vault encrypts your data, providing a unique ID and key for secure
                transmission. Your recipient can easily access the encrypted content using the ID
                and key, ensuring confidentiality. Data is automatically deleted upon access or
                expiration, prioritizing your privacy.
            </p>
            <div class="mt-10 flex items-center gap-x-6">
                <a
                    href="#claw"
                    class="rounded-md bg-brand px-3.5 py-2.5 text-sm font-semibold text-brand-dark shadow-sm hover:bg-brand/70"
                >
                    Get started
                </a>
                <a
                    href={GITHUB_LINK}
                    target="_blank"
                    class="text-sm font-semibold leading-6 text-brand-dark"
                >
                    View on GitHub
                    <span aria-hidden="true">→</span>
                </a>
            </div>
        </div>
    </div>
</section>

<ClawHome />
