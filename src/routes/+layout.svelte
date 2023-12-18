<script>
	import '../app.pcss';
	import NavBar from '../component/NavBar.svelte';
	import './styles.css';
</script>

<svelte:head>
	<script src="https://kit.fontawesome.com/f37da43382.js" crossorigin="anonymous"></script>
</svelte:head>

<div class="app flex flex-col h-screen max-h-screen overflow-hidden lg:flex-row-reverse">
	<main class="flex-grow overflow-y-auto bg-[#242434] p-10">
		<slot />
	</main>
	<div class="sticky bottom-0 lg:left-0">
		<NavBar />
	</div>
</div>

<style>
</style>
