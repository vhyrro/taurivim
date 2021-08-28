<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	// Register the Neovim UI
	const print_window_size = () => {
		invoke("register_ui");
	}
</script>

<main>
	<h1 on:click={print_window_size}>Hello!</h1>
	<p>This is a test just to make sure everything works</p>
	<p>Click the Hello heading to print the current window dimensions from Rust!</p>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
