import type { ClientInit } from "@sveltejs/kit";
import { setUser } from '$lib/stores';

export const init: ClientInit = async () => {
    let user = localStorage.getItem("user");
    console.log(user);
    if (user) {
        setUser(JSON.parse(user));
    }
};