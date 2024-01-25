pub fn add_404(tera: &mut tera::Tera) {
    let source = r#"
<!DOCTYPE html>
<html lang="en">

{% include "head.html" %}

<body class="h-full">
    {% include "header.html" %}

    <main class="relative isolate place-items-center bg-gray-900">
        <svg class="absolute inset-0 -z-10 h-full w-full stroke-accent" aria-hidden="true">
            <svg x="50%" y="-1" class="overflow-visible fill-accent-30">
                <path
                    d="M-100.5 0h201v201h-201Z M699.5 0h201v201h-201Z M499.5 400h201v201h-201Z M-300.5 600h201v201h-201Z"
                    stroke-width="0" />
            </svg>
            <rect width="100%" height="100%" stroke-width="0" fill="url(#83fd4e5a-9d52-42fc-97b6-718e5d7ee527)" />
        </svg>
        <div
            class="justify-center text-center flex flex-col mx-auto max-w-7xl px-6 py-12 sm:py-12 lg:flex lg:items-center lg:gap-x-10 lg:px-8 lg:py-24">
            <div class="lg:flex-shrink-0 lg:flex-grow justify-center flex">
                <img class="w-72" src="/assets/awk-claw.png" alt="">
            </div>
            <p class="text-3xl font-semibold text-accent">404</p>
            <h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-100 sm:text-5xl">
                Data not found
            </h1>
            <p class="mt-6 text-base leading-7 text-gray-400">
                Sorry, there is no claw with for this data ID.
            </p>
            <div class="mt-10 flex items-center justify-center gap-x-6">
                <a href="/"
                    class="rounded-md bg-accent px-3.5 py-2.5 text-sm font-semibold text-gray-900 shadow-sm hover:bg-[#9ad4d8]/90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2">
                    Go back home
                </a>
            </div>
        </div>
    </main>

    {% include "footer.html" %}
</body>

</html>"#;

    tera.add_raw_template("404.html", source)
        .expect("Failed to add 404 template")
}

