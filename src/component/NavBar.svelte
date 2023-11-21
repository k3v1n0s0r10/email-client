<script lang="ts">
	let selected = $state('/')
	const routes = {
		'/': {
			name: 'Inbox',
			icon: 'fa-solid fa-inbox'
		},
		'/send': {
			name: 'Send',
			icon: 'fa-solid fa-paper-plane '
		},
		'/trash': {
			name: 'Trash',
			icon: 'fa-regular fa-trash-can'
		},
		'/profile': {
			name: 'Profile',
			icon: 'fa-solid fa-user'
		}
	}
</script>

<nav class="bg-[#1C1B1B] py-3 h-full lg:p-4">
	<ul class="flex justify-evenly lg:flex-col lg:space-y-5">
		{#each Object.entries(routes) as [route, value]}
			<li class={`${selected === route ? 'selected' : undefined}`}>
				<a href={route} onclick={() => selected = route}> <i class={value.icon} /></a>
			</li>
		{/each}
	</ul>
</nav>

<style scoped>
li{
	@apply px-3 py-1 rounded-lg
}
.selected{
	@apply bg-[#12CCE1]
}
</style>
