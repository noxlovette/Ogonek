export function getGreeting() {
  const date = new Date();
  const hours = date.getHours();

  if (hours >= 5 && hours < 12) {
    return "morning";
  } else if (hours >= 12 && hours < 18) {
    return "afternoon";
  } else if (hours >= 18 && hours < 22) {
    return "evening";
  } else {
    return "night";
  }
}

import { thumbs } from "@dicebear/collection";
import { createAvatar } from "@dicebear/core";

export function getAvatar(seed: string) {
  const avatar = createAvatar(thumbs, {
    seed,
  });

  return avatar.toDataUri();
}
