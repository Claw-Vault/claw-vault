<script lang="ts">
    import BackPattern from "./BackPattern.svelte";
    import DecryptBlock from "./DecryptBlock.svelte";
    import EncryptBlock from "./EncryptBlock.svelte";
</script>

<section class="relative isolate overflow-hidden bg-white">
    <BackPattern />

    <div
        class="mx-auto max-w-7xl pb-24 pt-4 sm:pb-32 lg:grid lg:grid-cols-2 lg:gap-x-8 lg:px-8 lg:py-40"
    >
        <EncryptBlock />

        <DecryptBlock />
    </div>
</section>