pub fn add_index(tera: &mut tera::Tera) {
    let source = r#"
<!DOCTYPE html>
<html lang="en">

{% include "head.html" %}

<body>
    {% include "header.html" %}

    <section class="relative isolate pt-14 bg-gray-900">
        <svg class="absolute inset-0 -z-10 h-full w-full stroke-accent" aria-hidden="true">
            <svg x="50%" y="-1" class="overflow-visible fill-accent-30">
                <path
                    d="M-100.5 0h201v201h-201Z M699.5 0h201v201h-201Z M499.5 400h201v201h-201Z M-300.5 600h201v201h-201Z"
                    stroke-width="0" />
            </svg>
            <rect width="100%" height="100%" stroke-width="0" fill="url(#83fd4e5a-9d52-42fc-97b6-718e5d7ee527)" />
        </svg>
        <div class="mx-auto max-w-7xl px-6 py-12 sm:py-12 lg:flex lg:items-center lg:gap-x-10 lg:px-8 lg:py-24">
            <div class="lg:flex-shrink-0 lg:flex-grow justify-center flex">
                <img class="rounded-3xl shadow-xl" src="/assets/claw-vault.png" alt="" />
            </div>
            <div class="mx-auto max-w-2xl lg:mx-0 lg:flex-auto justify-center">
                <h1 class="mt-10 max-w-lg text-4xl font-bold tracking-tight text-gray-100 sm:text-6xl">
                    A better way to share private data
                </h1>
                <p class="mt-6 text-lg leading-8 font-semibold text-gray-400">
                    Claw Vault encrypts your data, providing a unique ID and key for secure transmission. Your
                    recipient can easily access the encrypted content using the ID and key, ensuring confidentiality.
                    Data is automatically deleted upon access or expiration, prioritizing your privacy.
                </p>
                <div class="mt-10 flex items-center gap-x-6">
                    <a href="\#claw"
                        class="rounded-md bg-accent px-3.5 py-2.5 text-sm font-semibold text-gray-900 shadow-sm hover:bg-accent focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                        Get started
                    </a>
                    <a href="https://github.com/Claw-Vault/claw-vault" target="_blank"
                        class="text-sm font-semibold leading-6 text-accent">View on GitHub
                        <span aria-hidden="true">→</span>
                    </a>
                </div>
            </div>
        </div>
    </section>

    <section id="claw" class="relative isolate overflow-hidden bg-gray-900">
        <svg class="absolute inset-0 -z-10 h-full w-full stroke-accent" aria-hidden="true">
            <svg x="50%" y="-1" class="overflow-visible fill-accent-30">
                <path
                    d="M-100.5 0h201v201h-201Z M699.5 0h201v201h-201Z M499.5 400h201v201h-201Z M-300.5 600h201v201h-201Z"
                    stroke-width="0" />
            </svg>
            <rect width="100%" height="100%" stroke-width="0" fill="url(#83fd4e5a-9d52-42fc-97b6-718e5d7ee527)" />
        </svg>
        <div class="mx-auto max-w-7xl pb-24 pt-4 sm:pb-32 lg:grid lg:grid-cols-2 lg:gap-x-8 lg:px-8 lg:py-40">
            <div class="px-6 lg:px-0 lg:pt-4">
                <div class="mx-auto max-w-2xl">
                    <div class="max-w-lg">
                        <form method="dialog" action="\#" onsubmit="encryptData()">
                            <h1 class="mt-10 max-w-lg text-4xl font-bold tracking-tight text-gray-100 sm:text-6xl">
                                Encrypt your private data
                            </h1>
                            <div class="mt-10 col-span-full">
                                <label for="enc-data" class="block text-sm font-medium leading-6 text-[#9AD4D8]">
                                    Data
                                </label>
                                <div class="mt-2 bg-gray-900">
                                    <textarea id="enc-data" name="data" rows="3"
                                        class="block w-full px-2 rounded-md min-h-32 border-0 bg-[#9AD4D8]/10 py-1.5 text-white shadow-sm ring-1 ring-inset ring-[#9AD4D8]/10 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6"
                                        required></textarea>
                                </div>
                                <p class="mt-3 text-sm leading-6 text-gray-400">Enter your private text data</p>
                            </div>
                            <div class="space-y-12">
                                <div class="border-b border-white/10 pb-12">
                                    <div class="mt-2 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
                                    </div>
                                </div>

                                <div class="sm:col-span-6">
                                    <label class="text-lg font-semibold text-accent">Expiration</label>
                                    <p class="text-sm text-gray-300">
                                        How long do you want this data to be accessible to the recipient ?
                                    </p>
                                    <fieldset class="mt-4">
                                        <div class="space-y-4 sm:flex sm:items-center sm:space-x-10 sm:space-y-0">
                                            <div class="flex items-center">
                                                <input id="1-min" name="expiration" type="radio" checked
                                                    class="h-4 w-4 bg-gray-900 border-gray-300 text-[#9AD4D8] focus:ring-[#9AD4D8]">
                                                <label for="1-min"
                                                    class="ml-3 block text-sm font-medium leading-6 text-gray-100">
                                                    1 minute
                                                </label>
                                            </div>
                                            <div class="flex items-center">
                                                <input id="15-min" name="expiration" type="radio"
                                                    class="h-4 w-4 bg-gray-900 border-gray-300 text-[#9AD4D8] focus:ring-[#9AD4D8]">
                                                <label for="15-min"
                                                    class="ml-3 block text-sm font-medium leading-6 text-gray-100">
                                                    15 minutes
                                                </label>
                                            </div>
                                            <div class="flex items-center">
                                                <input id="30-min" name="expiration" type="radio"
                                                    class="h-4 w-4 bg-gray-900 border-gray-300 text-[#9AD4D8] focus:ring-[#9AD4D8]">
                                                <label for="30-min"
                                                    class="ml-3 block text-sm font-medium leading-6 text-gray-100">
                                                    30 minutes
                                                </label>
                                            </div>
                                        </div>
                                    </fieldset>
                                </div>
                            </div>

                            <div class="mt-10 flex items-center gap-x-6">
                                <button
                                    class="inline-flex items-center gap-x-2 rounded-md bg-[#9AD4D8] px-3.5 py-2.5 text-sm font-semibold text-gray-900 shadow-sm hover:bg-accent focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2">
                                    <svg class="-ml-0.5 h-5 w-5" xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 448 512" fill="currentColor">
                                        <!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
                                        <path
                                            d="M144 144v48H304V144c0-44.2-35.8-80-80-80s-80 35.8-80 80zM80 192V144C80 64.5 144.5 0 224 0s144 64.5 144 144v48h16c35.3 0 64 28.7 64 64V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V256c0-35.3 28.7-64 64-64H80z" />
                                    </svg>
                                    Encrypt
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            </div>

            <div class="mt-20 sm:mt-24 md:mx-auto md:max-w-2xl lg:mx-0 lg:mt-0 lg:w-screen">
                <div class="shadow-lg md:rounded-3xl">
                    <div class="bg-accent [clip-path:inset(0)] md:[clip-path:inset(0_round_theme(borderRadius.3xl))]">
                        <div class="mx-auto max-w-2xl lg:mx-0 p-16 lg:flex-auto">
                            <form method="dialog" action="\#" onsubmit="getData()">
                                <h1 class="mt-10 max-w-lg text-4xl font-bold tracking-tight text-gray-800 sm:text-6xl">
                                    Get Data
                                </h1>
                                <div class="sm:col-span-4 mt-6 p-1">
                                    <label for="dec-id" class="block text-sm font-medium leading-6 text-gray-800">
                                        Data ID
                                    </label>
                                    <div class="mt-2 flex">
                                        <input id="dec-id" name="id" type="text" autocomplete="off"
                                            class="flex-grow max-w-96 rounded-md px-2 border-0 bg-gray-900/5 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-900/10 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6"
                                            placeholder="ID" required>
                                    </div>
                                </div>
                                <div class="mt-10 flex items-center gap-x-6">
                                    <button
                                        class="rounded-md bg-gray-800 px-3.5 py-2.5 text-sm font-semibold text-[#9AD4D8] shadow-sm hover:bg-gray-700 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2">
                                        Get Data
                                    </button>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <div id="modal" class="relative hidden" aria-labelledby="modal-title" role="dialog" aria-modal="true">
        <div id="backdrop" class="fixed inset-0 bg-gray-800 bg-opacity-75 transition-opacity opacity-100"></div>

        <div class="fixed inset-0 z-40 w-screen overflow-y-auto">
            <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                <div id="dialog"
                    class="relative transform overflow-hidden flex flex-col flex-grow rounded-lg bg-accent px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6">
                    <div id="modal-data" class="hidden flex flex-col">
                        <div
                            class="mx-auto flex-grow flex h-12 w-12 items-center justify-center rounded-full bg-green-100">
                            <svg class="h-6 w-6 text-green-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                                stroke="currentColor" aria-hidden="true">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                            </svg>
                        </div>
                        <div class="mt-3 text-center sm:mt-5 flex-grow">
                            <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">
                                Encrypted Data
                            </h3>
                            <div class="mt-2">
                                <div class="sm:col-span-4 mt-2 p-1">
                                    <label for="enc-url" class="block text-sm font-medium leading-6 text-gray-800">
                                        Data ID
                                    </label>
                                    <div class="mt-2">
                                        <input id="enc-url" name="id" type="text" autocomplete="text"
                                            class="block w-full rounded-md px-2 border-0 bg-gray-900/5 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-900/10 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6"
                                            placeholder="ID" disabled>
                                    </div>
                                </div>
                                <div class="sm:col-span-4 mt-2 p-1">
                                    <label for="enc-key" class="block text-sm font-medium leading-6 text-gray-800">
                                        Key
                                    </label>
                                    <div class="mt-2">
                                        <input id="enc-key" name="id" type="text" autocomplete="text"
                                            class="block w-full rounded-md px-2 border-0 bg-gray-900/5 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-900/10 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6"
                                            placeholder="Key" disabled>
                                    </div>
                                </div>
                                <div class="sm:col-span-4 mt-2 p-1">
                                    <label id="enc-valid" class="block text-sm font-medium leading-6 text-gray-800">

                                    </label>
                                </div>
                                <div class="sm:col-span-4 mt-2 p-1">
                                    <label id="closing" class="block text-sm font-medium leading-6 text-gray-800">

                                    </label>
                                </div>
                            </div>
                        </div>
                        <div class="mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3">
                            <button type="button"
                                class="mt-3 inline-flex w-full justify-center rounded-md bg-gray-900 px-3 py-2 text-sm font-semibold text-accent shadow-sm ring-1 ring-inset ring-gray-900 hover:bg-gray-800 sm:col-span-2 sm:mt-0"
                                onclick="closeDialog()">
                                <svg class="mr-2 h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"
                                    fill="currentColor">
                                    <!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
                                    <path
                                        d="M208 0H332.1c12.7 0 24.9 5.1 33.9 14.1l67.9 67.9c9 9 14.1 21.2 14.1 33.9V336c0 26.5-21.5 48-48 48H208c-26.5 0-48-21.5-48-48V48c0-26.5 21.5-48 48-48zM48 128h80v64H64V448H256V416h64v48c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V176c0-26.5 21.5-48 48-48z" />
                                </svg>
                                Copy ID and Key
                            </button>
                        </div>
                    </div>
                    <div id="progress" class="mx-auto flex-grow rounded-full">
                        <svg aria-hidden="true" class="w-8 h-8 text-gray-200 animate-spin fill-gray-900"
                            viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path
                                d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                                fill="currentColor" />
                            <path
                                d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                                fill="currentFill" />
                        </svg>
                    </div>
                    <div id="modal-err" class="hidden flex flex-col">
                        <div
                            class="mx-auto flex h-12 w-12 flex-grow items-center justify-center rounded-full bg-red-100">
                            <svg class="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                                stroke="currentColor" aria-hidden="true">
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M 5 4 L 17 18 M 5 18 l 12 -14" />
                            </svg>
                        </div>
                        <div class="mt-3 text-center sm:mt-5">
                            <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-err-title">
                                Error occured
                            </h3>
                        </div>
                        <div class="mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3">
                            <button type="button"
                                class="mt-3 inline-flex w-full justify-center rounded-md bg-gray-900 px-3 py-2 text-sm font-semibold text-accent shadow-sm ring-1 ring-inset ring-gray-900 hover:bg-gray-800 sm:col-span-2 sm:mt-0"
                                onclick="closeDialog(true)">
                                Close
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>

    {% include "footer.html" %}

    <script>
        function getData() {
            var id = document.getElementById('dec-id').value;
            if (!id) {
                return;
            }
            document.getElementById('dec-id').value = "";
            window.location.href = '/' + id;
        }

        function encryptData() {
            var valid = 60;
            if (document.getElementById('15-min').checked) {
                valid = 900;
            } else if (document.getElementById('30-min').checked) {
                valid = 1800;
            }

            var data = document.getElementById('enc-data').value;
            if (!data) {
                return;
            }

            document.getElementById('enc-data').value = "";
            document.getElementById('1-min').checked = true;

            toggleDialog(true);

            fetch('/api/v1/encrypt', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    value: data,
                    validity: valid,
                })
            }).then((res) => {
                if (res.status == 200) {
                    res.json().then((data) => {
                        document.getElementById('enc-url').value = data.data_id;
                        document.getElementById('enc-key').value = data.key_id;
                        document.getElementById('enc-valid').innerHTML = 'Valid for ' + data.valid_for;
                        document.getElementById('progress').classList.add('hidden');
                        document.getElementById('modal-data').classList.remove('hidden');
                        document.getElementById('modal-err').classList.add('hidden');

                        timer();
                    });
                } else {
                    res.json().then((data) => {
                        document.getElementById('modal-err').classList.remove('hidden');
                        document.getElementById('modal-err-title').innerHTML = data.message;
                        document.getElementById('progress').classList.add('hidden');
                    });
                }
            });
        }

        function timer() {
            let timer = 9;
            let interval = setInterval(() => {
                if (document.getElementById('modal').classList.contains('hidden')) {
                    clearInterval(interval);
                    return;
                }
                document.getElementById('closing').innerHTML = "Dialog will close in " + timer + " seconds";
                timer--;
                if (timer < 0) {
                    clearInterval(interval);
                }
            }, 1000);
            setTimeout(() => {
                if (document.getElementById('modal').classList.contains('hidden')) {
                    return;
                }
                toggleDialog(false);
            }, 10000);
        }

        function closeDialog(failed = false) {
            if (failed) {
                toggleDialog(false);
                return;
            }

            var clipboard = "ID: " + document.getElementById('enc-url').value + "\nKey: " + document.getElementById('enc-key').value;
            navigator.clipboard.writeText(clipboard).then(() => toggleDialog(false), () => toggleDialog(false));
        }

        function toggleDialog(show) {
            document.getElementById('modal-data').classList.add('hidden');
            document.getElementById('enc-url').value = "";
            document.getElementById('enc-key').value = "";
            document.getElementById('enc-valid').innerHTML = "";
            document.getElementById('progress').classList.remove('hidden');
            if (show) {
                document.getElementById('modal').classList.remove('hidden');
            } else {
                document.getElementById('modal').classList.add('hidden');
            }
            document.getElementById('modal-err').classList.add('hidden');
        }
    </script>
