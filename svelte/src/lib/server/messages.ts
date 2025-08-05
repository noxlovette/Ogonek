type MessageMap = {
  teacherNotify: (params: { username: string }) => string;
  completed: (params: { task: string; username: string; id: string }) => string;
  reminder: (params: { task: string; dueDate: string }) => string;
  deckCreated: (params: { title: string; id: string }) => string;
  taskCreated: (params: { title: string; id: string; date: string }) => string;
};

export const messages: MessageMap = {
  teacherNotify: ({ username }) =>
    `${escapeMarkdownV2(username)} needs homework\\. Add more on [Ogonek](https://ogonek\\.app/t/tasks)`,

  completed: ({ task, username, id }) =>
    `${escapeMarkdownV2(task)} for ${escapeMarkdownV2(username)} has been completed\\. View the result on [Ogonek](https://ogonek\\.app/t/tasks/${id})`,

  reminder: ({ task, dueDate }) =>
    `Don't forget to complete "${escapeMarkdownV2(task)}" by ${escapeMarkdownV2(dueDate)}`,

  deckCreated: ({ title, id }) =>
    `A new deck has been added: "${escapeMarkdownV2(title)}"\\. View it on [Ogonek](https://ogonek\\.app/s/flashcards/${id})\\.`,

  taskCreated: ({ title, id, date }) =>
    `A new task has been added: "${escapeMarkdownV2(title)}"\\. Due Date: ${escapeMarkdownV2(date)}\\. View it on [Ogonek](https://ogonek\\.app/s/tasks/${id})\\.`,
};

/**
 * Escapes user content for Telegram MarkdownV2
 * ONLY escapes characters that can break parsing in user input
 * Does NOT escape markdown formatting characters like [], (), etc.
 */
export function escapeMarkdownV2(text: string): string {
  // Only escape these specific chars that break MarkdownV2 in user content
  return text.replace(/[_*[\]~`>#+=|{}.!\\]/g, "\\$&");
}
