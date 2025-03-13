<script lang="ts">
    import clawVaultLogo from "$lib/images/claw-logo.png";
    import { GITHUB_LINK, Routes } from "$lib/routes";

    let year = new Date().getUTCFullYear();
</script>

<footer class="relative isolate overflow-hidden p-4 md:p-8 lg:p-10 bg-brand text-brand-dark">
    <div class="mx-auto max-w-screen-xl text-center">
        <a
            href={Routes.Home}
            class="flex justify-center items-center text-lg sm:text-2xl font-medium"
        >
            <img class="w-12 mr-4" src={clawVaultLogo} alt="Claw Vault" />
            Claw Vault
        </a>
        <p class="my-6 text-sm sm:text-base">
            Open-source platform for securely sharing private information.
        </p>
        <ul class="flex flex-wrap justify-center items-center mb-6 text-sm sm:text-base">
            <li>
                <a href={GITHUB_LINK} target="_blank" class="mr-4 hover:underline md:mr-6">Github</a
                >
            </li>
            <li>
                <a href={Routes.Privacy} class="mr-4 hover:underline md:mr-6">Privacy</a>
            </li>
        </ul>
        <span class="text-xs sm:text-sm sm:text-center">
            © {year} <a href={Routes.Home} class="hover:underline">Claw Vault ™</a>. All Rights
            Reserved.
        </span>
    </div>
</footer>
