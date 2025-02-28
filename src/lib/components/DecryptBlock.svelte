<script lang="ts">
    import { enhance } from "$app/forms";
    import Button from "./ui/button/button.svelte";

    let id = $state("");
</script>

<div class="mt-20 sm:mt-24 md:mx-auto md:max-w-2xl lg:mx-0 lg:mt-0 lg:w-screen">
    <div class="shadow-lg md:rounded-3xl">
        <div
            class="bg-brand [clip-path:inset(0)] md:[clip-path:inset(0_round_theme(borderRadius.3xl))]"
        >
            <div class="mx-auto max-w-2xl lg:mx-0 p-16 lg:flex-auto text-brand-dark">
                <form
                    method="POST"
                    use:enhance={({ formData }) => {
                        formData.append("dec_id", id);
                        return async ({ update, result }) => {
                            update();
                        };
                    }}
                ></form>

                <div class="flex flex-col gap-8">
                    <h1 class="max-w-lg text-4xl font-bold tracking-tight sm:text-6xl mt-8">
                        Get Data
                    </h1>
                    <div class="sm:col-span-4">
                        <p class="block text-sm font-medium leading-6">Data ID</p>
                        <div class="mt-2 flex">
                            <input
                                type="text"
                                autocomplete="off"
                                class="flex-grow max-w-96 rounded-md px-2 border border-brand-dark/20 bg-brand-dark/5 py-1.5 text-gray-900 shadow-sm sm:text-sm sm:leading-6"
                                placeholder="ID"
                                value={id}
                                oninput={(e) => {
                                    let tokens = e.target.value.split(".");
                                    id = tokens.length > 1 ? (tokens[0] as string) : e.target.value;
                                }}
                                required
                            />
                        </div>
                    </div>
                    <Button
                        class="w-fit bg-brand-dark text-sm font-semibold text-brand shadow-sm hover:bg-gray-700 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2"
                    >
                        Get Data
                    </Button>
                </div>
            </div>
        </div>
    </div>
</div>
