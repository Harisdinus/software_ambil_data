<script>
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
        <div class="h-40 w-80  border-[1px] border-slate-950 overflow-auto">
            {@html servo.content}
        </div>

        <div class="m-3 flex flex-row justify-center items-center">
          {@render btn("Servo Up", () => servo.up())}
          {@render btn("Servo Down", () => servo.down())}
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

