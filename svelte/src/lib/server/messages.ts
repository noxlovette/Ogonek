type MessageMap = {
  teacherNotify: (params: { username: string }) => string;
  completed: (params: { task: string; username: string }) => string;
  reminder: (params: { task: string; dueDate: string }) => string;
  deckCreated: (params: { title: string; id: string }) => string;
  taskCreated: (params: { title: string; id: string; date: string }) => string;
};

export const messages: MessageMap = {
  teacherNotify: ({ username }) => `${username} needs homework`,
  completed: ({ task, username }) =>
    `${task} for ${username} has been completed`,
  reminder: ({ task, dueDate }) =>
    `Don't forget to complete "${task}" by ${dueDate}`,
  deckCreated: ({ title, id }) =>
    escapeMarkdownV2(
      `A new deck has been added: "${title}". View it on [Ogonek](https://ogonek.app/s/words/w/${id}).`,
    ),
  taskCreated: ({ title, id, date }) =>
    escapeMarkdownV2(
      `A new task *has* been added: "${title}". Due Date: ${date}. View it on [Ogonek](https://ogonek.app/s/tasks/t/${id}).`,
    ),
};

/**
 * Escapes only unsafe characters for Telegram MarkdownV2
 * Leaves formatting characters (*, _, [, ], (, )) intact
 * Escapes: ~ ` > # + = | { } . ! - \
 */
export function escapeMarkdownV2(text: string): string {
  return text.replace(/[~`>#+=|{}.!\\-]/g, (char) => `\\${char}`);
}
