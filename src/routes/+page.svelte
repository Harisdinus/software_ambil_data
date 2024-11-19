<script>
    import { fetch } from "@tauri-apps/plugin-http";

    // Send a GET request

    let isi_json = $state("");

    async function getResponse() {
        const response = await fetch("http://192.168.1.69/perintah", {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ perintah: "ambil-data" }),
        });

        

        console.log(response.status); // e.g. 200
        console.log(response.statusText); // e.g. "OK"

        const content = await response.text();

        console.log(content); // e.g. "OK"

        isi_json += content + "<br>" 

    }
</script>

<section class="h-screen flex flex-col justify-between items-center">
    <div class="p-3 flex flex-row w-screen">
        <div class="flex items-center">
            <div
                class="border-[2px] border-slate-950 rounded-full items-center justify-center size-12 flex"
            >
                logo
            </div>
            <h1 class="ms-2 font-bold">Smart Grind Precision</h1>
        </div>
    </div>
    <div class="size-fit flex flex-col">
        <div class="h-40 w-80  border-[1px] border-slate-950 overflow-auto">
            {@html isi_json}
        </div>
        <div class="m-3 flex flex-row justify-center items-center">
            <button
                onclick={getResponse}
                class="bg-emerald-600 hover:bg-emerald-400 cli rounded-md size-full border border-black"
                >Ambil Data</button
            >
        </div>
    </div>
    <div></div>
</section>
