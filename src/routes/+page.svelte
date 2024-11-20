<script>
    import { Global, globals } from "../lib/global.svelte"
    import { Servo } from "../lib/servo.svelte"

    let servo = new Servo();

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

        <div class="m-3 flex flex-col gap-4 justify-center items-center">
          <input
            type="text"
            bind:value={globals.url}
            class="w-full px-2 py-3 border-[1px] border-slate-950"
          >
          {@render btn("Set Url", () => Global.setUrl())}
        </div>

        <pre
          class="h-40 border-[1px] border-slate-950 overflow-auto"
        >{
            servo.content.trim()
        }</pre>

        <div class="m-3 flex flex-row justify-center items-center">
          {@render btn("Ambil Data")}
        </div>

        <div class="m-3 flex flex-row gap-4 justify-center items-center">
          {@render btn("Geterin", () => servo.geterin())}
        </div>

        <div class="m-3 mt-12 flex flex-row gap-4 justify-center items-center">
          <input
            type="number"
            bind:value={servo.servo_up}
            class="px-2 py-3 border-[1px] border-slate-950"
          >
          <input
            type="number"
            bind:value={servo.servo_down}
            class="px-2 py-3 border-[1px] border-slate-950"
          >
        </div>

        <div class="m-3 flex flex-row gap-4 justify-center items-center">
          {@render btn("Servo Up", () => servo.up())}
          {@render btn("Servo Down", () => servo.down())}
        </div>

        <div class="m-3 flex flex-row gap-4 justify-center items-center">
          {@render btn("Servo Reset", () => servo.reset())}
        </div>

    </div>
    <div></div>
</section>

{#snippet btn(title, cmd)}
  <button
      onclick={cmd}
      class="bg-emerald-600 hover:bg-emerald-400 cli rounded-md size-full border border-black"
    >
      {title}
    </button
  >
{/snippet}