</body>

</html>"#;

    tera.add_raw_template("index.html", source)
        .expect("Failed to add index template");
}

pub fn add_privacy(tera: &mut tera::Tera) {
    let source = r#"
<!DOCTYPE html>
<html lang="en">

{% include "head.html" %}

<body>
    <title>Claw Vault - Privacy Policies</title>
    {% include "header.html" %}

    <section class="md:mt-16 mt-16 mb-16 relative overflow-hidden">
        <div class="px-4 sm:px-6 lg:px-8">
            <div class="relative mx-auto max-w-[37.5rem] pt-20 text-center pb-24">
                <h1 class="text-4xl font-extrabold tracking-tight text-slate-900 sm:text-5xl">Privacy policy</h1>
                <p class="mt-4 text-base leading-7 text-slate-600">Last updated on January 24, 2024</p>
            </div>
        </div>
        <div class="relative px-4 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-[40rem] text-slate-600">
                <p>
                    ClawVault, created by Shashank Verma, is committed to maintaining the privacy and security of your
                    personal information. This Privacy Policy outlines our practices concerning the collection, use, and
                    disclosure of personal information when you use our service.
                </p>
                <p>
                    This page is used to inform visitors regarding my policies with the collection, use, and disclosure
                    of Personal Information if anyone decided to use my Service.
                </p>
                <br>
                <p>
                    The terms used in this Privacy Policy have the same meanings
                    as in our Terms and Conditions, which are accessible at
                    MotiClubs unless otherwise defined in this Privacy Policy.
                </p>
                <br>
                <p><strong>Information Collection and Use</strong></p>
                <br>
                <p>
                    ClawVault is designed to prioritize the security and privacy of your data. We do not collect any
                    analytical information or utilize third-party services. The information collected is solely what you
                    choose to provide to the platform. This data is securely stored in an encrypted form until the
                    specified expiration date.
                </p>
                <br>
                <p>
                    Your trust is our priority, and we are committed to maintaining the highest standards of security
                    for your data.
                </p>
                <br>
                <p><strong>Log Data</strong></p>
                <br>
                <p>
                    Whenever you use ClawVault, the app may collect Log Data, including device Internet Protocol (“IP”)
                    address, device name, operating system version, app configuration, time and date of usage, and other
                    statistics. This data is collected for error tracking and app improvement.
                </p>
                <br>
                <p><strong>Cookies</strong></p>
                <br>
                <p>
                    Cookies are files with a small amount of data that are
                    commonly used as anonymous unique identifiers. These are sent
                    to your browser from the websites that you visit and are
                    stored on your device's internal memory.
                </p>
                <br>
                <p>
                    ClawVault does not use cookies explicitly. However, third-party code and libraries used by the app
                    may employ cookies to enhance services. Users have the option to accept or refuse these cookies,
                    though refusal may limit access to certain features.
                </p>
                <br>
                <p><strong>Service Providers</strong></p>
                <br>
                <p>
                    ClawVault is hosted on the <a class="underline" href="https://railway.app" target="_blank">Railway
                        Corporation</a> platform to facilitate our service. While Railway Corporation is our hosting
                    provider, it does not involve third-party companies or individuals accessing your Personal
                    Information beyond the necessary hosting services. Please review <a class="underline"
                        href="https://railway.app/legal/privacy" target="_blank">Railway Corporation's Privacy
                        Policy</a> for more information on their practices.
                </p>
                <br>
                <p><strong>Security</strong></p>
                <br>
                <p>
                    While ClawVault strives to use commercially acceptable means to protect personal information, it
                    cannot guarantee absolute security. Users are advised to be mindful of the limitations inherent in
                    internet transmission and electronic storage.
                </p>
                <br>
                <p>
                    It's crucial to highlight that the decryption key required to access your data is not stored in our
                    database; it is provided to you. In the event the key is lost, the data becomes unrecoverable and is
                    automatically deleted upon reaching the specified expiration date. Importantly, we do not have the
                    capability to decrypt or access your data, ensuring your information remains confidential throughout
                    its storage period.
                </p>
                <br>
                <p><strong>Links to Other Sites</strong></p>
                <br>
                <p>
                    ClawVault may contain links to external sites. Users are strongly advised to review the Privacy
                    Policies of these sites, as they are not operated by Shashank Verma, and ClawVault has no control
                    over their content, privacy policies, or practices.
                </p>
                <br>
                <p><strong>Children’s Privacy</strong></p>
                <br>
                <p>
                    ClawVault does not address individuals under the age of 13 and does not knowingly collect personal
                    information from children. If such information is discovered, it will be promptly deleted from our
                    servers.
                </p>
                <br>
                <p><strong>Changes to This Privacy Policy</strong></p>
                <br>
                <p>
                    We may update our Privacy Policy from time to time. Thus, you are advised to review this page
                    periodically for any changes. We will notify you of any changes by posting the new Privacy Policy on
                    this page.
                </p>
                <br>
                <p>This policy is effective as of 2024-1-24 (January 24, 2024).</p>
                <br>
                <p><strong>Contact Us</strong></p>
                <br>
                <p>
                    If you have any questions or suggestions about our
                    Privacy Policy, do not hesitate to contact shashank.verma2002@gmail.com
                </p>
            </div>
        </div>
    </section>

    {% include "footer.html" %}

</body>

</html>"#;

    tera.add_raw_template("privacy.html", source)
        .expect("Failed to add index template");
}

