<script lang="ts">
	import wasm, { init, WasmInterface } from 'lambda_calculator';
	import { onMount } from 'svelte';
    let wasm_interface: WasmInterface;
	onMount(async () => {
		await wasm();
        wasm_interface = init();
	});
	let expr_input: string = '';
	let expr_output: string | undefined = '';
	$: on_expr_change(expr_input);
    function on_expr_change(expr_input:string) {
        if (wasm_interface) {
            wasm_interface.set_current_expr_string(expr_input);
            expr_output = wasm_interface.get_current_expr_string();
            console.log(expr_output);
        }
    }
    const step = () => {
        let n_string = wasm_interface.step();
        expr_output = n_string;
    }
</script>

<textarea bind:value={expr_input}></textarea>

<button>interpret</button>
<p>Parsed Expression:</p>
<button on:click={(e)=>step()}>step</button>

<p>{expr_output}</p> 

<style lang="scss">

</style>
