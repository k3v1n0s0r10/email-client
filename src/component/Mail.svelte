<script lang="ts">
  import senderImg from '$lib/images/sender.png'
</script>

<div
	class="flex justify-between my-4 px-4 py-2 rounded-lg h-12 items-center text-white border-2 border-[#00000030] shadow-lg"
>
	<img src={senderImg} alt="" class="h-full rounded-full" />
	<p>Alerta de seguridad</p>
	<p>algo paso bien creizi omaiga</p>
	<p>{new Date().toDateString()}</p>
</div>