pub fn add_vault(tera: &mut tera::Tera) {
    let source = r#"
<!DOCTYPE html>
<html lang="en">

{% include "head.html" %}

<body>
    {% include "header.html" %}

    <section id="claw" class="relative isolate overflow-hidden bg-gray-900">
        <svg class="absolute inset-0 -z-10 h-full w-full stroke-accent" aria-hidden="true">
            <svg x="50%" y="-1" class="overflow-visible fill-accent-30">
                <path
                    d="M-100.5 0h201v201h-201Z M699.5 0h201v201h-201Z M499.5 400h201v201h-201Z M-300.5 600h201v201h-201Z"
                    stroke-width="0" />
            </svg>
            <rect width="100%" height="100%" stroke-width="0" fill="url(#83fd4e5a-9d52-42fc-97b6-718e5d7ee527)" />
        </svg>
        <div class="mx-auto max-w-7xl pb-24 pt-4 sm:pb-32 lg:grid lg:grid-cols-2 lg:gap-x-8 lg:px-8 lg:py-40">
            <div class="mt-20 sm:mt-24 md:mx-auto md:max-w-2xl lg:mx-0 lg:mt-0 lg:w-screen">
                <div class="shadow-lg md:rounded-3xl">
                    <div class="bg-accent [clip-path:inset(0)] md:[clip-path:inset(0_round_theme(borderRadius.3xl))]">
                        <div class="mx-auto max-w-2xl lg:mx-0 p-16 lg:flex-auto">
                            <form method="dialog" action="\#" onsubmit="decryptData()">
                                <h1 class="mt-10 max-w-lg text-4xl font-bold tracking-tight text-gray-800 sm:text-6xl">
                                    Decrypt
                                </h1>
                                <div class="sm:col-span-4 mt-6 p-1">
                                    <label for="dec-key" class="block text-sm font-medium leading-6 text-gray-800">
                                        Enter the Key for claw: {{ id }}
                                    </label>
                                    <div class="mt-2 flex">
                                        <input id="dec-key" name="key" type="text" autocomplete="off"
                                            class="flex-grow rounded-md px-2 border-0 bg-gray-900/5 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-900/10 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6"
                                            placeholder="Key" required>
                                    </div>
                                </div>
                                <div class="mt-10 flex items-center gap-x-6">
                                    <button
                                        class="inline-flex items-center gap-x-2 rounded-md bg-gray-900 px-3.5 py-2.5 text-sm font-semibold text-[#9AD4D8] shadow-sm hover:bg-accent focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2"
                                        onclick="decryptData()">
                                        <svg class="-ml-0.5 h-5 w-5" xmlns="http://www.w3.org/2000/svg"
                                            viewBox="0 0 448 512" fill="currentColor">
                                            <!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
                                            <path
                                                d="M144 144c0-44.2 35.8-80 80-80c31.9 0 59.4 18.6 72.3 45.7c7.6 16 26.7 22.8 42.6 15.2s22.8-26.7 15.2-42.6C331 33.7 281.5 0 224 0C144.5 0 80 64.5 80 144v48H64c-35.3 0-64 28.7-64 64V448c0 35.3 28.7 64 64 64H384c35.3 0 64-28.7 64-64V256c0-35.3-28.7-64-64-64H144V144z" />
                                        </svg>
                                        Decrypt
                                    </button>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <div id="modal" class="relative hidden" aria-labelledby="modal-title" role="dialog" aria-modal="true">
        <div id="backdrop" class="fixed inset-0 bg-gray-800 bg-opacity-75 transition-opacity opacity-100"></div>

        <div class="fixed inset-0 z-40 w-screen overflow-y-auto">
            <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                <div id="dialog"
                    class="relative transform overflow-hidden rounded-lg flex flex-col flex-grow bg-accent px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6">
                    <div id="modal-data" class="hidden">
                        <div class="mx-auto flex h-12 w-12 items-center justify-center rounded-full bg-green-100">
                            <svg class="h-6 w-6 text-green-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                                stroke="currentColor" aria-hidden="true">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                            </svg>
                        </div>
                        <div class="mt-3 text-center sm:mt-5">
                            <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-title">
                                Decrypted Data
                            </h3>
                            <div class="mt-2">
                                <div class="mt-10 col-span-full">
                                    <label for="dec-data" class="block text-sm font-medium leading-6 text-gray-900">
                                        Data
                                    </label>
                                    <div class="mt-2 bg-gray-900 rounded-md">
                                        <textarea id="dec-data" name="data" rows="3"
                                            class="block w-full px-2 rounded-md min-h-32 border-0 bg-[#9AD4D8]/10 py-1.5 text-white shadow-sm ring-1 ring-inset ring-[#9AD4D8]/10 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6"
                                            disabled></textarea>
                                    </div>
                                </div>
                                <div class="sm:col-span-4 mt-2 p-1">
                                    <label id="closing" class="block text-sm font-medium leading-6 text-gray-800">

                                    </label>
                                </div>
                            </div>
                        </div>
                        <div class="mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3">
                            <button type="button"
                                class="mt-3 inline-flex w-full justify-center rounded-md bg-gray-900 px-3 py-2 text-sm font-semibold text-accent shadow-sm ring-1 ring-inset ring-gray-900 hover:bg-gray-800 sm:col-span-2 sm:mt-0"
                                onclick="closeDialog()">
                                <svg class="mr-2 h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"
                                    fill="currentColor">
                                    <!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
                                    <path
                                        d="M208 0H332.1c12.7 0 24.9 5.1 33.9 14.1l67.9 67.9c9 9 14.1 21.2 14.1 33.9V336c0 26.5-21.5 48-48 48H208c-26.5 0-48-21.5-48-48V48c0-26.5 21.5-48 48-48zM48 128h80v64H64V448H256V416h64v48c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V176c0-26.5 21.5-48 48-48z" />
                                </svg>
                                Copy Data
                            </button>
                        </div>
                    </div>
                    <div id="progress" class="mx-auto flex rounded-full">
                        <svg aria-hidden="true" class="w-8 h-8 text-gray-200 animate-spin fill-gray-900"
                            viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path
                                d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                                fill="currentColor" />
                            <path
                                d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                                fill="currentFill" />
                        </svg>
                    </div>
                    <div id="modal-err" class="hidden">
                        <div class="mx-auto flex h-12 w-12 items-center justify-center rounded-full bg-red-100">
                            <svg class="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                                stroke="currentColor" aria-hidden="true">
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M 5 4 L 17 18 M 5 18 l 12 -14" />
                            </svg>
                        </div>
                        <div class="mt-3 text-center sm:mt-5">
                            <h3 class="text-base font-semibold leading-6 text-gray-900" id="modal-err-title">
                                Error occured
                            </h3>
                        </div>
                        <div class="mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3">
                            <button type="button"
                                class="mt-3 inline-flex w-full justify-center rounded-md bg-gray-900 px-3 py-2 text-sm font-semibold text-accent shadow-sm ring-1 ring-inset ring-gray-900 hover:bg-gray-800 sm:col-span-2 sm:mt-0"
                                onclick="closeDialog(true)">
                                Close
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>

    {% include "footer.html" %}

    <script>
        function decryptData() {
            var id = window.location.href.substring(window.location.href.lastIndexOf('/') + 1);
            var key = document.getElementById("dec-key").value;
            if (!key) {
                return;
            }

            toggleDialog(true);

            fetch('/api/v1/decrypt', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    id: id,
                    key: key,
                })
            }).then((res) => {
                if (res.status == 200) {
                    res.json().then((data) => {
                        document.getElementById('modal-data').classList.remove('hidden');
                        document.getElementById('dec-data').value = data.data;
                        document.getElementById('progress').classList.add('hidden');
                        document.getElementById('modal-err').classList.add('hidden');

                        timer();
                    });
                } else {
                    res.json().then((data) => {
                        document.getElementById('modal-err').classList.remove('hidden');
                        document.getElementById('modal-err-title').innerHTML = data.message;
                        document.getElementById('progress').classList.add('hidden');
                    });
                }
            });
        }

        function timer() {
            let timer = 9;
            let interval = setInterval(() => {
                if (document.getElementById('modal').classList.contains('hidden')) {
                    clearInterval(interval);
                    return;
                }
                document.getElementById('closing').innerHTML = "Dialog will close in " + timer + " seconds";
                timer--;
                if (timer < 0) {
                    clearInterval(interval);
                }
            }, 1000);
            setTimeout(() => {
                if (document.getElementById('modal').classList.contains('hidden')) {
                    return;
                }
                toggleDialog(false);
                window.location.href = '/';
            }, 10000);
        }

        function closeDialog(failed = false) {
            if (failed) {
                toggleDialog(false);
                return;
            }

            navigator.clipboard.writeText(document.getElementById('dec-data').value).then(() => {
                toggleDialog(false);
                window.location.href = '/';
            }, () => {
                toggleDialog(false);
                window.location.href = '/';
            });
        }

        function toggleDialog(show) {
            document.getElementById('modal-data').classList.add('hidden');
            document.getElementById('dec-data').value = "";
            document.getElementById('progress').classList.remove('hidden');
            document.getElementById('modal-err').classList.add('hidden');
            if (show) {
                document.getElementById('modal').classList.remove('hidden');
            } else {
                document.getElementById('modal').classList.add('hidden');
            }
        }
    </script>
</body>

</html>"#;

    tera.add_raw_template("vault.html", source)
        .expect("Failed to add vault template");
}
