<script lang="ts">
    import Navbar from "$lib/components/Navbar.svelte";
    import { ModeWatcher } from "mode-watcher";
    import "../app.css";
    import Footer from "$lib/components/Footer.svelte";

    let { children } = $props();
</script>

<svelte:head>
    <title>Claw Vault</title>
    <meta
        name="description"
        content="Claw Vault is a platform to share sensitive information with desired recipients. Claw Vault encrypts your data, providing a unique ID and key for secure transmission."
    />
    <link rel="icon" type="image/x-icon" href="/favicon.png" />
    <link rel="icon" href="/favicon.png" />

    <meta name="og:title" content="Claw Vault" />
    <meta
        name="og:description"
        content="Claw Vault is a platform to share sensitive information with desired recipients. Claw Vault encrypts your data, providing a unique ID and key for secure transmission."
    />
    <meta
        name="og:image"
        content="https://claw-vault.up.railway.app/assets/claw-vault.png"
    />
    <meta name="og:url" content="https://claw-vault.up.railway.app/" />
    <meta name="og:site_name" content="Claw Vault" />
    <meta name="og:type" content="website" />

    <meta name="twitter:title" content="Claw Vault" />
    <meta
        name="twitter:description"
        content="Claw Vault is a platform to share sensitive information with desired recipients. Claw Vault encrypts your data, providing a unique ID and key for secure transmission."
    />
    <meta
        name="twitter:image"
        content="https://claw-vault.up.railway.app/assets/claw-vault.png"
    />
    <meta name="twitter:card" content="summary_large_image" />

    <meta name="robots" content="index,follow" />
</svelte:head>

<ModeWatcher />

<Navbar />

<main>
    {@render children()}
</main>

<Footer />
