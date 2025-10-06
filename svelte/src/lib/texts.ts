export const notificationMessages = {
  // Auth messages
  auth: {
    loginSuccess: "С возвращением",
    logoutSuccess: "До новых встреч",
    signupSuccess: "Добро пожаловать на борт",
    passwordChanged: "Пароль обновлён",
    invalidCredentials: "Неверное имя пользователя или пароль",
    signupFailure: "Регистрация не удалась",
    sessionExpired: "Сеанс истёк, пожалуйста, войдите снова",
    accountLocked: "Аккаунт временно заблокирован",
  },

  table: {
    selectAll: "Выбрать все",
    empty: "Тут ничего",
  },

  // CRUD operations
  crud: {
    created: "Создано",
    updated: "Изменения сохранены",
    deleted: "Удалено",
    restored: "Успешно восстановлено",
    duplicated: "Успешно дублировано",
    archived: "Успешно архивировано",
    published: "Успешно опубликовано",
    unpublished: "Отменена публикация",

    save: "Сохранить",
    delete: "Удалить",
    complete: "Отметить как выполненное",
    uncomplete: "Отметить как невыполненное",
    edit: "Изменить",
    create: "Создать",

    editing: "Редактируем",
    cancel: "Передумал",
  },

  // Task-specific messages
  tasks: {
    completed: "Отмечено как выполненное",
    notCompleted: "Отмечено как невыполненное",
    assigned: "Задача назначена",
    unassigned: "Задача не назначена",
    overdue: "Задача просрочена",
    dueSoon: "Задача скоро истекает",
    priorityChanged: "Приоритет задачи обновлён",

    title: "Задания",
  },

  // File operations
  files: {
    uploaded: "Файл успешно загружен",
    deleted: "Файл удалён",
    downloadStarted: "Загрузка начата",
    uploadFailed: "Загрузка не удалась",
    fileTooLarge: "Размер файла превышает лимит",
    invalidFormat: "Недопустимый формат файла",
    processingFile: "Обработка файла...",
  },

  flashcards: {
    // Flashcard/Deck operations
    subscribed: "Вы подписаны",
    unsubscribed: "Вы отписаны",

    // Flashcards static
    title: "Карточки",
    study: "Режим учебы",
  },

  // Student/Teacher operations
  students: {
    added: "Студент добавлен",
    removed: "Студент удалён",
    invited: "Студент приглашён",
    inviteResent: "Приглашение отправлено повторно",
    profileUpdated: "Профиль студента обновлён",
    assignmentCreated: "Задание создано",
  },

  lessons: {
    scheduled: "Урок запланирован",
    rescheduled: "Урок перенесён",
    cancelled: "Урок отменён",
    started: "Урок начат",
    ended: "Урок завершён",
    title: "Занятия",
  },

  // Settings & Preferences
  settings: {
    saved: "Настройки сохранены",
    restored: "Настройки восстановлены по умолчанию",
    preferencesUpdated: "Предпочтения обновлены",
    themeChanged: "Тема изменена",
    languageChanged: "Язык изменён",
    notificationsEnabled: "Уведомления включены",
    notificationsDisabled: "Уведомления отключены",
  },

  // Notifications
  notifications: {
    deviceRegistered: "Устройство зарегистрировано для уведомлений",
    deviceRemoved: "Устройство удалено из уведомлений",
    notificationSent: "Уведомление отправлено",
    homeworkRequested: "Запрос домашнего задания отправлен",
    reminderSet: "Напоминание установлено",
  },

  // Error messages
  errors: {
    generic: "Что-то пошло не так",
    networkError: "Ошибка сетевого подключения",
    serverError: "Произошла ошибка сервера",
    validationFailed: "Пожалуйста, проверьте ввод",
    permissionDenied: "Доступ запрещён",
    resourceNotFound: "Ресурс не найден",
    operationFailed: "Операция не удалась",
    saveError: "Не удалось сохранить изменения",
    loadError: "Не удалось загрузить данные",
    connectionLost: "Соединение потеряно",
    timeoutError: "Время запроса истекло",
  },

  // Success confirmations
  confirmations: {
    dataSaved: "Данные успешно сохранены",
    emailSent: "Письмо отправлено",
    inviteSent: "Приглашение отправлено",
    backupCreated: "Резервная копия создана",
    importComplete: "Импорт завершён",
    exportComplete: "Экспорт завершён",
    syncComplete: "Синхронизация завершена",
  },

  // Warning messages
  warnings: {
    unsavedChanges: "У вас есть несохранённые изменения",
    actionCannotBeUndone: "Это действие нельзя отменить",
    dataWillBeLost: "Данные будут потеряны",
    largeFileSelected: "Выбран большой файл, загрузка может занять время",
    lowStorage: "Заканчивается место для хранения",
    maintenanceMode: "Ведутся технические работы",
  },

  // Info messages
  info: {
    loading: "Загрузка...",
    processing: "Обработка...",
    saving: "Сохранение...",
    uploading: "Загрузка...",
    downloading: "Скачивание...",
    syncing: "Синхронизация...",
    searching: "Поиск...",
    connecting: "Подключение...",
    preparingData: "Подготовка данных...",
    almostDone: "Почти готово...",
  },

  marketing: {
    lessons: "",
    tasks: "",
    flashcards: "",
    hero: "",
  },
} as const;

// Type-safe message accessor
type NotificationPath =
  | `auth.${keyof typeof notificationMessages.auth}`
  | `crud.${keyof typeof notificationMessages.crud}`
  | `tasks.${keyof typeof notificationMessages.tasks}`
  | `files.${keyof typeof notificationMessages.files}`
  | `flashcards.${keyof typeof notificationMessages.flashcards}`
  | `students.${keyof typeof notificationMessages.students}`
  | `lessons.${keyof typeof notificationMessages.lessons}`
  | `settings.${keyof typeof notificationMessages.settings}`
  | `notifications.${keyof typeof notificationMessages.notifications}`
  | `errors.${keyof typeof notificationMessages.errors}`
  | `confirmations.${keyof typeof notificationMessages.confirmations}`
  | `warnings.${keyof typeof notificationMessages.warnings}`
  | `info.${keyof typeof notificationMessages.info}`
  | `marketing.${keyof typeof notificationMessages.marketing}`
  | `table.${keyof typeof notificationMessages.table}`;

/**
 * Type-safe notification message getter
 * Usage: message.mnotificationText('auth.welcomeHome')
 */
export function getNotificationText(path: NotificationPath): string {
  const keys = path.split(".") as [keyof typeof notificationMessages, string];
  const category = notificationMessages[keys[0]] as Record<string, string>;
  return category[keys[1]] || "Message not found";
}

export const texts = {
  notificationText: getNotificationText,

  auth: notificationMessages.auth,
  crud: notificationMessages.crud,
  tasks: notificationMessages.tasks,
  files: notificationMessages.files,
  flashcards: notificationMessages.flashcards,
  students: notificationMessages.students,
  lessons: notificationMessages.lessons,
  settings: notificationMessages.settings,
  notifications: notificationMessages.notifications,
  errors: notificationMessages.errors,
  confirmations: notificationMessages.confirmations,
  warnings: notificationMessages.warnings,
  info: notificationMessages.info,
  table: notificationMessages.table,
  marketing: notificationMessages.marketing,
};

export default texts;
