<script>
	import { enhance } from '$app/forms';
</script>

<h1>Sign Up</h1>
<form use:enhance method="post">
	<div>
		<label for="name">Name:</label>
		<input type="text" name="name" required />
	</div>
	<div>
		<label for="username">Username:</label>
		<input type="text" name="username" required />
	</div>
	<div>
		<label for="role">Role:</label>
		<input type="text" name="role" required />
	</div>
	<div>
		<label for="email">Email:</label>
		<input type="email" name="email" required />
	</div>
	<div>
		<label for="password">Password:</label>
		<input type="password" name="password" required />
	</div>
	<button type="submit">Sign Up</button>
</form>
